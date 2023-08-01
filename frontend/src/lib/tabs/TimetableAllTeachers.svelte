<script>
    import TimetableTeacherSlot from "./TimetableTeacherSlot.svelte";

    export let timetable;
    export let classes;
    export let subjects;
    export let teachers;
    export let rooms;

    let teachers_timetable = [];

    console.log(timetable);

    for (let i = 0; i < teachers.length; i++) {
        let slots = [];
        for (let j = 0; j < 5 * timetable.max_periods_per_day; j++) {
            slots.push("Empty");
        }

        teachers_timetable.push({
            teacher_index: i,
            slots: slots,
        });
    }

    timetable.table.forEach((el) => {
        let class_index = el.class_index;
        let slots = el.slots;

        for (let i = 0; i < slots.length; i++) {
            const slot = slots[i];

            if (slot["Single"]) {
                if (slot["Single"]["PartiallyFilled"]) {
                    let teacher = slot["Single"]["PartiallyFilled"].teacher;
                    let subject = slot["Single"]["PartiallyFilled"].subject;

                    teachers_timetable[teacher].slots[i] = {
                        PartiallyFilled: {
                            subject: subject,
                            class: class_index,
                        },
                    };
                } else if (slot["Single"]["Filled"]) {
                    let teacher = slot["Single"]["Filled"].teacher;
                    let subject = slot["Single"]["Filled"].subject;
                    let room = slot["Single"]["Filled"].room;

                    teachers_timetable[teacher].slots[i] = {
                        Filled: {
                            subject: subject,
                            class: class_index,
                            room: room,
                        },
                    };
                }
            } else if (slot["Double"]) {
                // first
                if (slot["Double"]["first"]["PartiallyFilled"]) {
                    let teacher =
                        slot["Double"]["first"]["PartiallyFilled"].teacher;
                    let subject =
                        slot["Double"]["first"]["PartiallyFilled"].subject;

                    teachers_timetable[teacher].slots[i] = {
                        PartiallyFilled: {
                            subject: subject,
                            class: class_index,
                        },
                    };
                } else if (slot["Double"]["first"]["Filled"]) {
                    let teacher = slot["Double"]["first"]["Filled"].teacher;
                    let subject = slot["Double"]["first"]["Filled"].subject;
                    let room = slot["Double"]["first"]["Filled"].room;

                    teachers_timetable[teacher].slots[i] = {
                        Filled: {
                            subject: subject,
                            class: class_index,
                            room: room,
                        },
                    };
                }
                
                // second
                if (slot["Double"]["second"]["PartiallyFilled"]) {
                    let teacher =
                        slot["Double"]["second"]["PartiallyFilled"].teacher;
                    let subject =
                        slot["Double"]["second"]["PartiallyFilled"].subject;

                    teachers_timetable[teacher].slots[i] = {
                        PartiallyFilled: {
                            subject: subject,
                            class: class_index,
                        },
                    };
                } else if (slot["Double"]["second"]["Filled"]) {
                    let teacher = slot["Double"]["second"]["Filled"].teacher;
                    let subject = slot["Double"]["second"]["Filled"].subject;
                    let room = slot["Double"]["second"]["Filled"].room;

                    teachers_timetable[teacher].slots[i] = {
                        Filled: {
                            subject: subject,
                            class: class_index,
                            room: room,
                        },
                    };
                }
            }
        }
    });
</script>

<table>
    <tr>
        <th />
        <th class="day-begin day-end" colspan={timetable.max_periods_per_day}
            >Mon</th
        >
        <th class="day-begin day-end" colspan={timetable.max_periods_per_day}
            >Tue</th
        >
        <th class="day-begin day-end" colspan={timetable.max_periods_per_day}
            >Wed</th
        >
        <th class="day-begin day-end" colspan={timetable.max_periods_per_day}
            >Thu</th
        >
        <th class="day-begin day-end" colspan={timetable.max_periods_per_day}
            >Fri</th
        >
    </tr>
    <tr>
        <th>Teacher name</th>
        {#each { length: 5 } as _, _}
            <th class="day-begin">1</th>
            {#each { length: timetable.max_periods_per_day - 2 } as _, i}
                <th>{i + 2}</th>
            {/each}
            <th class="day-end">{timetable.max_periods_per_day}</th>
        {/each}
    </tr>
    {#each teachers_timetable as { teacher_index, slots }}
        <tr>
            <td><strong>{teachers[teacher_index]}</strong></td>
            {#each slots as slot, slot_index}
                <TimetableTeacherSlot
                    day_separators={true}
                    {slot_index}
                    {slot}
                    {subjects}
                    {classes}
                    {rooms}
                    max_periods_per_day={timetable.max_periods_per_day}
                />
            {/each}
        </tr>
    {/each}
</table>

<style>
    th,
    td {
        border: 1px solid black;
        margin: 0;
    }

    th {
        background-color: rgb(245, 245, 245);
    }

    td {
        padding: 10px;
    }

    .day-begin {
        border-left: 5px solid red;
    }
    .day-end {
        border-right: 5px solid red;
    }
</style>
