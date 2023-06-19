use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    vec,
};

use project_tomoyo::*;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::{cost, util};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Slot {
    Empty,
    PartiallyFilled {
        teacher: usize,
        subject: usize,
    },
    Filled {
        teacher: usize,
        subject: usize,
        room: usize,
    },
}

#[derive(Clone, Serialize, Debug)]
pub struct TimetableData {
    pub classes: Vec<Class>,
    pub rooms: Vec<Room>,
    pub subjects: Vec<Subject>,
    pub teachers: Vec<Teacher>,
    pub relations: Vec<Relation>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ClassSlots {
    pub class_index: u32,
    pub slots: Vec<Slot>,
}

#[derive(Clone, Serialize, Debug)]
pub struct Timetable {
    pub data: TimetableData,

    pub max_periods_per_day: u32,

    // Outer vector: Classes - S1A, S2A, etc.
    // Inner vector: Single class' timeslots - mon(1,2,3,4,5,6,7), tue(1,2,3,4,5,6,7), wed(1,2,3,4,5,6,7), thu(1,2,3,4,5,6,7), fri(1,2,3,4,5,6,7)
    pub table: Vec<ClassSlots>,
}

impl Timetable {
    pub fn new(data: TimetableData, max_periods_per_day: u32, table: Vec<ClassSlots>) -> Self {
        Self {
            data,
            max_periods_per_day,
            table,
        }
    }

    pub fn generate_random_table(&mut self) {
        let mut table: Vec<ClassSlots> = vec![];
        table.resize(
            self.data.classes.len(),
            ClassSlots {
                class_index: 0,
                slots: vec![],
            },
        );

        let mut i = 0;
        for class_slots in table.iter_mut() {
            class_slots.class_index = i;
            class_slots
                .slots
                .resize(5 * self.max_periods_per_day as usize, Slot::Empty);

            let mut class_relations: Vec<&Relation> = self
                .data
                .relations
                .iter()
                .filter_map(|relation| {
                    if relation.class == class_slots.class_index as usize {
                        Some(relation)
                    } else {
                        None
                    }
                })
                .collect();

            for slot in class_slots.slots.iter_mut() {
                if let Some(relation) = class_relations.pop() {
                    *slot = Slot::PartiallyFilled {
                        teacher: relation.teacher,
                        subject: relation.subject,
                    };
                }
            }

            i += 1;
        }

        self.table = table;
    }

    // HARD CONSTRAINTS:
    //  - No repeating teachers for a single period
    //  - No holes in a class's schedule
    //  - Not too many subjects with the same room kind (such as 5 subjects meant for computer classrooms being taught in the same period when there's only 3 computer classrooms in the school)

    //  - Respect a subject's max number of classes per day
    //  - Some classes should have one day per week with only a specified set of subjects (strucni predmeti)

    //  - * No repeating rooms for a single period
    //  - * Respect room kind

    // SOFT CONSTRAINTS:
    //  - How evenly spread the lessons are per day for each class
    //  - Teachers should have classes grouped per days (coming to school just to teach a single period is a pain)

