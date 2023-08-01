use std::{collections::HashMap, println, vec};

use crate::{
    logic::{Slot, SlotData, Timetable},
    util::{TeacherSlot, TeacherSlots},
};

/* ==================== */
/*   HARD CONSTRAINTS   */
/* ==================== */

/// Increment points by 1 for each teacher teaching multiple classes in the same period
pub fn hard_repeating_teachers(timetable: &Timetable) -> i32 {
    let mut points = 0;

    for period in 0..(timetable.max_periods_per_day * 5) {
        let mut seen_teachers: Vec<usize> = vec![];

        for class in 0..timetable.table.len() {
            match timetable.table[class].slots[period as usize] {
                Slot::Single(s) => match s {
                    SlotData::PartiallyFilled { teacher, .. } => {
                        if seen_teachers.contains(&teacher) {
                            points += 1;
                        } else {
                            seen_teachers.push(teacher);
                        }
                    }

                    SlotData::Filled { teacher, .. } => {
                        if seen_teachers.contains(&teacher) {
                            points += 1;
                        } else {
                            seen_teachers.push(teacher);
                        }
                    }

                    _ => {}
                },
                Slot::Double { first, second, .. } => {
                    match first {
                        SlotData::PartiallyFilled { teacher, .. } => {
                            if seen_teachers.contains(&teacher) {
                                points += 1;
                            } else {
                                seen_teachers.push(teacher);
                            }
                        }

                        SlotData::Filled { teacher, .. } => {
                            if seen_teachers.contains(&teacher) {
                                points += 1;
                            } else {
                                seen_teachers.push(teacher);
                            }
                        }

                        _ => {}
                    }

                    match second {
                        SlotData::PartiallyFilled { teacher, .. } => {
                            if seen_teachers.contains(&teacher) {
                                points += 1;
                            } else {
                                seen_teachers.push(teacher);
                            }
                        }

                        SlotData::Filled { teacher, .. } => {
                            if seen_teachers.contains(&teacher) {
                                points += 1;
                            } else {
                                seen_teachers.push(teacher);
                            }
                        }

                        _ => {}
                    }
                }
            }
        }
    }

    points
}

/// Increment points by 1 for every hole in a class timetable
pub fn hard_holes_in_class_timetable(timetable: &Timetable) -> i32 {
    let mut points: i32 = 0;

    for class_slots in timetable.table.iter() {
        for day in 0..5 {
            let mut empty_slots = 0;
            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match class_slots.slots[index as usize] {
                    Slot::Single(SlotData::Empty)
                    | Slot::Double {
                        first: SlotData::Empty,
                        second: SlotData::PartiallyFilled { .. } | SlotData::Filled { .. },
                        ..
                    }
                    | Slot::Double {
                        first: SlotData::PartiallyFilled { .. } | SlotData::Filled { .. },
                        second: SlotData::Empty,
                        ..
                    } => empty_slots += 1,

                    _ => {}
                }
            }

            let mut start = 0;
            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match class_slots.slots[index as usize] {
                    Slot::Single(_) => break,

                    Slot::Double {
                        first: SlotData::PartiallyFilled { .. } | SlotData::Filled { .. },
                        second: SlotData::Empty,
                        ..
                    } => empty_slots -= 1,
                    Slot::Double { .. } => break,
                }

                start += 1;
            }

            let mut one_group_left = false;
            for period in ((start + 1)..timetable.max_periods_per_day).rev() {
                let index = day * timetable.max_periods_per_day + period;

                match class_slots.slots[index as usize] {
                    Slot::Single(SlotData::Empty) => {
                        if one_group_left {
                            break;
                        }
                        empty_slots -= 1;
                    }
                    Slot::Single(_) => break,

                    Slot::Double {
                        first: SlotData::PartiallyFilled { .. } | SlotData::Filled { .. },
                        second: SlotData::Empty,
                        ..
                    } => {
                        empty_slots -= 1;
                        one_group_left = true;
                    }
                    Slot::Double { .. } => break,
                }
            }

            points += empty_slots;
        }
    }

    points
}

