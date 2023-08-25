use std::{collections::HashMap, print, println, vec};

use crate::{
    logic::{ClassSlots, Shift, Slot, SlotData, Timetable, TimetableData},
    util::{TeacherSlot, TeacherSlots},
};

/* ==================== */
/*   HARD CONSTRAINTS   */
/* ==================== */

pub fn repeating_rooms(timetable: &Timetable, shift: Shift, debug: bool) -> i32 {
    let mut points = 0;

    for period in 0..(timetable.max_periods_per_day * 5) {
        let mut seen_rooms: Vec<usize> = vec![];

        for class in 0..timetable.table(shift).len() {
            match timetable.table(shift)[class].slots[period as usize] {
                Slot::Single(s) => match s {
                    SlotData::PartiallyFilled { room, .. } => match room {
                        Some(r) => {
                            if seen_rooms.contains(&r) {
                                points += 1;

                                if debug {
                                    println!("  {}", timetable.data.rooms[r].name);
                                }
                            } else {
                                seen_rooms.push(r);
                            }
                        }
                        None => {}
                    },

                    _ => {}
                },
                Slot::Double { first, second, .. } => {
                    match first {
                        SlotData::PartiallyFilled { room, .. } => match room {
                            Some(r) => {
                                if seen_rooms.contains(&r) {
                                    points += 1;

                                    if debug {
                                        println!("  {}", timetable.data.rooms[r].name);
                                    }
                                } else {
                                    seen_rooms.push(r);
                                }
                            }
                            None => {}
                        },

                        _ => {}
                    }

                    match second {
                        SlotData::PartiallyFilled { room, .. } => match room {
                            Some(r) => {
                                if seen_rooms.contains(&r) {
                                    points += 1;

                                    if debug {
                                        println!("  {}", timetable.data.rooms[r].name);
                                    }
                                } else {
                                    seen_rooms.push(r);
                                }
                            }
                            None => {}
                        },

                        _ => {}
                    }
                }
            }
        }
    }

    points
}

