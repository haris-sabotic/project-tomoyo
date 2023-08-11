<script>
// @ts-nocheck

    import TimetableClassSlot from "./TimetableClassSlot.svelte";

    export let shift;
    export let class_index;
    export let timetable;
    export let subjects;
    export let teachers;
    export let rooms;

    const DAYS = ["Mon", "Tue", "Wed", "Thu", "Fri"];

    let getTable = () => {
        if (shift == 1) {
            return timetable.table1
        } else {
            return timetable.table2
        }
    }
</script>

<table>
    <tr>
        <th />
        {#each { length: timetable.max_periods_per_day } as _, i}
            <th>{i + 1}</th>
        {/each}
    </tr>

    {#each DAYS as dayName, day}
        <tr>
            <td class="day-cell">{dayName}</td>

            {#each { length: timetable.max_periods_per_day } as _, period}
                <TimetableClassSlot
                    day_separators={false}
                    slot_index={day * timetable.max_periods_per_day + period}
                    slot={getTable()[class_index].slots[day * timetable.max_periods_per_day + period]}
                    {subjects}
                    {teachers}
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

    .day-cell {
        background-color: rgb(245, 245, 245);
    }
</style>