/// Increment points by 1 for each period during which too many classes are being held with the same kind
///
/// Example: During the 2nd period of Tuesday, 5 IT classes are being held, when there's only 3 computer classrooms in the school
pub fn hard_too_many_subjects_of_same_kind(timetable: &Timetable) -> i32 {
    let mut points = 0;

    for period in 0..(timetable.max_periods_per_day * 5) {
        let mut subject_kinds_count: HashMap<String, u32> = HashMap::new();

        for class_slots in timetable.table.iter() {
            match class_slots.slots[period as usize] {
                Slot::Single(s) => match s {
                    SlotData::PartiallyFilled { subject, .. } => {
                        subject_kinds_count
                            .entry(timetable.data.subjects[subject].kind.clone())
                            .and_modify(|c| *c += 1)
                            .or_insert(1);
                    }

                    SlotData::Filled { subject, .. } => {
                        subject_kinds_count
                            .entry(timetable.data.subjects[subject].kind.clone())
                            .and_modify(|c| *c += 1)
                            .or_insert(1);
                    }

                    _ => {}
                },
                Slot::Double { first, second, .. } => {
                    match first {
                        SlotData::PartiallyFilled { subject, .. } => {
                            subject_kinds_count
                                .entry(timetable.data.subjects[subject].kind.clone())
                                .and_modify(|c| *c += 1)
                                .or_insert(1);
                        }

                        SlotData::Filled { subject, .. } => {
                            subject_kinds_count
                                .entry(timetable.data.subjects[subject].kind.clone())
                                .and_modify(|c| *c += 1)
                                .or_insert(1);
                        }

                        _ => {}
                    }

                    match second {
                        SlotData::PartiallyFilled { subject, .. } => {
                            subject_kinds_count
                                .entry(timetable.data.subjects[subject].kind.clone())
                                .and_modify(|c| *c += 1)
                                .or_insert(1);
                        }

                        SlotData::Filled { subject, .. } => {
                            subject_kinds_count
                                .entry(timetable.data.subjects[subject].kind.clone())
                                .and_modify(|c| *c += 1)
                                .or_insert(1);
                        }

                        _ => {}
                    }
                }
            }
        }

        // println!("{:?}", subject_kinds_count);
        let mut used_rooms: Vec<usize> = vec![];
        for kind in [
            "sala", "vjezbe", "masinska", "computer", "laptop", "regular",
        ] {
            let mut found_rooms = false;
            match subject_kinds_count.get(kind) {
                Some(n) => {
                    for _ in 0..(*n) {
                        found_rooms = false;

                        for i in 0..timetable.data.rooms.len() {
                            if !used_rooms.contains(&i)
                                && timetable.data.rooms[i].kinds.contains(&kind.to_string())
                            {
                                used_rooms.push(i);
                                found_rooms = true;
                                break;
                            }
                        }
                    }

                    if !found_rooms {
                        // println!("NOT FOUND: {}", kind);
                        points += 1;
                    }
                }
                None => {}
            }
        }
    }

    points
}

pub fn hard_block_classes(timetable: &Timetable) -> i32 {
    let mut points: i32 = 0;

    let subjects = vec![67, 66, 64, 58];

    for class_slots in timetable.table.iter() {
        for subject in subjects.iter() {
            let mut block_found = false;
            let mut subject_exists = false;

            for day in 0..5 {
                if block_found {
                    break;
                }

                for period in 0..timetable.max_periods_per_day {
                    let index = day * timetable.max_periods_per_day + period;

                    match class_slots.slots[index as usize] {
                        Slot::Single(SlotData::PartiallyFilled {
                            subject: subject_out,
                            ..
                        }) => {
                            if subject_out == *subject {
                                subject_exists = true;

                                if period < timetable.max_periods_per_day - 1 {
                                    match class_slots.slots[index as usize + 1] {
                                        Slot::Single(SlotData::PartiallyFilled {
                                            subject: subject_in,
                                            ..
                                        }) => {
                                            if subject_in == *subject {
                                                block_found = true;
                                                break;
                                            }
                                        }

                                        _ => {}
                                    }
                                }
                            }
                        }

                        _ => {}
                    }
                }
            }

            if subject_exists && !block_found {
                points += 1;
            }
        }
    }

    points
}

