use project_tomoyo::*;
use serde_json::{json, Value};
use ws::Sender;

use crate::util::*;

pub fn send_classes(sender: &Sender, classes: &Vec<Class>) {
    let mut data: Vec<&str> = vec![];
    data.reserve(classes.len());

    for class in classes.iter() {
        data.push(class.name.as_str());
    }

    let json = json!({
        "kind": "list",
        "tab": "classes",
        "data": data
    });

    ws_send(&sender, &json);
}
pub fn send_rooms(sender: &Sender, rooms: &Vec<Room>) {
    let mut data: Vec<Value> = vec![];
    data.reserve(rooms.len());

    for room in rooms.iter() {
        data.push(json!({"name": room.name.as_str(), "kinds": room.kinds.join(" ")}));
    }

    let json = json!({
        "kind": "list",
        "tab": "rooms",
        "data": data
    });

    ws_send(&sender, &json);
}
pub fn send_subjects(sender: &Sender, subjects: &Vec<Subject>) {
    let mut data: Vec<Value> = vec![];
    data.reserve(subjects.len());

    for subject in subjects.iter() {
        data.push(json!({"name": subject.name.as_str(), "kind": subject.kind.as_str()}));
    }

    let json = json!({
        "kind": "list",
        "tab": "subjects",
        "data": data
    });

    ws_send(&sender, &json);
}
pub fn send_teachers(sender: &Sender, teachers: &Vec<Teacher>) {
    let mut data: Vec<&str> = vec![];
    data.reserve(teachers.len());

    for teacher in teachers.iter() {
        data.push(teacher.name.as_str());
    }

    let json = json!({
        "kind": "list",
        "tab": "teachers",
        "data": data
    });

    ws_send(&sender, &json);
}
pub fn send_relations(sender: &Sender, relations: &Vec<Relation>) {
    let mut data: Vec<Value> = vec![];
    data.reserve(relations.len());

    for relation in relations.iter() {
        data.push(json!({"shift": relation.shift, "teacher": relation.teacher, "subject": relation.subject, "class_": relation.class, "perWeekFirst": relation.per_week_first, "perWeekSecond": relation.per_week_second}));
    }

    let json = json!({
        "kind": "list",
        "tab": "relations",
        "data": data
    });

    ws_send(&sender, &json);
}

pub fn update_classes(classes: &mut Vec<Class>, data: &Value) {
    match data.as_array() {
        Some(arr) => {
            classes.clear();
            classes.reserve(arr.len());

            for el in arr {
                classes.push(Class {
                    name: String::from(el.as_str().unwrap()),
                })
            }
        }
        None => panic!("Invalid message"),
    }
}
pub fn update_rooms(rooms: &mut Vec<Room>, data: &Value) {
    match data.as_array() {
        Some(arr) => {
            rooms.clear();
            rooms.reserve(arr.len());

            for el in arr {
                let kinds: String = String::from(el["kinds"].as_str().unwrap());

                rooms.push(Room {
                    name: String::from(el["name"].as_str().unwrap()),
                    kinds: kinds.split(' ').map(str::to_string).collect(),
                })
            }
        }
        None => panic!("Invalid message"),
    }
}
pub fn update_subjects(subjects: &mut Vec<Subject>, data: &Value) {
    match data.as_array() {
        Some(arr) => {
            subjects.clear();
            subjects.reserve(arr.len());

            for el in arr {
                subjects.push(Subject {
                    name: String::from(el["name"].as_str().unwrap()),
                    kind: String::from(el["kind"].as_str().unwrap()),
                })
            }
        }
        None => panic!("Invalid message"),
    }
}
pub fn update_teachers(teachers: &mut Vec<Teacher>, data: &Value) {
    match data.as_array() {
        Some(arr) => {
            teachers.clear();
            teachers.reserve(arr.len());

            for el in arr {
                teachers.push(Teacher {
                    name: String::from(el.as_str().unwrap()),
                })
            }
        }
        None => panic!("Invalid message"),
    }
}
pub fn update_relations(relations: &mut Vec<Relation>, data: &Value) {
    match data.as_array() {
        Some(arr) => {
            relations.clear();
            relations.reserve(arr.len());

            for el in arr {
                // annoying process to convert it to an Option<u32>
                let mut second: Option<u32> = None;
                match el["perWeekSecond"].as_u64() {
                    Some(n) => {
                        second = Some(n as u32);
                    }
                    None => {}
                }

                relations.push(Relation {
                    shift: el["shift"].as_i64().unwrap() as i32,
                    teacher: el["teacher"].as_i64().unwrap() as usize,
                    subject: el["subject"].as_i64().unwrap() as usize,
                    class: el["class_"].as_i64().unwrap() as usize,
                    per_week_first: el["perWeekFirst"].as_i64().unwrap() as u32,
                    per_week_second: second,
                })
            }
        }
        None => panic!("Invalid message"),
    }
}
