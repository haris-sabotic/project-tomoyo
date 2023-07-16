use std::{
    collections::HashMap,
    println,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    vec,
};

use project_tomoyo::*;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use ws::Sender;

use crate::{cost, util};

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
            print!("{}, {:?}", c, self.data.classes[c]);
            class_slots.class_index = c as u32;
            class_slots.slots.resize(
                5 * self.max_periods_per_day as usize,
                Slot::Single(SlotData::Empty),
            );

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

            let counts: Vec<(u32, Option<u32>)> = class_relations
                .iter()
                .map(|x| (x.per_week_first, x.per_week_second))
                .collect();
            let sum = counts.iter().fold(0, |acc, x| {
                let (x1, x2) = x;
                let mut result = acc + x1;

                match x2 {
                    Some(v) => result += v,
                    None => {}
                }

                result
            });
            println!("  {} {:?}", sum, counts);

            let mut i = 0;
            while i < class_slots.slots.len() {
                if let Some(relation) = class_relations.pop() {
                    let per_week_first = relation.per_week_first;

                    println!("Teacher: {:?} \nSubject: {:?} \nClass: {:?} \nPer week first: {:?} \nPer week second: {:?}\n", self.data.teachers[relation.teacher], self.data.subjects[relation.subject], self.data.classes[relation.class], relation.per_week_first, relation.per_week_second);

                    match relation.per_week_second {
                        // single relation
                        None => {
                            for j in 0..per_week_first {
                                class_slots.slots[i + j as usize] =
                                    Slot::Single(SlotData::PartiallyFilled {
                                        teacher: relation.teacher,
                                        subject: relation.subject,
                                    });
                            }

                            i += per_week_first as usize - 1;
                        }
                        // double relation
                        Some(per_week_second) => {
                            let mut placed = false;

                            // start searching from the start up until where you last placed a slot and try to place all of the relation's first group's classes into consecutive slots
                            for j in 0..i {
                                match class_slots.slots[j] {
                                    Slot::Single(_) => {}

                                    // if it's a double slot
                                    Slot::Double {
                                        first,
                                        second,
                                        before,
                                        after,
                                    } => {
                                        // if it's the start of per_week_first consecutive double blocks
                                        if before == 0 && after == (per_week_first - 1) {
                                            match (first, second) {
                                                // if the first slots are all empty
                                                (SlotData::Empty, _) => {
                                                    let mut different = true;
                                                    match second {
                                                        SlotData::PartiallyFilled {
                                                            teacher,
                                                            ..
                                                        } => {
                                                            if teacher == relation.teacher {
                                                                different = false;
                                                            }
                                                        }

                                                        _ => {}
                                                    }

                                                    if different {
                                                        // place all of the relations first group's classes in the empty slots
                                                        for k in 0..per_week_first {
                                                            match class_slots.slots[j + k as usize] {
                                                            Slot::Double {
                                                                second,
                                                                before,
                                                                after,
                                                                ..
                                                            } => class_slots.slots
                                                                [j + k as usize] =
                                                                Slot::Double {
                                                                    first:
                                                                        SlotData::PartiallyFilled {
                                                                            teacher: relation
                                                                                .teacher,
                                                                            subject: relation
                                                                                .subject,
                                                                        },
                                                                    second,
                                                                    before,
                                                                    after,
                                                                },

                                                            _ => {}
                                                        }
                                                        }

                                                        placed = true;
                                                        break;
                                                    }
                                                }
                                                // if the second slots are all empty
                                                (_, SlotData::Empty) => {
                                                    let mut different = true;
                                                    match first {
                                                        SlotData::PartiallyFilled {
                                                            teacher,
                                                            ..
                                                        } => {
                                                            if teacher == relation.teacher {
                                                                different = false;
                                                            }
                                                        }

                                                        _ => {}
                                                    }

                                                    if different {
                                                        // place all of the relations first group's classes in the empty slots
                                                        for k in 0..per_week_first {
                                                            match class_slots.slots[j + k as usize] {
                                                            Slot::Double {
                                                                first,
                                                                before,
                                                                after,
                                                                ..
                                                            } => class_slots.slots
                                                                [j + k as usize] =
                                                                Slot::Double {
                                                                    first,
                                                                    second:
                                                                        SlotData::PartiallyFilled {
                                                                            teacher: relation
                                                                                .teacher,
                                                                            subject: relation
                                                                                .subject,
                                                                        },
                                                                    before,
                                                                    after,
                                                                },

                                                            _ => {}
                                                        }
                                                        }

                                                        placed = true;
                                                        break;
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }

                            if !placed {
                                for j in 0..per_week_first {
                                    class_slots.slots[i + j as usize] = Slot::Double {
                                        first: SlotData::PartiallyFilled {
                                            teacher: relation.teacher,
                                            subject: relation.subject,
                                        },
                                        second: SlotData::Empty,
                                        before: j,
                                        after: per_week_first - j - 1,
                                    };
                                }

                                i += per_week_first as usize - 1;
                            }

                            // SAME THING EXCEPT FOR THE SECOND GROUP
                            // ======================================
                            if per_week_second != 0 {
                                placed = false;

                                for j in 0..i {
                                    match class_slots.slots[j] {
                                        Slot::Single(_) => {}

                                        // if it's a double slot
                                        Slot::Double {
                                            first,
                                            second,
                                            before,
                                            after,
                                        } => {
                                            // if it's the start of per_week_second consecutive double blocks
                                            if before == 0 && after == (per_week_second - 1) {
                                                match (first, second) {
                                                    // if the first slots are all empty
                                                    (SlotData::Empty, _) => {
                                                        let mut different = true;
                                                        match second {
                                                            SlotData::PartiallyFilled {
                                                                teacher,
                                                                ..
                                                            } => {
                                                                if teacher == relation.teacher {
                                                                    different = false;
                                                                }
                                                            }

                                                            _ => {}
                                                        }

                                                        if different {
                                                            // place all of the relations second group's classes in the empty slots
                                                            for k in 0..per_week_second {
                                                                match class_slots.slots[j + k as usize] {
                                                            Slot::Double {
                                                                second,
                                                                before,
                                                                after,
                                                                ..
                                                            } => class_slots.slots
                                                                [j + k as usize] =
                                                                Slot::Double {
                                                                    first:
                                                                        SlotData::PartiallyFilled {
                                                                            teacher: relation
                                                                                .teacher,
                                                                            subject: relation
                                                                                .subject,
                                                                        },
                                                                    second,
                                                                    before,
                                                                    after,
                                                                },

                                                            _ => {}
                                                        }
                                                            }

                                                            placed = true;
                                                            break;
                                                        }
                                                    }
                                                    // if the second slots are all empty
                                                    (_, SlotData::Empty) => {
                                                        let mut different = true;
                                                        match first {
                                                            SlotData::PartiallyFilled {
                                                                teacher,
                                                                ..
                                                            } => {
                                                                if teacher == relation.teacher {
                                                                    different = false;
                                                                }
                                                            }

                                                            _ => {}
                                                        }

                                                        if different {
                                                            // place all of the relations second group's classes in the empty slots
                                                            for k in 0..per_week_second {
                                                                match class_slots.slots[j + k as usize] {
                                                            Slot::Double {
                                                                first,
                                                                before,
                                                                after,
                                                                ..
                                                            } => class_slots.slots
                                                                [j + k as usize] =
                                                                Slot::Double {
                                                                    first,
                                                                    second:
                                                                        SlotData::PartiallyFilled {
                                                                            teacher: relation
                                                                                .teacher,
                                                                            subject: relation
                                                                                .subject,
                                                                        },
                                                                    before,
                                                                    after,
                                                                },

                                                            _ => {}
                                                        }
                                                            }

                                                            placed = true;
                                                            break;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                }

                                if !placed {
                                    for j in 0..per_week_second {
                                        class_slots.slots[i + j as usize] = Slot::Double {
                                            first: SlotData::PartiallyFilled {
                                                teacher: relation.teacher,
                                                subject: relation.subject,
                                            },
                                            second: SlotData::Empty,
                                            before: j,
                                            after: per_week_second - j - 1,
                                        }
                                    }

                                    i += per_week_second as usize - 1;
                                }
                            }
                        }
                    }
                }

                i += 1;
            }

            c += 1;
        }

        self.table = table;
    }

    pub fn start_algorithm(&mut self, running: Arc<AtomicBool>, out: &Sender) {
        let room_kinds_count = util::room_kinds_count(&self.data.rooms);

        // SIMULATED ANNEALLING:
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
                    let new_s = s.generate_neighbor(out);

                    let new_s_cost_hard = new_s.hard_points(&room_kinds_count);
                    let s_cost_hard = s.hard_points(&room_kinds_count);

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

                        let best_s_cost_hard = best_s.hard_points(&room_kinds_count);
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

                t = t * ALPHA;
            }

            self.table = best_s.table;
        }

        let teacher_table =
            util::class_table_to_teacher_table(&self.table, &self.data, self.max_periods_per_day);

        let cost1 = 2 * cost::hard_repeating_teachers(self);
        let cost2 = 2 * cost::hard_holes_in_class_timetable(self);
        let cost3 = 2 * cost::hard_too_many_subjects_of_same_kind(self, &room_kinds_count);
        let cost4 = cost::soft_class_spread(self);
        let cost5 = cost::soft_teacher_class_spread(self, &teacher_table);
        let cost6 = cost::soft_holes_in_teacher_timetable(self, &teacher_table);

        println!("DETAILED COST");
        println!(" (h) Repeating teachers: {}", cost1);
        println!(" (h) Holes: {}", cost2);
        println!(" (h) Too many subjects of same kind: {}", cost3);
        println!(" (s) Class spread: {}", cost4);
        println!(" (s) Teacher class spread: {}", cost5);
        println!(" (s) Teacher holes: {}", cost6);
    }

    pub fn generate_neighbor(&self, _out: &Sender) -> Self {
        let mut rng = thread_rng();

        let mut timetable = self.clone();

        let class_index: usize = rng.gen_range(0..self.data.classes.len());

        let mut start_index = rng.gen_range(0..timetable.table[class_index].slots.len());

        match timetable.table[class_index].slots[start_index] {
            Slot::Single(_) => {
                let mut end_index = rng.gen_range(0..timetable.table[class_index].slots.len());

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
                // set start index to the start of this double block
                start_index -= before_start as usize;
                let length = before_start + after_start + 1;

                let mut done = false;

                while !done {
                    // choose a random day
                    let day = rng.gen_range(0..5);

                    let index = day * timetable.max_periods_per_day
                        + rng.gen_range(0..(timetable.max_periods_per_day - length));

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
                            if before_end == 0 && after_end == length - 1 {
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
                    }
                }
            }
        }

        timetable
    }

    // Should be 0
    pub fn hard_points(&self, room_kinds_count: &HashMap<String, u32>) -> i32 {
        let mut points = 0;

        points += 2 * cost::hard_repeating_teachers(self);
        points += 2 * cost::hard_holes_in_class_timetable(self);
        points += 2 * cost::hard_too_many_subjects_of_same_kind(self, room_kinds_count);

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

        points
    }

    pub fn fill_rooms(&mut self) {
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
    }
}
