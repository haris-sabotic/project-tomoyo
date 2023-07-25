<script>
    import TimetableClassSlot from "./TimetableClassSlot.svelte";

    export let timetable;
    export let classes;
    export let subjects;
    export let teachers;
    export let rooms;

    let selectedClass = null;
    let selectedSlot = null;

    let onClassSlotClick = (class_index, slot_index) => {
        if (selectedClass == null) {
            selectedClass = class_index;
            selectedSlot = slot_index;
        } else {
            // only swap if it's 2 slots within the same class
            if (selectedClass == class_index) {
                let tmp = timetable.table[selectedClass].slots[selectedSlot];
                timetable.table[selectedClass].slots[selectedSlot] =
                    timetable.table[class_index].slots[slot_index];
                timetable.table[class_index].slots[slot_index] = tmp;
            }

            selectedClass = null;
            selectedSlot = null;
        }

        console.log(
            "Class index: " + class_index + ", Slot index: " + slot_index
        );
        console.log(
            "Selected class index: " +
                selectedClass +
                ", Selected slot index: " +
                selectedSlot
        );
    };
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
        <th>Class name</th>
        {#each { length: 5 } as _, _}
            <th class="day-begin">1</th>
            {#each { length: timetable.max_periods_per_day - 2 } as _, i}
                <th>{i + 2}</th>
            {/each}
            <th class="day-end">{timetable.max_periods_per_day}</th>
        {/each}
    </tr>
    {#each timetable.table as { class_index, slots }}
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
                    selected={class_index == selectedClass &&
                        slot_index == selectedSlot}
                    onClick={() => onClassSlotClick(class_index, slot_index)}
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
