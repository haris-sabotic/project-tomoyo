<script>
    import TimetableTeacherSlot from "./TimetableTeacherSlot.svelte";

    export let timetable;
    export let classes;
    export let subjects;
    export let teachers;
    export let rooms;

    let teachers_timetable1 = [];
    let teachers_timetable2 = [];

    console.log(timetable);

    for (let i = 0; i < teachers.length; i++) {
        let slots1 = [];
        for (let j = 0; j < 5 * timetable.max_periods_per_day; j++) {
            slots1.push("Empty");
        }

        let slots2 = [];
        for (let j = 0; j < 5 * timetable.max_periods_per_day; j++) {
            slots2.push("Empty");
        }

        teachers_timetable1.push({
            teacher_index: i,
            slots: slots1,
        });

        teachers_timetable2.push({
            teacher_index: i,
            slots: slots2,
        });
    }

    timetable.table1.forEach((el) => {
        let class_index = el.class_index;
        let slots = el.slots;

        for (let i = 0; i < slots.length; i++) {
            const slot = slots[i];

            if (slot["Single"]) {
                if (slot["Single"]["PartiallyFilled"]) {
                    let teacher = slot["Single"]["PartiallyFilled"].teacher;
                    let subject = slot["Single"]["PartiallyFilled"].subject;
                    let room = slot["Single"]["PartiallyFilled"].room;

                    teachers_timetable1[teacher].slots[i] = {
                        PartiallyFilled: {
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
                    let room = slot["Double"]["first"]["PartiallyFilled"].room;

                    teachers_timetable1[teacher].slots[i] = {
                        PartiallyFilled: {
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
                    let room = slot["Double"]["second"]["PartiallyFilled"].room;

                    teachers_timetable1[teacher].slots[i] = {
                        PartiallyFilled: {
                            subject: subject,
                            class: class_index,
                            room: room,
                        },
                    };
                }
            }
        }
    });

    timetable.table2.forEach((el) => {
        let class_index = el.class_index;
        let slots = el.slots;

        for (let i = 0; i < slots.length; i++) {
            const slot = slots[i];

            if (slot["Single"]) {
                if (slot["Single"]["PartiallyFilled"]) {
                    let teacher = slot["Single"]["PartiallyFilled"].teacher;
                    let subject = slot["Single"]["PartiallyFilled"].subject;
                    let room = slot["Single"]["PartiallyFilled"].room;

                    teachers_timetable2[teacher].slots[i] = {
                        PartiallyFilled: {
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
                    let room = slot["Double"]["first"]["PartiallyFilled"].room;

                    teachers_timetable2[teacher].slots[i] = {
                        PartiallyFilled: {
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
                    let room = slot["Double"]["second"]["PartiallyFilled"].room;

                    teachers_timetable2[teacher].slots[i] = {
                        PartiallyFilled: {
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

<h1>Shift 1</h1>
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
    {#each teachers_timetable1 as { teacher_index, slots }}
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

<h1>Shift 2</h1>
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
    {#each teachers_timetable2 as { teacher_index, slots }}
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