pub fn hard_specific_subject_days(timetable: &Timetable) -> i32 {
    let mut points = 0;

    let subjects = vec![67, 66, 65, 64, 63, 62, 61, 60, 59, 58];
    let classes = vec![7, 6, 5, 15, 14, 13];

    for class in classes.iter() {
        let mut days_without_subjects = 0;

        for day in 0..5 {
            days_without_subjects += 1;

            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match timetable.table[*class as usize].slots[index as usize] {
                    Slot::Single(s) => match s {
                        SlotData::PartiallyFilled { subject, .. } => {
                            if subjects.contains(&subject) {
                                days_without_subjects -= 1;
                                break;
                            }
                        }

                        _ => {}
                    },

                    _ => {}
                }
            }
        }

        if days_without_subjects == 0 {
            points += 1;
        }
    }

    points
}

pub fn hard_subject_per_day_limits(timetable: &Timetable) -> i32 {
    let mut points = 0;

    let subjects2 = vec![67, 66, 64, 58];
    let subjects1 = vec![59];

    for class_slots in timetable.table.iter() {
        for day in 0..5 {
            let mut subject_counts: HashMap<usize, i32> = HashMap::new();

            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match class_slots.slots[index as usize] {
                    Slot::Single(SlotData::PartiallyFilled { subject, .. }) => {
                        subject_counts
                            .entry(subject)
                            .and_modify(|c| *c += 1)
                            .or_insert(1);
                    }

                    _ => {}
                }
            }

            for subject in subject_counts.keys() {
                if subjects1.contains(subject) {
                    if subject_counts[subject] > 1 {
                        points += 1;
                    }
                }

                if subjects2.contains(subject) {
                    if subject_counts[subject] > 2 {
                        points += 1;
                    }
                }
            }
        }
    }

    points
}

pub fn hard_subject_holes(timetable: &Timetable) -> i32 {
    let mut points = 0;

    for class_slots in timetable.table.iter() {
        for day in 0..5 {
            let mut last_subject: i32 = -1;
            let mut seen_subjects: Vec<usize> = vec![];

            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match class_slots.slots[index as usize] {
                    Slot::Single(s) => match s {
                        SlotData::PartiallyFilled { subject, .. } => {
                            if seen_subjects.contains(&subject) {
                                if subject != last_subject as usize {
                                    points += 1;
                                }
                            }

                            seen_subjects.push(subject);
                            last_subject = subject as i32;
                        }

                        _ => {}
                    },

                    Slot::Double { .. } => {}
                }
            }
        }
    }

    points
}

/* ==================== */
/*   SOFT CONSTRAINTS   */
/* ==================== */

/// Increment points by 1 for each day in a class timetable that contains more periods than what's ideal (even spread)
pub fn soft_class_spread(timetable: &Timetable) -> i32 {
    let mut points = 0;

    for class_slots in timetable.table.iter() {
        // total number of classes in a week
        let mut class_count = 0;
        for slot in class_slots.slots.iter() {
            match slot {
                Slot::Single(s) => match s {
                    SlotData::Empty => {}
                    _ => class_count += 1,
                },
                Slot::Double { first, second, .. } => {
                    let mut empty = true;

                    match first {
                        SlotData::Empty => {}
                        _ => empty = false,
                    }
                    match second {
                        SlotData::Empty => {}
                        _ => empty = false,
                    }

                    if !empty {
                        class_count += 1;
                    }
                }
            }
        }

        // don't do anything if the class timetable is empty
        if class_count == 0 {
            continue;
        }

        // ideal number of classes per day
        let ideal_class_spread = (class_count as f32 / 5.0).floor() as i32;

        for day in 0..5 {
            // calculate number of classes during this day for each group
            let mut day_class_count_first_group = 0;
            let mut day_class_count_second_group = 0;

            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match class_slots.slots[index as usize] {
                    Slot::Single(s) => match s {
                        SlotData::Empty => {}
                        _ => {
                            day_class_count_first_group += 1;
                            day_class_count_second_group += 1;
                        }
                    },
                    Slot::Double { first, second, .. } => {
                        match first {
                            SlotData::Empty => {}
                            _ => day_class_count_first_group += 1,
                        }
                        match second {
                            SlotData::Empty => {}
                            _ => day_class_count_second_group += 1,
                        }
                    }
                }
            }

            if day_class_count_first_group == day_class_count_second_group {
                if day_class_count_first_group < ideal_class_spread
                    || day_class_count_second_group < ideal_class_spread
                {
                    points += 1;
                }
            } else {
                if day_class_count_first_group < ideal_class_spread {
                    points += 1;
                }

                if day_class_count_second_group < ideal_class_spread {
                    points += 1;
                }
            }
        }
    }

    points
}

