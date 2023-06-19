<script>
    import TimetableSlot from "./TimetableSlot.svelte";

    export let class_index;
    export let timetable;
    export let subjects;
    export let teachers;
    export let rooms;

    const DAYS = ["Mon", "Tue", "Wed", "Thu", "Fri"];
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
                <TimetableSlot
                    day_separators={false}
                    slot_index={day * timetable.max_periods_per_day + period}
                    slot={timetable.table[class_index].slots[day * timetable.max_periods_per_day + period]}
                    {subjects}
                    {teachers}
                    {rooms}
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