/// Increment points by 1 for each teacher teaching multiple classes in the same period
pub fn hard_repeating_teachers(timetable: &Timetable, shift: Shift, debug: bool) -> i32 {
    let mut points = 0;

    for period in 0..(timetable.max_periods_per_day * 5) {
        let mut seen_teachers: Vec<usize> = vec![];

        for class in 0..timetable.table(shift).len() {
            match timetable.table(shift)[class].slots[period as usize] {
                Slot::Single(s) => match s {
                    SlotData::PartiallyFilled { teacher, .. } => {
                        if seen_teachers.contains(&teacher) {
                            points += 1;

                            if debug {
                                println!("  {}", timetable.data.teachers[teacher].name);
                            }
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

                                if debug {
                                    println!("  {}", timetable.data.teachers[teacher].name);
                                }
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

                                if debug {
                                    println!("  {}", timetable.data.teachers[teacher].name);
                                }
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
pub fn hard_holes_in_class_timetable(timetable: &Timetable, shift: Shift) -> i32 {
    let mut points: i32 = 0;

    for class_slots in timetable.table(shift).iter() {
        for day in 0..5 {
            let mut empty_slots_single = 0;
            let mut empty_slots = 0;
            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match class_slots.slots[index as usize] {
                    Slot::Single(SlotData::Empty) => {
                        empty_slots += 1;
                        empty_slots_single += 1;
                    }

                    Slot::Double {
                        first: SlotData::Empty,
                        second: SlotData::PartiallyFilled { .. },
                        ..
                    }
                    | Slot::Double {
                        first: SlotData::PartiallyFilled { .. },
                        second: SlotData::Empty,
                        ..
                    } => empty_slots += 1,

                    _ => {}
                }
            }

            // just give up right away if the day was entirely empty
            if empty_slots_single == timetable.max_periods_per_day {
                continue;
            }

            let mut start = 0;
            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match class_slots.slots[index as usize] {
                    Slot::Single(_) => break,

                    Slot::Double {
                        first: SlotData::PartiallyFilled { .. },
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
                        first: SlotData::PartiallyFilled { .. },
                        second: SlotData::Empty,
                        ..
                    } => {
                        empty_slots -= 1;
                        one_group_left = true;
                    }
                    Slot::Double { .. } => break,
                }
            }

            // multiply by 2 for higher priority
            points += empty_slots;
        }
    }

    points
}

/// Increment points by 1 for each period during which too many classes are being held with the same kind
///
/// Example: During the 2nd period of Tuesday, 5 IT classes are being held, when there's only 3 computer classrooms in the school
pub fn hard_too_many_subjects_of_same_kind(
    timetable: &Timetable,
    shift: Shift,
    debug: bool,
) -> i32 {
    let mut points = 0;

    for period in 0..(timetable.max_periods_per_day * 5) {
        let mut subject_kinds_count: HashMap<String, u32> = HashMap::new();

        for class_slots in timetable.table(shift).iter() {
            match class_slots.slots[period as usize] {
                Slot::Single(s) => match s {
                    SlotData::PartiallyFilled { subject, .. } => {
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

                        _ => {}
                    }

                    match second {
                        SlotData::PartiallyFilled { subject, .. } => {
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

        /*
        let mut kinds: Vec<(&String, &u32)> = subject_kinds_count.iter().collect();
        kinds.sort_by(|a, b| a.1.cmp(b.1));
        let mut used_rooms: Vec<usize> = vec![];
        for (kind, count) in kinds {
            let mut found_rooms = true;
            for _ in 0..(*count) {
                let mut found_room = false;
                for i in 0..timetable.data.rooms.len() {
                    if !used_rooms.contains(&i)
                        && timetable.data.rooms[i].kinds.contains(&kind.to_string())
                    {
                        used_rooms.push(i);
                        found_room = true;
                        break;
                    }
                }

                if !found_room {
                    found_rooms = false;
                    break;
                }
            }

            if !found_rooms {
                points += 1;
            }
        }
        */

        let mut used_rooms: Vec<usize> = vec![];
        for kind in [
            "masinska",
            "14",
            "sd",
            "sala",
            "regular",
            "computer",
            "computer-regular",
            "14-23",
            "masinska-14",
            "masinska-sd",
            "masinska-computer",
            "masinska-regular",
            "masinska-regular-sd",
        ] {
            let mut found_rooms = false;
            match subject_kinds_count.get(kind) {
                Some(n) => {
                    for _ in 0..(*n) {
                        found_rooms = false;

                        //let mut found_room = false;
                        for i in 0..timetable.data.rooms.len() {
                            if !used_rooms.contains(&i)
                                && timetable.data.rooms[i].kinds.contains(&kind.to_string())
                            {
                                used_rooms.push(i);
                                //found_room = true;
                                found_rooms = true;
                                break;
                            }
                        }

                        /*
                        if !found_room {
                            points += 1;
                        }
                        */
                    }

                    if !found_rooms {
                        points += 1;

                        if debug {
                            println!("  {}: {}", period, kind);
                        }
                    }
                }
                None => {}
            }
        }
    }

    points
}

pub fn hard_block_classes(timetable: &Timetable, shift: Shift) -> i32 {
    let mut points: i32 = 0;
    /*

    let subjects = vec![67, 66, 64, 58];

    for class_slots in timetable.table(shift).iter() {
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

    */
    points
}

pub fn hard_specific_subject_days(timetable: &Timetable, shift: Shift) -> i32 {
    let mut points = 0;
    /*

    let subjects = vec![67, 66, 65, 64, 63, 62, 61, 60, 59, 58];
    let classes = vec![7, 6, 5, 15, 14, 13];

    for class in classes.iter() {
        let mut days_without_subjects = 0;

        for day in 0..5 {
            days_without_subjects += 1;

            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match timetable.table(shift)[*class as usize].slots[index as usize] {
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

    */
    points
}

pub fn hard_subject_per_day_limits(timetable: &Timetable, shift: Shift) -> i32 {
    let mut points = 0;

    let subjects2 = vec![67, 66, 64, 58];
    // let subjects1 = vec![59];

    for class_slots in timetable.table(shift).iter() {
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
                /*
                if subjects1.contains(subject) {
                    if subject_counts[subject] > 1 {
                        points += 1;
                    }
                }
                */

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

pub fn hard_subject_holes(timetable: &Timetable, shift: Shift, debug: bool) -> i32 {
    let mut points = 0;

    for class_slots in timetable.table(shift).iter() {
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

                                    if debug {
                                        println!(
                                            "  {}: {}",
                                            timetable.data.classes
                                                [class_slots.class_index as usize]
                                                .name,
                                            timetable.data.subjects[subject].name
                                        );
                                    }
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

pub fn hard_teacher_shift_spread(timetable: &Timetable, shift: Shift, debug: bool) -> i32 {
    let mut points = 0;
    /*
    let teacher_spread: HashMap<&str, [i32; 5]> = HashMap::from([
        ("Stanisic Milanka", [1, 1, 2, 2, 2]),
        ("Knezevic Svetlana", [2, 2, 2, 1, 1]),
        ("Scepanovic Suzana", [2, 2, 1, 1, 1]),
        ("Becirovic Emsada", [1, 1, 1, 2, 2]),
        ("Sukovic Biljana", [1, 1, 1, 2, 2]),
        ("Scekic Jelena", [2, 1, 1, 1, 1]),
        //
        ("Mandic Olivera", [2, 2, 2, 3, 1]),
        ("Ivanovic Olivera", [2, 2, 1, 1, 1]),
        //
        ("Papic Spasoje", [2, 1, 1, 2, 2]),
        ("Zezelj Marija", [1, 2, 2, 2, 1]),
        //
        ("Sanja Radusinovic", [1, 2, 2, 1, 1]),
        ("Jelena Bogicevic", [1, 1, 1, 2, 2]),
        ("Ana Markovic", [2, 1, 1, 1, 2]),
        ("Engleski 1", [1, 2, 1, 3, 1]),
        //
        ("Aleksandra Budrak", [2, 1, 2, 2, 2]),
        ("Rada Mugosa", [2, 2, 2, 2, 1]),
        ("Sociologija 2", [1, 1, 2, 2, 1]),
        //
        ("Cimbaljevic Drago", [2, 1, 1, 2, 2]),
        ("Djeric Bogdan", [2, 2, 2, 1, 2]),
        ("Fizicko 1", [2, 2, 2, 1, 1]),
        ("Fizicko 2", [2, 2, 1, 1, 1]),
        //
        ("Zana Krgusic", [1, 1, 2, 2, 1]),
        ("Dejan Maras", [2, 1, 1, 2, 1]),
        ("Nevenka Roganovic", [1, 1, 2, 2, 2]),
        ("Marija Babovic", [2, 1, 1, 2, 1]),
        ("Natasa Stojanovic", [1, 1, 1, 2, 2]),
        //
        ("Samardzic Rada", [1, 2, 2, 1, 1]),
        ("Vratnica Mladen", [1, 1, 2, 2, 2]),
        //("Pekovic Milijana", [1, 2, 1, 2, 2]),
        ("Zekovic Jelena", [1, 1, 2, 2, 2]),
        ("Vojinovic Nikolija", [2, 2, 2, 1, 1]),
        ("Dasic Nada", [1, 2, 2, 1, 2]),
        ("Calasan Vesna", [1, 2, 2, 1, 1]),
        ("Kocovic Mitra", [2, 1, 2, 1, 1]),
        ("Energetika 1", [2, 2, 1, 1, 1]),
        ("Energetika 2", [2, 1, 1, 1, 1]),
        //
        ("Djakovic Persa", [1, 1, 1, 1, 2]),
        ("Matovic Dubravka", [2, 2, 1, 1, 2]),
        ("Coguric Radmila", [1, 2, 2, 2, 1]),
        ("Tasic Gordana", [2, 2, 1, 1, 2]),
        ("Milentijevic Dragica", [1, 1, 2, 1, 1]),
        ("Tadic Slobodan", [2, 2, 1, 1, 1]),
        ("Nikolic Natalija", [2, 2, 2, 1, 2]),
        ("Ana Vujovic", [2, 1, 1, 1, 2]),
        ("Vemic Nada", [1, 2, 1, 2, 1]),
        ("Radulovic Zoran", [2, 2, 2, 1, 1]),
        ("Lucic Mileva", [2, 1, 2, 1, 1]),
        ("Raskovic Violeta", [1, 2, 2, 1, 2]),
        ("Kojovic Nikola", [2, 1, 2, 1, 2]),
        ("Obradovic Aleksandar", [1, 2, 2, 1, 1]),
        ("Stevovic Mirjana", [1, 2, 1, 2, 1]),
        //
        //("Lidija Vuletic", [1, 2, 2, 1, 2]),
        //("Kovac Vladimir", [1, 2, 1, 2, 1]),
        //("Babic Jelena", [1, 2, 1, 2, 2]),
        //("Marina Radonjic", [2, 2, 1, 2, 2]),
        //("Sladjana Saric", [1, 1, 2, 1, 1]),
        //("Veselin Picuric", [2, 1, 2, 2, 1]),
        //("Alen Adilovic", [1, 1, 1, 2, 2]),
        //("Snezana Krunic", [2, 2, 1, 1, 2]),
        //("Lopicic PRedrag", [1, 2, 1, 2, 1]),
        //("Elektronika 1", [2, 2, 1, 1, 1]),
        //("Elektronika 4", [2, 2, 2, 2, 1]),
        //("Elektronika 6", [1, 1, 1, 2, 2]),
        //("Elektronika 8", [2, 1, 1, 1, 2]),
        //("Elektronika 9", [2, 2, 2, 2, 1]),
    ]);

    for day in 0..5 {
        let mut seen_teachers: Vec<usize> = vec![];

        for class_slots in timetable.table(shift).iter() {
            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match class_slots.slots[index as usize] {
                    Slot::Single(s) => match s {
                        SlotData::PartiallyFilled { teacher, .. } => {
                            seen_teachers.push(teacher);
                        }

                        _ => {}
                    },
                    Slot::Double { first, second, .. } => {
                        match first {
                            SlotData::PartiallyFilled { teacher, .. } => {
                                seen_teachers.push(teacher);
                            }

                            _ => {}
                        }

                        match second {
                            SlotData::PartiallyFilled { teacher, .. } => {
                                seen_teachers.push(teacher);
                            }

                            _ => {}
                        }
                    }
                }
            }
        }

        for teacher in seen_teachers {
            let name = timetable.data.teachers[teacher].name.as_str();

            if teacher_spread.contains_key(&name) {
                let s = teacher_spread[&name][day as usize];
                if s != shift.to_i32() && s != 3 {
                    points += 1;

                    if debug {
                        let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
                        println!("  {}, {}", name, days[day as usize]);
                    }
                }
            }
        }
    }
    */
    points
}

pub fn hard_teacher_extra_constraints(timetable: &Timetable, shift: Shift) -> i32 {
    let mut points = 0;

    // 0 1 2 3 4 5 6    7 8 9 10 11 12 13    14 15 16 17 18 19 20    21 22 23 24 25 26 27    28 29 30 31 32 33 34
    let blacklist: HashMap<usize, Vec<u32>> = HashMap::from([
        (
            59, // Svetlana Miranovic
            vec![0, 1, 7, 8, 14, 15, 21, 23, 28, 29],
        ),
        (79, vec![5, 6, 12, 13, 19, 20, 26, 27]), // Lidija Lazarevic
        (61, vec![5, 6, 19, 20, 33, 34]),         // Selman Sabotic
        (33, vec![28, 29, 30, 31, 32, 33, 34]),   // Tadic Slobodan
    ]);

    for class_slots in timetable.table(shift).iter() {
        for period in 0..(timetable.max_periods_per_day * 5) {
            match class_slots.slots[period as usize] {
                Slot::Single(s) => match s {
                    SlotData::PartiallyFilled { teacher, .. } => {
                        if blacklist.contains_key(&teacher) {
                            if blacklist[&teacher].contains(&period) {
                                points += 1;
                            }
                        }
                    }

                    _ => {}
                },
                Slot::Double { first, second, .. } => {
                    match first {
                        SlotData::PartiallyFilled { teacher, .. } => {
                            if blacklist.contains_key(&teacher) {
                                if blacklist[&teacher].contains(&period) {
                                    points += 1;
                                }
                            }
                        }

                        _ => {}
                    }

                    match second {
                        SlotData::PartiallyFilled { teacher, .. } => {
                            if blacklist.contains_key(&teacher) {
                                if blacklist[&teacher].contains(&period) {
                                    points += 1;
                                }
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

/* ==================== */
/*   SOFT CONSTRAINTS   */
/* ==================== */

/// Increment points by 1 for each day in a class timetable that contains more periods than what's ideal (even spread)
pub fn soft_class_spread(timetable: &Timetable, shift: Shift) -> i32 {
    let mut points = 0;

    /*

    for class_slots in timetable.table(shift).iter() {
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
        // let ideal_class_spread = (class_count as f32 / 5.0).floor() as i32;

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

            /*
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
            */

            if day_class_count_first_group < 4 || day_class_count_second_group < 4 {
                points += 1;
            }
        }
    }

    */

    points
}

/// Same as `soft_class_spread` except for teachers instead of classes
pub fn soft_teacher_class_spread(timetable: &Timetable, teacher_table: &Vec<TeacherSlots>) -> i32 {
    let mut points = 0;
    /*

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

        let mut one_day_with6 = false;
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
            if day_class_count == 1 {
                points += 1;
            }
            */
            if day_class_count >= 6 {
                one_day_with6 = true;
                break;
            }
        }

        if !one_day_with6 {
            points += 2;
        }
    }

    */
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

            if holes > 1 {
                points += 1;
            }
        }
    }

    points
}

pub fn soft_preferred_subject_times(timetable: &Timetable, shift: Shift) -> i32 {
    let mut points = 0;
    /*

    for class_slots in timetable.table(shift).iter() {
        for day in 0..5 {
            // math shouldn't be the 6th or 7th class
            let start = 5;
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
            /*
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
            */
        }
    }

    */
    points
}

pub fn teacher_shifts(
    table1: &Vec<ClassSlots>,
    table2: &Vec<ClassSlots>,
    max_periods_per_day: u32,
    data: &TimetableData,
    debug: bool,
) -> i32 {
    let mut points = 0;

    for day in 0..5 {
        let mut teachers_seen_in_shift1: HashMap<usize, u32> = HashMap::new();
        let mut teachers_seen_in_shift2: HashMap<usize, u32> = HashMap::new();

        for period in 0..max_periods_per_day {
            let index = day * max_periods_per_day + period;

            for class_slots in table1.iter() {
                match class_slots.slots[index as usize] {
                    Slot::Single(SlotData::PartiallyFilled { teacher, .. }) => {
                        teachers_seen_in_shift1
                            .entry(teacher)
                            .and_modify(|n| *n += 1)
                            .or_insert(1);

                        // teachers_seen_in_shift1.insert(teacher, true);
                    }
                    Slot::Double { first, second, .. } => {
                        match first {
                            SlotData::PartiallyFilled { teacher, .. } => {
                                teachers_seen_in_shift1
                                    .entry(teacher)
                                    .and_modify(|n| *n += 1)
                                    .or_insert(1);

                                // teachers_seen_in_shift1.insert(teacher, true);
                            }

                            _ => {}
                        }

                        match second {
                            SlotData::PartiallyFilled { teacher, .. } => {
                                teachers_seen_in_shift1
                                    .entry(teacher)
                                    .and_modify(|n| *n += 1)
                                    .or_insert(1);

                                // teachers_seen_in_shift1.insert(teacher, true);
                            }

                            _ => {}
                        }
                    }

                    _ => {}
                }
            }

            for class_slots in table2.iter() {
                match class_slots.slots[index as usize] {
                    Slot::Single(SlotData::PartiallyFilled { teacher, .. }) => {
                        teachers_seen_in_shift2
                            .entry(teacher)
                            .and_modify(|n| *n += 1)
                            .or_insert(1);

                        // teachers_seen_in_shift2.insert(teacher, true);
                    }
                    Slot::Double { first, second, .. } => {
                        match first {
                            SlotData::PartiallyFilled { teacher, .. } => {
                                teachers_seen_in_shift2
                                    .entry(teacher)
                                    .and_modify(|n| *n += 1)
                                    .or_insert(1);

                                // teachers_seen_in_shift2.insert(teacher, true);
                            }

                            _ => {}
                        }

                        match second {
                            SlotData::PartiallyFilled { teacher, .. } => {
                                teachers_seen_in_shift2
                                    .entry(teacher)
                                    .and_modify(|n| *n += 1)
                                    .or_insert(1);

                                // teachers_seen_in_shift2.insert(teacher, true);
                            }

                            _ => {}
                        }
                    }

                    _ => {}
                }
            }
        }

        for t in teachers_seen_in_shift1.keys() {
            if teachers_seen_in_shift2.contains_key(t) {
                points += 2;

                if debug {
                    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
                    println!(
                        "  {} ({}) [{}]",
                        data.teachers[*t].name,
                        days[day as usize],
                        teachers_seen_in_shift1[t] + teachers_seen_in_shift2[t]
                    );
                }
            }
        }
    }

    if debug {
        println!("\nTotal cost: {}", points);
    }

    points
}