/// Same as `soft_class_spread` except for teachers instead of classes
pub fn soft_teacher_class_spread(timetable: &Timetable, teacher_table: &Vec<TeacherSlots>) -> i32 {
    let mut points = 0;

    for slots in teacher_table.iter() {
        let mut count = 0;
        for slot in slots.slots.iter() {
            match slot {
                TeacherSlot::Empty => {}

                _ => count += 1,
            }
        }

        // don't do anything if the teacher timetable is empty
        if count == 0 {
            continue;
        }

        // ideal number of classes per day
        // let ideal_spread = (count as f32 / 5.0).ceil() as i32;

        for day in 0..5 {
            // calculate number of classes during this day
            let mut day_class_count = 0;

            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match slots.slots[index as usize] {
                    TeacherSlot::Empty => {}
                    _ => {
                        day_class_count += 1;
                    }
                }
            }

            /*
            if day_class_count > ideal_spread {
                points += 1;
            }

            if day_class_count == 1 {
                points += 1;
            }
            */

            if day_class_count >= 6 || day_class_count == 1 {
                points += 1;
            }
        }
    }

    points
}

/// Same as `hard_holes_in_class_timetable` except for teachers instead of classes
pub fn soft_holes_in_teacher_timetable(
    timetable: &Timetable,
    teacher_table: &Vec<TeacherSlots>,
) -> i32 {
    let mut points = 0;

    for slots in teacher_table.iter() {
        for day in 0..5 {
            let mut start = 0;
            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match slots.slots[index as usize] {
                    TeacherSlot::Empty => {}
                    _ => {
                        start = period;
                        break;
                    }
                }
            }

            let mut end = 0;
            for period in (0..timetable.max_periods_per_day).rev() {
                let index = day * timetable.max_periods_per_day + period;

                match slots.slots[index as usize] {
                    TeacherSlot::Empty => {}
                    _ => {
                        end = period;
                        break;
                    }
                }
            }

            let mut holes = 0;
            for period in start..end {
                let index = day * timetable.max_periods_per_day + period;

                match slots.slots[index as usize] {
                    TeacherSlot::Empty => holes += 1,
                    _ => {}
                }
            }

            if holes >= 1 {
                points += 1;
            }
        }
    }

    points
}

pub fn soft_preferred_subject_times(timetable: &Timetable) -> i32 {
    let mut points = 0;

    for class_slots in timetable.table.iter() {
        for day in 0..5 {
            // math shouldn't be in the second half of a day
            let start = (timetable.max_periods_per_day as f32 / 2 as f32).ceil() as u32 - 1;
            for period in start..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match class_slots.slots[index as usize] {
                    Slot::Single(s) => match s {
                        SlotData::PartiallyFilled { subject, .. } => {
                            if subject == 67 {
                                points += 1;
                            }
                        }

                        _ => {}
                    },

                    _ => {}
                }
            }

            // P.E. shouldn't be at the end of a day
            let period = timetable.max_periods_per_day as usize - 1;
            match class_slots.slots
                [day as usize * timetable.max_periods_per_day as usize + period as usize]
            {
                Slot::Single(s) => match s {
                    SlotData::PartiallyFilled { subject, .. } => {
                        if subject == 59 {
                            points += 1;
                        }
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }

    points
}
