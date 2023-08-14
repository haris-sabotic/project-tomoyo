use std::{collections::HashMap, println, vec};

use serde_json::Value;
use ws::Sender;

use serde::{Deserialize, Serialize};

use crate::logic::{ClassSlots, Timetable, TimetableData};

pub fn ws_send(sender: &Sender, json: &Value) {
    sender.send(json.to_string()).unwrap();

    // println!("===== MESSAGE SENT BY SERVER START =====");
    // println!("{}", serde_json::to_string_pretty(json).unwrap());
    // println!("=====  MESSAGE SENT BY SERVER END  =====");
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TeacherSlot {
    Empty,
    PartiallyFilled { class: usize, subject: usize },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TeacherSlots {
    pub slots: Vec<TeacherSlot>,
}

pub fn class_table_to_teacher_table(
    class_table: &Vec<ClassSlots>,
    table_data: &TimetableData,
    max_periods_per_day: u32,
) -> Vec<TeacherSlots> {
    let mut table: Vec<TeacherSlots> = vec![];
    table.resize(
        table_data.teachers.len(),
        TeacherSlots {
            slots: {
                let mut slots: Vec<TeacherSlot> = vec![];
                slots.resize(5 * max_periods_per_day as usize, TeacherSlot::Empty);

                slots
            },
        },
    );

    for class_slots in class_table.iter() {
        let mut i = 0;
        for slot in class_slots.slots.iter() {
            match slot {
                crate::logic::Slot::Single(s) => match s {
                    crate::logic::SlotData::Empty => {}
                    crate::logic::SlotData::PartiallyFilled {
                        teacher, subject, ..
                    } => {
                        table[*teacher].slots[i] = TeacherSlot::PartiallyFilled {
                            class: class_slots.class_index as usize,
                            subject: *subject,
                        }
                    }
                },

                crate::logic::Slot::Double { first, second, .. } => {
                    match first {
                        crate::logic::SlotData::Empty => {}
                        crate::logic::SlotData::PartiallyFilled {
                            teacher, subject, ..
                        } => {
                            table[*teacher].slots[i] = TeacherSlot::PartiallyFilled {
                                class: class_slots.class_index as usize,
                                subject: *subject,
                            }
                        }
                    }

                    match second {
                        crate::logic::SlotData::Empty => {}
                        crate::logic::SlotData::PartiallyFilled {
                            teacher, subject, ..
                        } => {
                            table[*teacher].slots[i] = TeacherSlot::PartiallyFilled {
                                class: class_slots.class_index as usize,
                                subject: *subject,
                            }
                        }
                    }
                }
            }

            i += 1;
        }
    }

    table
}

pub fn teacher_count_per_shift(timetable: &Timetable) {
    let mut count1: HashMap<usize, u32> = HashMap::new();
    let mut count2: HashMap<usize, u32> = HashMap::new();

    for relation in timetable.data.relations.iter() {
        let n = relation.per_week_first + relation.per_week_second.unwrap_or(0);

        if relation.shift == 1 {
            count1
                .entry(relation.teacher)
                .and_modify(|c| *c += n)
                .or_insert(n);
        } else if relation.shift == 2 {
            count2
                .entry(relation.teacher)
                .and_modify(|c| *c += n)
                .or_insert(n);
        } else {
            println!("Invalid shift: {:?}", relation);
        }
    }

    for teacher_id in 0..timetable.data.teachers.len() {
        println!(
            "{}: {} {}",
            timetable.data.teachers[teacher_id].name,
            count1.get(&teacher_id).unwrap_or(&0),
            count2.get(&teacher_id).unwrap_or(&0),
        );
    }
}
