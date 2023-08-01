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

use crate::{cost, send_timetable, util};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum SlotData {
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

    pub fn generate_random_table(&mut self, _out: &Sender) {
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
                    if relation.class == class_slots.class_index as usize {
                        Some(relation)
                    } else {
                        None
                    }
                })
                .collect();

            for relation in class_relations.iter() {
                println!(
                    "T: {},  S: {},  F: {},  S: {:?}",
                    self.data.teachers[relation.teacher].name,
                    self.data.subjects[relation.subject].name,
                    relation.per_week_first,
                    relation.per_week_second
                );

                let relation_slot = SlotData::PartiallyFilled {
                    teacher: relation.teacher,
                    subject: relation.subject,
                };

                match relation.per_week_second {
                    // single relation
                    None => {
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
                    Some(per_week_second) => {
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

                        /*
                        if self.data.classes[relation.class].name == "S4E"
                            && self.data.subjects[relation.subject].name
                                == "Aplikativni softver (P)"
                        {
                            self.table = table;
                            return;
                        }
                        */
                    }
                }
            }

            println!("");
            c += 1;
        }

        self.table = table;
    }

    pub fn start_algorithm(
        &mut self,
        running: Arc<AtomicBool>,
        out: &Sender,
        alpha: f64,
        t0: f64,
        sa_max: i64,
    ) {
        // SIMULATED ANNEALLING:
        {
            let mut s = self.clone();
            let mut best_s = self.clone();

            let mut t = t0;

            while running.load(Ordering::Relaxed) {
                let mut exit = false;

                for _ in 0..sa_max {
                    let new_s = s.generate_neighbor(out);

                    let new_s_cost_hard = new_s.hard_points();
                    let s_cost_hard = s.hard_points();

                    let new_s_cost_soft = new_s.soft_points();
                    let s_cost_soft = s.soft_points();

                    let new_s_cost = new_s_cost_hard + new_s_cost_soft;
                    let s_cost = s_cost_hard + s_cost_soft;

                    if s_cost == 0 {
                        exit = true;
                        break;
                    }

                    let delta = new_s_cost - s_cost;

                    if delta <= 0 {
                        print!(
                            "[TEMP: {}] NEW SOLUTION ACCEPTED (hard cost: {}, soft cost: {})",
                            t, new_s_cost_hard, new_s_cost_soft
                        );
                        s = new_s;

                        let best_s_cost_hard = best_s.hard_points();
                        let best_s_cost_soft = best_s.soft_points();
                        let best_s_cost = best_s_cost_soft + best_s_cost_hard;
                        if new_s_cost < best_s_cost {
                            print!(" AS BEST");
                            best_s = s.clone();
                        }

                        println!(",  DELTA: {}", delta);
                    } else {
                        let x: f64 = thread_rng().gen_range(0.0..1.0);

                        let base: f64 = std::f64::consts::E;

                        let exponent = (-delta as f64) / (t as f64);

                        let chance = base.powf(exponent);
                        if x < chance {
                            print!(
                                "[TEMP: {}] NEW SOLUTION ACCEPTED (hard cost: {}, soft cost: {}) BY CHANCE ({})",
                                t, new_s_cost_hard, new_s_cost_soft, chance
                            );
                            s = new_s;

                            println!(",  DELTA: {}", delta);
                        }
                    }
                }

                if exit {
                    break;
                }

                t = t * alpha;
            }

            self.table = best_s.table;
        }

        self.detailed_cost();
    }

    pub fn detailed_cost(&self) {
        let teacher_table =
            util::class_table_to_teacher_table(&self.table, &self.data, self.max_periods_per_day);

        let hcost1 = 2 * cost::hard_repeating_teachers(self);
        let hcost2 = 2 * cost::hard_holes_in_class_timetable(self);
        let hcost3 = 2 * cost::hard_too_many_subjects_of_same_kind(self);
        let hcost4 = 2 * cost::hard_block_classes(self);
        let hcost5 = 2 * cost::hard_specific_subject_days(self);
        let hcost6 = 2 * cost::hard_subject_per_day_limits(self);
        let hcost7 = 2 * cost::hard_subject_holes(self);
        let scost1 = cost::soft_class_spread(self);
        let scost2 = cost::soft_teacher_class_spread(self, &teacher_table);
        let scost3 = cost::soft_holes_in_teacher_timetable(self, &teacher_table);
        let scost4 = cost::soft_preferred_subject_times(self);

        println!("DETAILED COST");
        println!(" (h) Repeating teachers: {}", hcost1);
        println!(" (h) Holes: {}", hcost2);
        println!(" (h) Too many subjects of same kind: {}", hcost3);
        println!(" (h) Block classes: {}", hcost4);
        println!(" (h) Specific subject days: {}", hcost5);
        println!(" (h) Subject per day limits: {}", hcost6);
        println!(" (h) Subject holes: {}", hcost7);
        println!(" (s) Class spread: {}", scost1);
        println!(" (s) Teacher class spread: {}", scost2);
        println!(" (s) Teacher holes: {}", scost3);
        println!(" (s) Soft preferred subject times: {}", scost4);
    }

    pub fn generate_neighbor(&self, _out: &Sender) -> Self {
        let mut rng = thread_rng();

        let mut timetable = self.clone();

        let class_index: usize = rng.gen_range(0..self.data.classes.len());

        let mut start_index = rng.gen_range(0..timetable.table[class_index].slots.len());

        // println!("Class: {}", self.data.classes[class_index].name);
        match timetable.table[class_index].slots[start_index] {
            Slot::Single(_) => {
                let mut end_index = rng.gen_range(0..timetable.table[class_index].slots.len());

                // println!("{} - {}", start_index, end_index);

                // regenerate if the slot at `end_index` is a Double block
                loop {
                    match timetable.table[class_index].slots[end_index] {
                        Slot::Single(_) => break,
                        Slot::Double { .. } => {
                            end_index = rng.gen_range(0..timetable.table[class_index].slots.len())
                        }
                    }
                }

                let tmp = timetable.table[class_index].slots[start_index];
                timetable.table[class_index].slots[start_index] =
                    timetable.table[class_index].slots[end_index];
                timetable.table[class_index].slots[end_index] = tmp;
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

                    match timetable.table[class_index].slots[index as usize] {
                        Slot::Single(_) => {
                            let mut singles_in_a_row = 0;
                            for j in 0..length {
                                match timetable.table[class_index].slots
                                    [index as usize + j as usize]
                                {
                                    Slot::Single(_) => singles_in_a_row += 1,
                                    Slot::Double { .. } => singles_in_a_row = 0,
                                }
                            }

                            if singles_in_a_row == length {
                                for j in 0..length {
                                    let a = timetable.table[class_index].slots
                                        [start_index + j as usize]
                                        .clone();
                                    let b = timetable.table[class_index].slots
                                        [index as usize + j as usize]
                                        .clone();

                                    timetable.table[class_index].slots[start_index + j as usize] =
                                        b;

                                    timetable.table[class_index].slots
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
                                    let a = timetable.table[class_index].slots
                                        [start_index + j as usize]
                                        .clone();
                                    let b = timetable.table[class_index].slots
                                        [index as usize - before_end as usize + j as usize]
                                        .clone();

                                    timetable.table[class_index].slots[start_index + j as usize] =
                                        b;

                                    timetable.table[class_index].slots
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
    pub fn hard_points(&self) -> i32 {
        let mut points = 0;

        points += 2 * cost::hard_repeating_teachers(self);
        points += 2 * cost::hard_holes_in_class_timetable(self);
        points += 2 * cost::hard_too_many_subjects_of_same_kind(self);
        points += 2 * cost::hard_block_classes(self);
        points += 2 * cost::hard_specific_subject_days(self);
        points += 2 * cost::hard_subject_per_day_limits(self);
        points += 2 * cost::hard_subject_holes(self);

        points
    }

    // Should be as close to 0 as possible
    pub fn soft_points(&self) -> i32 {
        let mut points = 0;

        let teacher_table =
            util::class_table_to_teacher_table(&self.table, &self.data, self.max_periods_per_day);

        points += cost::soft_class_spread(self);
        points += cost::soft_teacher_class_spread(self, &teacher_table);
        points += cost::soft_holes_in_teacher_timetable(self, &teacher_table);
        points += cost::soft_preferred_subject_times(self);

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
