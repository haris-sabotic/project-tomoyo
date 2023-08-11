use std::{
    println,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    unreachable, vec,
};

use project_tomoyo::*;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use ws::Sender;

use crate::{cost, util};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum SlotData {
    Empty,
    PartiallyFilled { teacher: usize, subject: usize },
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Slot {
    Single(SlotData),
    Double {
        first: SlotData,
        second: SlotData,
        before: u32,
        after: u32,
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

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Shift {
    First,
    Second,
}

impl Shift {
    pub fn to_i32(&self) -> i32 {
        match self {
            Shift::First => 1,
            Shift::Second => 2,
        }
    }

    pub fn from_i32(v: i32) -> Self {
        match v {
            1 => Self::First,
            2 => Self::Second,
            _ => panic!("Invalid shift"),
        }
    }

    pub fn equals(&self, v: i32) -> bool {
        self.to_i32() == v
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct Timetable {
    pub data: TimetableData,

    pub max_periods_per_day: u32,

    // Outer vector: Classes - S1A, S2A, etc.
    // Inner vector: Single class' timeslots - mon(1,2,3,4,5,6,7), tue(1,2,3,4,5,6,7), wed(1,2,3,4,5,6,7), thu(1,2,3,4,5,6,7), fri(1,2,3,4,5,6,7)
    pub table1: Vec<ClassSlots>,
    pub table2: Vec<ClassSlots>,
}

impl Timetable {
    pub fn new(
        data: TimetableData,
        max_periods_per_day: u32,
        table1: Vec<ClassSlots>,
        table2: Vec<ClassSlots>,
    ) -> Self {
        Self {
            data,
            max_periods_per_day,
            table1,
            table2,
        }
    }

    pub fn table(&self, shift: Shift) -> &Vec<ClassSlots> {
        match shift {
            Shift::First => &self.table1,
            Shift::Second => &self.table2,
        }
    }

    pub fn table_mut(&mut self, shift: Shift) -> &mut Vec<ClassSlots> {
        match shift {
            Shift::First => &mut self.table1,
            Shift::Second => &mut self.table2,
        }
    }

    pub fn generate_random_table(&mut self, shift: Shift, _out: &Sender) {
        let mut table: Vec<ClassSlots> = vec![];
        table.resize(
            self.data.classes.len(),
            ClassSlots {
                class_index: 0,
                slots: vec![],
            },
        );

        let mut c = 0;
        for class_slots in table.iter_mut() {
            println!("{}, {:?}", c, self.data.classes[c]);
            class_slots.class_index = c as u32;
            class_slots.slots.resize(
                5 * self.max_periods_per_day as usize,
                Slot::Single(SlotData::Empty),
            );

            // get all relations of the current class
            let class_relations: Vec<&Relation> = self
                .data
                .relations
                .iter()
                .filter_map(|relation| {
                    if shift.equals(relation.shift)
                        && relation.class == class_slots.class_index as usize
                    {
                        Some(relation)
                    } else {
                        None
                    }
                })
                .collect();

            let mut leftover_doubles: Vec<Relation> = vec![];

            println!("Placing single relations...");

            for relation in class_relations.iter() {
                let relation_slot = SlotData::PartiallyFilled {
                    teacher: relation.teacher,
                    subject: relation.subject,
                };

                match relation.per_week_second {
                    // single relation
                    None => {
                        println!(
                            "T: {},  S: {}    [{}]",
                            self.data.teachers[relation.teacher].name,
                            self.data.subjects[relation.subject].name,
                            relation.per_week_first,
                        );

                        for i in 0..class_slots.slots.len() {
                            match class_slots.slots[i] {
                                Slot::Single(SlotData::Empty) => {
                                    for j in 0..relation.per_week_first {
                                        class_slots.slots[i + j as usize] =
                                            Slot::Single(relation_slot);
                                    }

                                    break;
                                }

                                _ => {}
                            }
                        }
                    }

                    // double relation
                    Some(_) => {
                        leftover_doubles.push((*relation).clone());
                    }
                }
            }

            println!("Moving on to double relations...");

            // Sort leftover double relations
            leftover_doubles
                .sort_by(|a, b| a.per_week_first.partial_cmp(&b.per_week_first).unwrap());
            leftover_doubles.reverse();

            for relation in leftover_doubles {
                let relation_slot = SlotData::PartiallyFilled {
                    teacher: relation.teacher,
                    subject: relation.subject,
                };

                match relation.per_week_second {
                    Some(per_week_second) => {
                        println!(
                            "T: {},  S: {}    [{} / {:?}]",
                            self.data.teachers[relation.teacher].name,
                            self.data.subjects[relation.subject].name,
                            relation.per_week_first,
                            per_week_second
                        );

                        let per_week_first = relation.per_week_first;

                        let mut first_group_placed = false;

                        for i in 0..class_slots.slots.len() {
                            match class_slots.slots[i] {
                                Slot::Double {
                                    first,
                                    second,
                                    before,
                                    after,
                                } => {
                                    if before == 0 && after >= per_week_first - 1 {
                                        // offset at which to start replacing, so that empty spaces are at the start rather than the end
                                        let offset = after + 1 - per_week_first;

                                        match (first, second) {
                                            (SlotData::Empty, _) => {
                                                // don't place in the double block if the teacher is the same
                                                match second {
                                                    SlotData::PartiallyFilled {
                                                        teacher, ..
                                                    } => {
                                                        if teacher == relation.teacher {
                                                            continue;
                                                        }
                                                    }
                                                    _ => {}
                                                }

                                                // continue if it isn't fully empty
                                                match class_slots.slots[i + after as usize] {
                                                    Slot::Double {
                                                        first: SlotData::Empty,
                                                        ..
                                                    } => {}

                                                    _ => continue,
                                                }

                                                for j in 0..per_week_first {
                                                    let (_before, _after) = match class_slots.slots
                                                        [i + j as usize + offset as usize]
                                                    {
                                                        Slot::Single(_) => unreachable!(),
                                                        Slot::Double { before, after, .. } => {
                                                            (before, after)
                                                        }
                                                    };

                                                    class_slots.slots
                                                        [i + j as usize + offset as usize] =
                                                        Slot::Double {
                                                            first: relation_slot,
                                                            second: second,
                                                            before: _before,
                                                            after: _after,
                                                        };
                                                }

                                                first_group_placed = true;

                                                break;
                                            }

                                            (_, SlotData::Empty) => {
                                                // don't place in the double block if the teacher is the same
                                                match first {
                                                    SlotData::PartiallyFilled {
                                                        teacher, ..
                                                    } => {
                                                        if teacher == relation.teacher {
                                                            continue;
                                                        }
                                                    }
                                                    _ => {}
                                                }

                                                // continue if it isn't fully empty
                                                match class_slots.slots[i + after as usize] {
                                                    Slot::Double {
                                                        second: SlotData::Empty,
                                                        ..
                                                    } => {}

                                                    _ => continue,
                                                }

                                                for j in 0..per_week_first {
                                                    let (_before, _after) = match class_slots.slots
                                                        [i + j as usize + offset as usize]
                                                    {
                                                        Slot::Single(_) => unreachable!(),
                                                        Slot::Double { before, after, .. } => {
                                                            (before, after)
                                                        }
                                                    };

                                                    class_slots.slots
                                                        [i + j as usize + offset as usize] =
                                                        Slot::Double {
                                                            first: first,
                                                            second: relation_slot,
                                                            before: _before,
                                                            after: _after,
                                                        };
                                                }

                                                first_group_placed = true;

                                                break;
                                            }

                                            _ => {}
                                        }
                                    }
                                }

                                _ => {}
                            }
                        }

                        if !first_group_placed {
                            for i in 0..class_slots.slots.len() {
                                match class_slots.slots[i] {
                                    Slot::Single(SlotData::Empty) => {
                                        for j in 0..per_week_first {
                                            class_slots.slots[i + j as usize] = Slot::Double {
                                                first: relation_slot,
                                                second: SlotData::Empty,
                                                before: j,
                                                after: per_week_first - j - 1,
                                            };
                                        }

                                        break;
                                    }

                                    _ => {}
                                }
                            }
                        }

                        let mut second_group_placed = false;

                        for i in 0..class_slots.slots.len() {
                            match class_slots.slots[i] {
                                Slot::Double {
                                    first,
                                    second,
                                    before,
                                    after,
                                } => {
                                    if before == 0 && after >= per_week_second - 1 {
                                        // offset at which to start replacing, so that empty spaces are at the start rather than the end
                                        let offset = after + 1 - per_week_second;

                                        match (first, second) {
                                            (SlotData::Empty, _) => {
                                                // don't place in the double block if the teacher is the same
                                                match second {
                                                    SlotData::PartiallyFilled {
                                                        teacher, ..
                                                    } => {
                                                        if teacher == relation.teacher {
                                                            continue;
                                                        }
                                                    }
                                                    _ => {}
                                                }

                                                // continue if it isn't fully empty
                                                match class_slots.slots[i + after as usize] {
                                                    Slot::Double {
                                                        first: SlotData::Empty,
                                                        ..
                                                    } => {}

                                                    _ => continue,
                                                }

                                                for j in 0..per_week_second {
                                                    let (_before, _after) = match class_slots.slots
                                                        [i + j as usize + offset as usize]
                                                    {
                                                        Slot::Single(_) => unreachable!(),
                                                        Slot::Double { before, after, .. } => {
                                                            (before, after)
                                                        }
                                                    };

                                                    class_slots.slots
                                                        [i + j as usize + offset as usize] =
                                                        Slot::Double {
                                                            first: relation_slot,
                                                            second: second,
                                                            before: _before,
                                                            after: _after,
                                                        };
                                                }

                                                second_group_placed = true;

                                                break;
                                            }

                                            (_, SlotData::Empty) => {
                                                // don't place in the double block if the teacher is the same
                                                match first {
                                                    SlotData::PartiallyFilled {
                                                        teacher, ..
                                                    } => {
                                                        if teacher == relation.teacher {
                                                            continue;
                                                        }
                                                    }
                                                    _ => {}
                                                }

                                                // continue if it isn't fully empty
                                                match class_slots.slots[i + after as usize] {
                                                    Slot::Double {
                                                        second: SlotData::Empty,
                                                        ..
                                                    } => {}

                                                    _ => continue,
                                                }

                                                for j in 0..per_week_second {
                                                    let (_before, _after) = match class_slots.slots
                                                        [i + j as usize + offset as usize]
                                                    {
                                                        Slot::Single(_) => unreachable!(),
                                                        Slot::Double { before, after, .. } => {
                                                            (before, after)
                                                        }
                                                    };

                                                    class_slots.slots
                                                        [i + j as usize + offset as usize] =
                                                        Slot::Double {
                                                            first: first,
                                                            second: relation_slot,
                                                            before: _before,
                                                            after: _after,
                                                        };
                                                }

                                                second_group_placed = true;

                                                break;
                                            }

                                            _ => {}
                                        }
                                    }
                                }

                                _ => {}
                            }
                        }

                        if !second_group_placed {
                            for i in 0..class_slots.slots.len() {
                                match class_slots.slots[i] {
                                    Slot::Single(SlotData::Empty) => {
                                        for j in 0..per_week_second {
                                            class_slots.slots[i + j as usize] = Slot::Double {
                                                first: relation_slot,
                                                second: SlotData::Empty,
                                                before: j,
                                                after: per_week_second - j - 1,
                                            };
                                        }

                                        break;
                                    }

                                    _ => {}
                                }
                            }
                        }
                    }

                    None => {}
                }
            }

            println!("Done.");

            println!("");
            c += 1;
        }

        match shift {
            Shift::First => self.table1 = table,
            Shift::Second => self.table2 = table,
        }
    }

    pub fn start_algorithm(
        &mut self,
        running: Arc<AtomicBool>,
        out: &Sender,
        alpha: f64,
        t0: f64,
        sa_max: i64,
        static_classes: &String,
        hard_1: i32,
        soft_1: i32,
        hard_2: i32,
        soft_2: i32,
    ) {
        // SIMULATED ANNEALLING:
        {
            let mut s1 = self.clone();
            let mut s2 = self.clone();
            // let mut best_s1 = self.clone();
            // let mut best_s2 = self.clone();

            let mut t = t0;

            while running.load(Ordering::Relaxed) {
                for _ in 0..sa_max {
                    let new_s1 = s1.generate_neighbor(Shift::First, out, static_classes);

                    let new_s1_cost_hard = hard_1 * new_s1.hard_points(Shift::First);
                    let s1_cost_hard = hard_1 * s1.hard_points(Shift::First);

                    let new_s1_cost_soft = soft_1 * new_s1.soft_points(Shift::First);
                    let s1_cost_soft = soft_1 * s1.soft_points(Shift::First);

                    let new_s1_cost = new_s1_cost_hard + new_s1_cost_soft;
                    let s1_cost = s1_cost_hard + s1_cost_soft;

                    let delta1 = new_s1_cost - s1_cost;

                    let mut updated1 = false;
                    if delta1 <= 0 {
                        s1 = new_s1;
                        updated1 = true;

                    /*
                    let best_s1_cost_hard = best_s1.hard_points(Shift::First);
                    let best_s1_cost_soft = best_s1.soft_points(Shift::First);
                    let best_s1_cost_other = best_s1.other_points();
                    let best_s1_cost =
                        best_s1_cost_soft + best_s1_cost_hard + best_s1_cost_other;
                    if new_s1_cost < best_s1_cost {
                        best_s1 = s1.clone();
                    }
                    */
                    } else {
                        let x: f64 = thread_rng().gen_range(0.0..1.0);

                        let base: f64 = std::f64::consts::E;

                        let exponent = (-delta1 as f64) / (t as f64);

                        let chance = base.powf(exponent);
                        if x < chance {
                            s1 = new_s1;
                            updated1 = true;
                        }
                    }

                    // ================

                    let new_s2 = s2.generate_neighbor(Shift::Second, out, static_classes);

                    let new_s2_cost_hard = hard_2 * new_s2.hard_points(Shift::Second);
                    let s2_cost_hard = hard_2 * s2.hard_points(Shift::Second);

                    let new_s2_cost_soft = soft_2 * new_s2.soft_points(Shift::Second);
                    let s2_cost_soft = soft_2 * s2.soft_points(Shift::Second);

                    let new_s2_cost = new_s2_cost_hard + new_s2_cost_soft;
                    let s2_cost = s2_cost_hard + s2_cost_soft;

                    let delta2 = new_s2_cost - s2_cost;

                    let mut updated2 = false;
                    if delta2 <= 0 {
                        s2 = new_s2;
                        updated2 = true;

                    /*
                    let best_s2_cost_hard = best_s2.hard_points(Shift::Second);
                    let best_s2_cost_soft = best_s2.soft_points(Shift::Second);
                    let best_s2_cost_other = best_s2.other_points();
                    let best_s2_cost =
                        best_s2_cost_soft + best_s2_cost_hard + best_s2_cost_other;
                    if new_s2_cost < best_s2_cost {
                        best_s2 = s2.clone();
                    }
                    */
                    } else {
                        let x: f64 = thread_rng().gen_range(0.0..1.0);

                        let base: f64 = std::f64::consts::E;

                        let exponent = (-delta2 as f64) / (t as f64);

                        let chance = base.powf(exponent);
                        if x < chance {
                            s2 = new_s2;
                            updated2 = true;
                        }
                    }

                    // ================

                    if updated1 || updated2 {
                        let (hard1, soft1) = if updated1 {
                            (new_s1_cost_hard, new_s1_cost_soft)
                        } else {
                            (s1_cost_hard, s1_cost_soft)
                        };

                        let (hard2, soft2) = if updated2 {
                            (new_s2_cost_hard, new_s2_cost_soft)
                        } else {
                            (s2_cost_hard, s2_cost_soft)
                        };

                        println!(
                            "[TEMP: {}]    [{: >3}, {: >3}]    [{: >3}, {: >3}]",
                            t, hard1, soft1, hard2, soft2,
                        );
                    }
                }

                t = t * alpha;
            }

            self.table1 = s1.table1;
            self.table2 = s2.table2;
        }

        self.detailed_cost(hard_1, soft_1, hard_2, soft_2);
    }

    pub fn detailed_cost(&self, hard_1: i32, soft_1: i32, hard_2: i32, soft_2: i32) {
        let teacher_table1 =
            util::class_table_to_teacher_table(&self.table1, &self.data, self.max_periods_per_day);
        let teacher_table2 =
            util::class_table_to_teacher_table(&self.table2, &self.data, self.max_periods_per_day);

        println!("DETAILED COST");

        let hcost1 = hard_1 * cost::hard_repeating_teachers(self, Shift::First);
        let hcost2 = hard_1 * cost::hard_holes_in_class_timetable(self, Shift::First);
        let hcost3 = hard_1 * cost::hard_too_many_subjects_of_same_kind(self, Shift::First);
        let hcost4 = hard_1 * cost::hard_block_classes(self, Shift::First);
        let hcost5 = hard_1 * cost::hard_specific_subject_days(self, Shift::First);
        let hcost6 = hard_1 * cost::hard_subject_per_day_limits(self, Shift::First);
        let hcost7 = hard_1 * cost::hard_subject_holes(self, Shift::First);
        let hcost8 = hard_1 * cost::hard_teacher_shift_spread(self, Shift::First);
        let hcost9 = hard_1 * cost::hard_teacher_extra_constraints(self, Shift::First);

        let scost1 = soft_1 * cost::soft_class_spread(self, Shift::First);
        let scost2 = soft_1 * cost::soft_teacher_class_spread(self, &teacher_table1);
        let scost3 = soft_1 * cost::soft_holes_in_teacher_timetable(self, &teacher_table1);
        let scost4 = soft_1 * cost::soft_preferred_subject_times(self, Shift::First);

        println!(
            "FIRST SHIFT ({}, {}):",
            hcost1 + hcost2 + hcost3 + hcost4 + hcost5 + hcost6 + hcost7 + hcost8 + hcost9,
            scost1 + scost2 + scost3 + scost4
        );
        println!("  (h) Repeating teachers: {}", hcost1);
        println!("  (h) Holes: {}", hcost2);
        println!("  (h) Too many subjects of same kind: {}", hcost3);
        println!("  (h) Block classes: {}", hcost4);
        println!("  (h) Specific subject days: {}", hcost5);
        println!("  (h) Subject per day limits: {}", hcost6);
        println!("  (h) Subject holes: {}", hcost7);
        println!("  (h) Teacher shift spread: {}", hcost8);
        println!("  (h) Teacher extra constraints: {}", hcost9);
        println!("  (s) Class spread: {}", scost1);
        println!("  (s) Teacher class spread: {}", scost2);
        println!("  (s) Teacher holes: {}", scost3);
        println!("  (s) Soft preferred subject times: {}", scost4);

        let hcost1 = hard_2 * cost::hard_repeating_teachers(self, Shift::Second);
        let hcost2 = hard_2 * cost::hard_holes_in_class_timetable(self, Shift::Second);
        let hcost3 = hard_2 * cost::hard_too_many_subjects_of_same_kind(self, Shift::Second);
        let hcost4 = hard_2 * cost::hard_block_classes(self, Shift::Second);
        let hcost5 = hard_2 * cost::hard_specific_subject_days(self, Shift::Second);
        let hcost6 = hard_2 * cost::hard_subject_per_day_limits(self, Shift::Second);
        let hcost7 = hard_2 * cost::hard_subject_holes(self, Shift::Second);
        let hcost8 = hard_2 * cost::hard_teacher_shift_spread(self, Shift::Second);
        let hcost9 = hard_2 * cost::hard_teacher_extra_constraints(self, Shift::Second);

        let scost1 = soft_2 * cost::soft_class_spread(self, Shift::Second);
        let scost2 = soft_2 * cost::soft_teacher_class_spread(self, &teacher_table2);
        let scost3 = soft_2 * cost::soft_holes_in_teacher_timetable(self, &teacher_table2);
        let scost4 = soft_2 * cost::soft_preferred_subject_times(self, Shift::Second);

        println!(
            "SECOND SHIFT ({}, {}):",
            hcost1 + hcost2 + hcost3 + hcost4 + hcost5 + hcost6 + hcost7 + hcost8 + hcost9,
            scost1 + scost2 + scost3 + scost4
        );
        println!("  (h) Repeating teachers: {}", hcost1);
        println!("  (h) Holes: {}", hcost2);
        println!("  (h) Too many subjects of same kind: {}", hcost3);
        println!("  (h) Block classes: {}", hcost4);
        println!("  (h) Specific subject days: {}", hcost5);
        println!("  (h) Subject per day limits: {}", hcost6);
        println!("  (h) Subject holes: {}", hcost7);
        println!("  (h) Teacher shift spread: {}", hcost8);
        println!("  (h) Teacher extra constraints: {}", hcost9);
        println!("  (s) Class spread: {}", scost1);
        println!("  (s) Teacher class spread: {}", scost2);
        println!("  (s) Teacher holes: {}", scost3);
        println!("  (s) Soft preferred subject times: {}", scost4);
    }

    pub fn generate_neighbor(&self, shift: Shift, _out: &Sender, static_classes: &String) -> Self {
        let mut rng = thread_rng();

        let mut timetable = self.clone();

        let class_index = {
            let mut c: usize = rng.gen_range(0..self.data.classes.len());
            let sc: Vec<String> = static_classes
                .as_str()
                .split(',')
                .map(|s| s.to_string())
                .collect();

            while sc.contains(&self.data.classes[c].name) {
                c = rng.gen_range(0..self.data.classes.len());
            }

            c
        };

        let mut start_index = rng.gen_range(0..timetable.table(shift)[class_index].slots.len());

        // println!("Class: {}", self.data.classes[class_index].name);
        match timetable.table(shift)[class_index].slots[start_index] {
            Slot::Single(_) => {
                let mut end_index =
                    rng.gen_range(0..timetable.table(shift)[class_index].slots.len());

                // println!("{} - {}", start_index, end_index);

                // regenerate if the slot at `end_index` is a Double block
                loop {
                    match timetable.table(shift)[class_index].slots[end_index] {
                        Slot::Single(_) => break,
                        Slot::Double { .. } => {
                            end_index =
                                rng.gen_range(0..timetable.table(shift)[class_index].slots.len())
                        }
                    }
                }

                let tmp = timetable.table(shift)[class_index].slots[start_index];
                timetable.table_mut(shift)[class_index].slots[start_index] =
                    timetable.table(shift)[class_index].slots[end_index];
                timetable.table_mut(shift)[class_index].slots[end_index] = tmp;
            }
            Slot::Double {
                first: _,
                second: _,
                before: before_start,
                after: after_start,
            } => {
                // println!("Before start: {}", before_start);
                // set start index to the start of this double block
                start_index -= before_start as usize;
                let length = before_start + after_start + 1;

                let mut done = false;

                while !done {
                    // choose a random day
                    let day = rng.gen_range(0..5);

                    let index = day * timetable.max_periods_per_day
                        + rng.gen_range(0..(timetable.max_periods_per_day - length));

                    // println!("{} - {}", start_index, index);

                    match timetable.table(shift)[class_index].slots[index as usize] {
                        Slot::Single(_) => {
                            let mut singles_in_a_row = 0;
                            for j in 0..length {
                                match timetable.table(shift)[class_index].slots
                                    [index as usize + j as usize]
                                {
                                    Slot::Single(_) => singles_in_a_row += 1,
                                    Slot::Double { .. } => singles_in_a_row = 0,
                                }
                            }

                            if singles_in_a_row == length {
                                for j in 0..length {
                                    let a = timetable.table(shift)[class_index].slots
                                        [start_index + j as usize]
                                        .clone();
                                    let b = timetable.table(shift)[class_index].slots
                                        [index as usize + j as usize]
                                        .clone();

                                    timetable.table_mut(shift)[class_index].slots
                                        [start_index + j as usize] = b;

                                    timetable.table_mut(shift)[class_index].slots
                                        [index as usize + j as usize] = a;
                                }

                                done = true;
                            }
                        }
                        Slot::Double {
                            first: _,
                            second: _,
                            before: before_end,
                            after: after_end,
                        } => {
                            // if it's a double block of the same length
                            if before_end + 1 + after_end == length {
                                for j in 0..length {
                                    let a = timetable.table(shift)[class_index].slots
                                        [start_index + j as usize]
                                        .clone();
                                    let b = timetable.table(shift)[class_index].slots
                                        [index as usize - before_end as usize + j as usize]
                                        .clone();

                                    timetable.table_mut(shift)[class_index].slots
                                        [start_index + j as usize] = b;

                                    timetable.table_mut(shift)[class_index].slots
                                        [index as usize - before_end as usize + j as usize] = a;
                                }

                                done = true;
                            }
                        }
                    }
                }
            }
        }

        // println!("=================\n");

        // send_timetable(&timetable, _out);

        timetable
    }

    // Should be 0
    pub fn hard_points(&self, shift: Shift) -> i32 {
        let mut points = 0;

        points += cost::hard_repeating_teachers(self, shift);
        points += cost::hard_holes_in_class_timetable(self, shift);
        points += cost::hard_too_many_subjects_of_same_kind(self, shift);
        points += cost::hard_block_classes(self, shift);
        points += cost::hard_specific_subject_days(self, shift);
        points += cost::hard_subject_per_day_limits(self, shift);
        points += cost::hard_subject_holes(self, shift);
        points += cost::hard_teacher_shift_spread(self, shift);
        points += cost::hard_teacher_extra_constraints(self, shift);

        points
    }

    // Should be as close to 0 as possible
    pub fn soft_points(&self, shift: Shift) -> i32 {
        let mut points = 0;

        let teacher_table = util::class_table_to_teacher_table(
            &self.table(shift),
            &self.data,
            self.max_periods_per_day,
        );

        points += cost::soft_class_spread(self, shift);
        points += cost::soft_teacher_class_spread(self, &teacher_table);
        points += cost::soft_holes_in_teacher_timetable(self, &teacher_table);
        points += cost::soft_preferred_subject_times(self, shift);

        points
    }

    pub fn fill_rooms(&mut self) {
        /*
        for period in 0..(self.max_periods_per_day * 5) {
            let mut used_rooms: Vec<usize> = vec![];

            for class in 0..self.table.len() {
                match self.table[class].slots[period as usize] {
                    Slot::Single(slot) => {
                        match slot {
                            SlotData::PartiallyFilled { teacher, subject } => {
                                // Choose room
                                let mut room = 0;

                                for room_index in 0..self.data.rooms.len() {
                                    if self.data.rooms[room_index].kind
                                        == self.data.subjects[subject].kind
                                        && !used_rooms.contains(&room_index)
                                    {
                                        used_rooms.push(room_index);
                                        room = room_index;

                                        break;
                                    }
                                }

                                self.table[class].slots[period as usize] =
                                    Slot::Single(SlotData::Filled {
                                        teacher,
                                        subject,
                                        room,
                                    });
                            }

                            _ => {}
                        }
                    }
                    Slot::Double { first, second, .. } => {
                        match first {
                            SlotData::PartiallyFilled { teacher, subject } => {
                                // Choose room
                                let mut room = 0;

                                for room_index in 0..self.data.rooms.len() {
                                    if self.data.rooms[room_index].kind
                                        == self.data.subjects[subject].kind
                                        && !used_rooms.contains(&room_index)
                                    {
                                        used_rooms.push(room_index);
                                        room = room_index;

                                        break;
                                    }
                                }

                                self.table[class].slots[period as usize] =
                                    Slot::Single(SlotData::Filled {
                                        teacher,
                                        subject,
                                        room,
                                    });
                            }

                            _ => {}
                        }

                        match second {
                            SlotData::PartiallyFilled { teacher, subject } => {
                                // Choose room
                                let mut room = 0;

                                for room_index in 0..self.data.rooms.len() {
                                    if self.data.rooms[room_index].kind
                                        == self.data.subjects[subject].kind
                                        && !used_rooms.contains(&room_index)
                                    {
                                        used_rooms.push(room_index);
                                        room = room_index;

                                        break;
                                    }
                                }

                                self.table[class].slots[period as usize] =
                                    Slot::Single(SlotData::Filled {
                                        teacher,
                                        subject,
                                        room,
                                    });
                            }

                            _ => {}
                        }
                    }
                }
            }
        }
        */
    }
}