    //  - Some subjects have preferred times during the day (P.E. should be last, math should be early, etc.)
    pub fn start_algorithm(&mut self, running: Arc<AtomicBool>) {
        let room_kinds_count = util::room_kinds_count(&self.data.rooms);

        // OLD ALGORITHM:
        {
            const ALPHA: f32 = 0.97;
            const T0: f32 = 1.0;
            const SA_MAX: i32 = 10000;

            let mut s = self.clone();
            let mut best_s = self.clone();

            let mut t = T0;

            while running.load(Ordering::Relaxed) {
                let mut exit = false;

                for _ in 0..SA_MAX {
                    let new_s = s.generate_neighbor();

                    let new_s_cost_hard = new_s.hard_points(&room_kinds_count);
                    let s_cost_hard = s.hard_points(&room_kinds_count);

                    if s_cost_hard == 0 {
                        exit = true;
                        break;
                    }

                    let delta_hard = new_s_cost_hard - s_cost_hard;

                    if delta_hard < 0 {
                        print!("NEW SOLUTION ACCEPTED (hard cost: {})", new_s_cost_hard);
                        s = new_s;

                        let best_s_cost_hard = best_s.hard_points(&room_kinds_count);
                        if new_s_cost_hard < best_s_cost_hard {
                            print!(" AS BEST");
                            best_s = s.clone();
                        }

                        println!(
                            ",  DELTA: {} ({} - {})",
                            delta_hard, new_s_cost_hard, s_cost_hard
                        );
                    } else {
                        let x: f64 = thread_rng().gen_range(0.0..1.0);

                        let base: f64 = std::f64::consts::E;
                        let exponent = (-delta_hard as f64) / (t as f64);
                        let chance = base.powf(exponent);
                        if x < chance {
                            print!(
                                "NEW SOLUTION ACCEPTED (hard cost: {}) BY CHANCE ({})",
                                new_s_cost_hard, chance
                            );
                            s = new_s;

                            println!(
                                ",  DELTA: {} ({} - {})",
                                delta_hard, new_s_cost_hard, s_cost_hard
                            );
                        }
                    }
                }

                if exit {
                    break;
                }

                t = t * ALPHA;
            }

            self.table = best_s.table;

            let mut s = self.clone();
            let mut best_s = self.clone();
            let mut t = T0;

            while running.load(Ordering::Relaxed) {
                let mut exit = false;

                for _ in 0..SA_MAX {
                    let new_s = s.generate_neighbor();

                    let new_s_cost_hard = new_s.hard_points(&room_kinds_count);
                    let new_s_cost_soft = new_s.soft_points();
                    let s_cost_soft = s.soft_points();

                    if s_cost_soft == 0 {
                        exit = true;
                        break;
                    }

                    let delta_soft = new_s_cost_soft - s_cost_soft;

                    if new_s_cost_hard == 0 {
                        if delta_soft < 0 {
                            print!("NEW SOLUTION ACCEPTED (soft cost: {})", new_s_cost_soft);
                            s = new_s;

                            let best_s_cost_soft = best_s.soft_points();
                            if new_s_cost_soft < best_s_cost_soft {
                                print!(" AS BEST");
                                best_s = s.clone();
                            }

                            println!(
                                ",  DELTA: {} ({} - {})",
                                delta_soft, new_s_cost_soft, s_cost_soft
                            );
                        } else {
                            let x: f64 = thread_rng().gen_range(0.0..1.0);

                            let base: f64 = std::f64::consts::E;
                            let exponent = (-delta_soft as f64) / (t as f64);
                            let chance = base.powf(exponent);
                            if x < chance {
                                print!(
                                    "NEW SOLUTION ACCEPTED (soft cost: {}) BY CHANCE ({})",
                                    new_s_cost_soft, chance
                                );
                                s = new_s;

                                println!(
                                    ",  DELTA: {} ({} - {})",
                                    delta_soft, new_s_cost_soft, s_cost_soft
                                );
                            }
                        }
                    }
                }

                if exit {
                    break;
                }

                t = t * ALPHA;
            }

            self.table = best_s.table;
        }

        // NEW ALGORITHM:
        {
            /*
            const L: i32 = 100;

            let mut s = self.clone();
            let initial_cost = s.hard_points(&room_kinds_count);

            let mut p: Vec<i32> = vec![];
            p.resize(L as usize, initial_cost);

            let mut best_s = self.clone();
            let mut best_cost = initial_cost;

            let mut i = 0;
            while running.load(Ordering::Relaxed) {
                let new_s = s.generate_neighbor();
                let v = i % L;

                let new_cost = new_s.hard_points(&room_kinds_count);
                if new_cost < p[v as usize] {
                    print!("New solution accepted");
                    s = new_s.clone();

                    if new_cost < best_cost {
                        print!(" (as best)");
                        best_s = new_s;
                        best_cost = new_cost;
                    }

                    println!("!! Cost: {}", new_cost);
                } /*else {
                      println!(
                          "New solution discarded. not {} < {}",
                          new_cost, p[v as usize]
                      );
                  }*/

                p[v as usize] = s.hard_points(&room_kinds_count);
                i += 1;
            }

            self.table = best_s.table;
            */
        }

        let cost1 = cost::hard_repeating_teachers(self);
        let cost2 = cost::hard_holes_in_class_timetable(self);
        let cost3 = cost::hard_too_many_subjects_of_same_kind(self, &room_kinds_count);
        let cost4 = cost::soft_class_spread(self);
        let cost5 = cost::soft_teacher_free_days(self);

        println!("DETAILED COST");
        println!(" (h) Repeating teachers: {}", cost1);
        println!(" (h) Holes: {}", cost2);
        println!(" (h) Too many subjects of same kind: {}", cost3);
        println!(" (s) Class spread: {}", cost4);
        println!(" (s) Teacher free days: {}", cost5);
    }

    pub fn generate_neighbor(&self) -> Self {
        let mut rng = thread_rng();

        let mut timetable = self.clone();
        let class_index: usize = rng.gen_range(0..self.table.len());

        let start_slot = rng.gen_range(0..timetable.table[class_index].slots.len());
        let end_slot = rng.gen_range(0..timetable.table[class_index].slots.len());

        let tmp = timetable.table[class_index].slots[start_slot];
        timetable.table[class_index].slots[start_slot] =
            timetable.table[class_index].slots[end_slot];
        timetable.table[class_index].slots[end_slot] = tmp;

        timetable
    }

    // Should be 0
    pub fn hard_points(&self, room_kinds_count: &HashMap<String, u32>) -> i32 {
        let mut points = 0;

        points += cost::hard_repeating_teachers(self);
        points += cost::hard_holes_in_class_timetable(self);
        points += cost::hard_too_many_subjects_of_same_kind(self, room_kinds_count);

        points
    }

    // Should be as close to 0 as possible
    pub fn soft_points(&self) -> i32 {
        let mut points = 0;

        points += cost::soft_class_spread(self);
        // points += cost::soft_teacher_free_days(self);

        points
    }

    pub fn fill_rooms(&mut self) {
        for period in 0..(self.max_periods_per_day * 5) {
            let mut used_rooms: Vec<usize> = vec![];

            for class in 0..self.table.len() {
                match self.table[class].slots[period as usize] {
                    Slot::PartiallyFilled { teacher, subject } => {
                        // Choose room
                        let mut room = 0;

                        for room_index in 0..self.data.rooms.len() {
                            if self.data.rooms[room_index].kind == self.data.subjects[subject].kind
                                && !used_rooms.contains(&room_index)
                            {
                                used_rooms.push(room_index);
                                room = room_index;

                                break;
                            }
                        }

                        self.table[class].slots[period as usize] = Slot::Filled {
                            teacher,
                            subject,
                            room,
                        }
                    }

                    _ => {}
                }
            }
        }
    }
}
