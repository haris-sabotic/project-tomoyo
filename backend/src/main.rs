use csv::{Reader, Writer};
use logic::ClassSlots;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Instant,
    vec,
};
use ws::{Builder, Handler, Handshake, Message, Sender, Settings};

use project_tomoyo::*;

pub mod cost;
pub mod regular_tabs;
use regular_tabs::*;

use crate::logic::{Timetable, TimetableData};

pub mod logic;

pub mod util;
use util::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RoomRecord {
    name: String,
    kinds: String,
}

const MAX_PERIODS_PER_DAY: u32 = 7;

struct Server {
    out: Sender,
    timetable: Arc<Mutex<Timetable>>,
    running_algorithm: Arc<AtomicBool>,
    time: Instant,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        println!("Server got message '{}'. ", msg);

        let parsed_msg: Value = match &msg {
            Message::Text(text) => serde_json::from_str(&text).expect("Invalid message."),
            Message::Binary(_) => panic!("Binary message"),
        };

        if parsed_msg["kind"].as_str() == Some("pause") {
            self.running_algorithm.store(false, Ordering::Relaxed);
        }

        let timetable_ref = self.timetable.clone(); // cloned reference to timetable
        let mut timetable = timetable_ref.lock().unwrap(); // cloned reference to timetable

        match parsed_msg["kind"].as_str() {
            Some("import") => handle_import(&mut timetable, &parsed_msg, &self.out),

            Some("export") => handle_export(&mut timetable, &parsed_msg),

            Some("list") => {
                self.out.send(msg).unwrap();

                let tab = parsed_msg["tab"].as_str().unwrap();

                match tab {
                    "classes" => {
                        let data = parsed_msg["data"].as_array().unwrap();

                        timetable.data.classes.clear();
                        for v in data.iter() {
                            timetable.data.classes.push(Class {
                                name: String::from(v.as_str().unwrap()),
                            })
                        }
                    }
                    "rooms" => {
                        let data = parsed_msg["data"].as_array().unwrap();

                        timetable.data.rooms.clear();
                        for v in data.iter() {
                            let kinds: String = String::from(v["kinds"].as_str().unwrap());

                            timetable.data.rooms.push(Room {
                                name: String::from(v["name"].as_str().unwrap()),
                                kinds: kinds.split(' ').map(str::to_string).collect(),
                            })
                        }
                    }
                    "subjects" => {
                        let data = parsed_msg["data"].as_array().unwrap();

                        timetable.data.subjects.clear();
                        for v in data.iter() {
                            timetable.data.subjects.push(Subject {
                                name: String::from(v["name"].as_str().unwrap()),
                                kind: String::from(v["kind"].as_str().unwrap()),
                            })
                        }
                    }
                    "teachers" => {
                        let data = parsed_msg["data"].as_array().unwrap();

                        timetable.data.teachers.clear();
                        for v in data.iter() {
                            timetable.data.teachers.push(Teacher {
                                name: String::from(v.as_str().unwrap()),
                            })
                        }
                    }
                    "relations" => {
                        let data = parsed_msg["data"].as_array().unwrap();

                        timetable.data.relations.clear();
                        for v in data.iter() {
                            // annoying process to convert it to an Option<u32>
                            let mut second: Option<u32> = None;
                            match v["perWeekSecond"].as_u64() {
                                Some(n) => {
                                    second = Some(n as u32);
                                }
                                None => {}
                            }

                            timetable.data.relations.push(Relation {
                                teacher: v["teacher"].as_u64().unwrap() as usize,
                                subject: v["subject"].as_u64().unwrap() as usize,
                                class: v["class_"].as_u64().unwrap() as usize,
                                per_week_first: v["perWeekFirst"].as_u64().unwrap() as u32,
                                per_week_second: second,
                            })
                        }
                    }
                    _ => {}
                }
            }

            Some("initial_timetable") => {
                timetable.generate_random_table(&self.out);
                send_timetable(&timetable, &self.out);
            }

            Some("timetable") => {
                send_timetable(&timetable, &self.out);
            }

            Some("play") => {
                self.time = Instant::now();

                let table = parsed_msg["data"]["table"].as_array().unwrap();
                for i in 0..timetable.table.len() {
                    timetable.table[i] = serde_json::from_value(table[i].clone()).unwrap();
                }

                self.running_algorithm
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                let timetable_local_ref = self.timetable.clone(); // cloned reference to timetable
                let running_algorithm_local_ref = self.running_algorithm.clone(); // cloned reference to timetable
                let out_local_ref = self.out.clone(); // cloned reference to out channel

                let alpha = parsed_msg["data"]["alpha"].as_f64().unwrap();
                let t0 = parsed_msg["data"]["t0"].as_f64().unwrap();
                let sa_max = parsed_msg["data"]["sa_max"].as_i64().unwrap();

                thread::spawn(move || {
                    timetable_local_ref.lock().unwrap().start_algorithm(
                        running_algorithm_local_ref,
                        &out_local_ref,
                        alpha,
                        t0,
                        sa_max,
                    );
                });
            }

            Some("pause") => {
                println!("Duration: {:?}", self.time.elapsed());

                send_timetable(&timetable, &self.out);
            }

            Some("fill_rooms") => {
                timetable.fill_rooms();

                send_timetable(&timetable, &self.out);
            }

            Some("detailed_cost") => {
                let data = parsed_msg["data"].as_array().unwrap();
                for i in 0..timetable.table.len() {
                    timetable.table[i] = serde_json::from_value(data[i].clone()).unwrap();
                }

                timetable.detailed_cost();
            }

            _ => panic!("Invalid message."),
        }

