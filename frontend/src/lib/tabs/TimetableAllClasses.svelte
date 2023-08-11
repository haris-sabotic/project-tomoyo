<script>
    import TimetableClassSlot from "./TimetableClassSlot.svelte";

    export let timetable;
    export let classes;
    export let subjects;
    export let teachers;
    export let rooms;

    let selectedShift = null;
    let selectedClass = null;
    let selectedSlot = null;

    let onClassSlotClick = (shift, class_index, slot_index) => {
        if (selectedClass == null) {
            selectedShift = shift;
            selectedClass = class_index;
            selectedSlot = slot_index;
        } else {
            // only swap if it's 2 slots within the same class
            if (selectedClass == class_index) {
                if (shift == 1) {
                    let tmp =
                        timetable.table1[selectedClass].slots[selectedSlot];
                    timetable.table1[selectedClass].slots[selectedSlot] =
                        timetable.table1[class_index].slots[slot_index];
                    timetable.table1[class_index].slots[slot_index] = tmp;
                } else if (shift == 2) {
                    let tmp =
                        timetable.table2[selectedClass].slots[selectedSlot];
                    timetable.table2[selectedClass].slots[selectedSlot] =
                        timetable.table2[class_index].slots[slot_index];
                    timetable.table2[class_index].slots[slot_index] = tmp;
                }
            }

            selectedShift = null;
            selectedClass = null;
            selectedSlot = null;
        }
    };
</script>

<h1>FIRST SHIFT</h1>

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
        <th>Class name</th>
        {#each { length: 5 } as _, _}
            <th class="day-begin">1</th>
            {#each { length: timetable.max_periods_per_day - 2 } as _, i}
                <th>{i + 2}</th>
            {/each}
            <th class="day-end">{timetable.max_periods_per_day}</th>
        {/each}
    </tr>
    {#each timetable.table1 as { class_index, slots }}
        <tr>
            <td><strong>{classes[class_index]}</strong></td>
            {#each slots as slot, slot_index}
                <TimetableClassSlot
                    day_separators={true}
                    {slot_index}
                    {slot}
                    {subjects}
                    {teachers}
                    {rooms}
                    max_periods_per_day={timetable.max_periods_per_day}
                    selected={selectedShift == 1 &&
                        class_index == selectedClass &&
                        slot_index == selectedSlot}
                    onClick={() => onClassSlotClick(1, class_index, slot_index)}
                />
            {/each}
        </tr>
    {/each}
</table>

<h1>SECOND SHIFT</h1>

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
        <th>Class name</th>
        {#each { length: 5 } as _, _}
            <th class="day-begin">1</th>
            {#each { length: timetable.max_periods_per_day - 2 } as _, i}
                <th>{i + 2}</th>
            {/each}
            <th class="day-end">{timetable.max_periods_per_day}</th>
        {/each}
    </tr>
    {#each timetable.table2 as { class_index, slots }}
        <tr>
            <td><strong>{classes[class_index]}</strong></td>
            {#each slots as slot, slot_index}
                <TimetableClassSlot
                    day_separators={true}
                    {slot_index}
                    {slot}
                    {subjects}
                    {teachers}
                    {rooms}
                    max_periods_per_day={timetable.max_periods_per_day}
                    selected={selectedShift == 2 &&
                        class_index == selectedClass &&
                        slot_index == selectedSlot}
                    onClick={() => onClassSlotClick(2, class_index, slot_index)}
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
