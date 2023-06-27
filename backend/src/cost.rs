use std::collections::HashMap;

use crate::logic::{Slot, Timetable};

/* ==================== */
/*   HARD CONSTRAINTS   */
/* ==================== */

/// Increment points by 10 for each teacher teaching multiple classes in the same period
pub fn hard_repeating_teachers(timetable: &Timetable) -> i32 {
    let mut points = 0;

    for period in 0..(timetable.max_periods_per_day * 5) {
        let mut seen_teachers: Vec<usize> = vec![];

        for class in 0..timetable.table.len() {
            match timetable.table[class].slots[period as usize] {
                Slot::PartiallyFilled { teacher, .. } => {
                    if seen_teachers.contains(&teacher) {
                        points += 10;
                    } else {
                        seen_teachers.push(teacher);
                    }
                }

                _ => {}
            }
        }
    }

    points
}

/// Increment points by 5 for every hole in a class timetable
pub fn hard_holes_in_class_timetable(timetable: &Timetable) -> i32 {
    let mut points = 0;

    for class_slots in timetable.table.iter() {
        for day in 0..5 {
            let mut day_ended = false;

            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match class_slots.slots[index as usize] {
                    Slot::Empty => {
                        day_ended = true;
                    }

                    _ => {
                        if day_ended {
                            points += 5;
                        }
                    }
                }
            }
        }
    }

    points
}

/// Increment points by 20 for each period during which too many classes are being held with the same kind
///
/// Example: During the 2nd period of Tuesday, 5 IT classes are being held, when there's only 3 computer classrooms in the school
pub fn hard_too_many_subjects_of_same_kind(
    timetable: &Timetable,
    room_kinds_count: &HashMap<String, u32>,
) -> i32 {
    let mut points = 0;

    for period in 0..(timetable.max_periods_per_day * 5) {
        let mut subject_kinds_count: HashMap<String, u32> = HashMap::new();

        for class_slots in timetable.table.iter() {
            match class_slots.slots[period as usize] {
                Slot::PartiallyFilled { subject, .. } => {
                    subject_kinds_count
                        .entry(timetable.data.subjects[subject].kind.clone())
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }

                _ => {}
            }
        }

        for kind in subject_kinds_count.keys() {
            if subject_kinds_count[kind] > room_kinds_count[kind] {
                points += 20;
            }
        }
    }

    points
}

/* ==================== */
/*   SOFT CONSTRAINTS   */
/* ==================== */

/// Increment points by 5 for each day in a class timetable that contains more periods than what's ideal (even spread)
pub fn soft_class_spread(timetable: &Timetable) -> i32 {
    let mut points = 0;

    for class_slots in timetable.table.iter() {
        // ideal number of classes per day
        let mut class_count = 0;
        for slot in class_slots.slots.iter() {
            match slot {
                Slot::Empty => {}
                _ => class_count += 1,
            }
        }
        let ideal_class_spread = (class_count as f32 / 5.0).ceil() as i32;

        for day in 0..5 {
            // calculate number of classes during this day
            let mut day_class_count = 0;
            for period in 0..timetable.max_periods_per_day {
                let index = day * timetable.max_periods_per_day + period;

                match class_slots.slots[index as usize] {
                    Slot::Empty => {
                        break;
                    }
                    _ => {
                        day_class_count += 1;
                    }
                }
            }

            if day_class_count > ideal_class_spread {
                points += 5;
            }
        }
    }

    points
}

/// Increment points by 5 for each teacher that doesn't have a day off
pub fn soft_teacher_free_days(timetable: &Timetable) -> i32 {
    let mut points = 0;

    points
}