        Ok(())
    }
}

fn handle_import(timetable: &mut Timetable, parsed_msg: &Value, out: &Sender) {
    let tab = parsed_msg["tab"].as_str().unwrap();

    match tab {
        "timetable" => {
            let contents = fs::read_to_string("./import/timetable.json").unwrap();
            let table: Vec<ClassSlots> = serde_json::from_str(&contents).unwrap();

            timetable.table = table;
            send_timetable(timetable, out);
        }
        _ => {
            let mut rdr = Reader::from_path(format!("./import/{}.csv", tab.clone())).unwrap();

            match tab {
                "classes" => {
                    for result in rdr.deserialize() {
                        let record: Class = result.unwrap();
                        timetable.data.classes.push(record);
                    }
                    send_classes(&out, &timetable.data.classes);
                }
                "rooms" => {
                    for result in rdr.deserialize() {
                        let record: RoomRecord = result.unwrap();

                        let room: Room = Room {
                            name: record.name,
                            kinds: record.kinds.split(' ').map(str::to_string).collect(),
                        };
                        timetable.data.rooms.push(room);
                    }
                    send_rooms(&out, &timetable.data.rooms);
                }
                "subjects" => {
                    for result in rdr.deserialize() {
                        let record: Subject = result.unwrap();
                        timetable.data.subjects.push(record);
                    }
                    send_subjects(&out, &timetable.data.subjects);
                }
                "teachers" => {
                    for result in rdr.deserialize() {
                        let record: Teacher = result.unwrap();
                        timetable.data.teachers.push(record);
                    }
                    send_teachers(&out, &timetable.data.teachers);
                }
                "relations" => {
                    for result in rdr.deserialize() {
                        let record: Relation = result.unwrap();
                        timetable.data.relations.push(record);
                    }
                    send_relations(&out, &timetable.data.relations);
                }

                _ => {}
            }
        }
    }
}

fn handle_export(timetable: &mut Timetable, parsed_msg: &Value) {
    let tab = parsed_msg["tab"].as_str().unwrap();

    match tab {
        "timetable" => {
            let json = json!(timetable.table);
            fs::write("./export/timetable.json", json.to_string()).unwrap();
        }
        _ => {
            let mut wrtr = Writer::from_path(format!("./export/{}.csv", tab.clone())).unwrap();

            match tab {
                "classes" => {
                    update_classes(&mut timetable.data.classes, &parsed_msg["data"]);
                    for row in timetable.data.classes.iter() {
                        wrtr.serialize(row).unwrap();
                    }
                }
                "rooms" => {
                    update_rooms(&mut timetable.data.rooms, &parsed_msg["data"]);
                    for row in timetable.data.rooms.iter() {
                        let room_record = RoomRecord {
                            name: row.name.clone(),
                            kinds: row.kinds.join(" "),
                        };
                        wrtr.serialize(room_record).unwrap();
                    }
                }
                "subjects" => {
                    update_subjects(&mut timetable.data.subjects, &parsed_msg["data"]);
                    for row in timetable.data.subjects.iter() {
                        wrtr.serialize(row).unwrap();
                    }
                }
                "teachers" => {
                    update_teachers(&mut timetable.data.teachers, &parsed_msg["data"]);
                    for row in timetable.data.teachers.iter() {
                        wrtr.serialize(row).unwrap();
                    }
                }
                "relations" => {
                    update_relations(&mut timetable.data.relations, &parsed_msg["data"]);
                    for row in timetable.data.relations.iter() {
                        wrtr.serialize(row).unwrap();
                    }
                }

                _ => {}
            }

            wrtr.flush().unwrap();
        }
    }
}

fn send_timetable(timetable: &Timetable, out: &Sender) {
    let json = json!({
        "kind": "timetable",
        "tab": "timetable",
        "data": {
            "max_periods_per_day": timetable.max_periods_per_day,
            "table": timetable.table
        }
    });

    ws_send(&out, &json);
}

fn main() {
    env_logger::init();

    let ws = Builder::new()
        .with_settings(Settings {
            max_connections: 1,
            ..Settings::default()
        })
        .build(|out| Server {
            out,
            timetable: Arc::new(Mutex::new(Timetable::new(
                TimetableData {
                    classes: vec![],
                    rooms: vec![],
                    subjects: vec![],
                    teachers: vec![],
                    relations: vec![],
                },
                MAX_PERIODS_PER_DAY,
                vec![],
            ))),
            running_algorithm: Arc::new(AtomicBool::new(false)),
            time: Instant::now(),
        })
        .unwrap();

    ws.listen("127.0.0.1:3012").unwrap();
}
