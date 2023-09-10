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

    const PERIOD_TIMES = {
        first: [
            "7:00 - 7:45",
            "7:50 - 8:35",
            "8:50 - 9:35",
            "9:40 - 10:25",
            "10:35 - 11:20",
            "11:25 - 12:10",
            "12:15 - 13:00",
        ],
        second: [
            "13:10 - 13:55",
            "14:00 - 14:45",
            "15:00 - 15:45",
            "15:50 - 16:35",
            "16:45 - 17:30",
            "17:35 - 18:20",
            "18:25 - 19:10",
        ],
    };

    let getTable = () => {
        if (shift == 1) {
            return timetable.table1;
        } else {
            return timetable.table2;
        }
    };

    let selectedSlot = null;

    let onClassSlotClick = (slot_index) => {
        if (selectedSlot == null) {
            selectedSlot = slot_index;
        } else {
            if (shift == 1) {
                let tmp = timetable.table1[class_index].slots[selectedSlot];
                timetable.table1[class_index].slots[selectedSlot] =
                    timetable.table1[class_index].slots[slot_index];
                timetable.table1[class_index].slots[slot_index] = tmp;
            } else if (shift == 2) {
                let tmp = timetable.table2[class_index].slots[selectedSlot];
                timetable.table2[class_index].slots[selectedSlot] =
                    timetable.table2[class_index].slots[slot_index];
                timetable.table2[class_index].slots[slot_index] = tmp;
            }

            selectedSlot = null;
        }
    };

    let handleSwitchGroups = () => {
        if (shift == 1) {
            let tmp =
                timetable.table1[class_index].slots[selectedSlot]["Double"][
                    "first"
                ];
            timetable.table1[class_index].slots[selectedSlot]["Double"][
                "first"
            ] =
                timetable.table1[class_index].slots[selectedSlot]["Double"][
                    "second"
                ];
            timetable.table1[class_index].slots[selectedSlot]["Double"][
                "second"
            ] = tmp;
        } else if (shift == 2) {
            let tmp =
                timetable.table2[class_index].slots[selectedSlot]["Double"][
                    "first"
                ];
            timetable.table2[class_index].slots[selectedSlot]["Double"][
                "first"
            ] =
                timetable.table2[class_index].slots[selectedSlot]["Double"][
                    "second"
                ];
            timetable.table2[class_index].slots[selectedSlot]["Double"][
                "second"
            ] = tmp;
        }
    };
</script>

<!--
<p>Switch groups:</p>
<button on:click={handleSwitchGroups}>OK</button>
-->

<table>
    <tr>
        <th />
        {#each { length: timetable.max_periods_per_day } as _, i}
            <th>
                <p>{i + 1}</p>
                <div>
                    <p>{PERIOD_TIMES.first[i]}</p>
                    <p>{PERIOD_TIMES.second[i]}</p>
                </div>
            </th>
        {/each}
    </tr>

    {#each DAYS as dayName, day}
        <tr>
            <td class="day-cell">{dayName}</td>

            {#each { length: timetable.max_periods_per_day } as _, period}
                <TimetableClassSlot
                    day_separators={false}
                    slot_index={day * timetable.max_periods_per_day + period}
                    slot={getTable()[class_index].slots[
                        day * timetable.max_periods_per_day + period
                    ]}
                    {subjects}
                    {teachers}
                    {rooms}
                    max_periods_per_day={timetable.max_periods_per_day}
                    selected={selectedSlot ==
                        day * timetable.max_periods_per_day + period}
                    onClick={() =>
                        onClassSlotClick(
                            day * timetable.max_periods_per_day + period
                        )}
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

    th > p {
        margin: 0;
        font-size: 1.5em;
    }

    th > div {
        margin-top: 10px;
        margin-bottom: 5px;
    }

    th > div > p {
        font-weight: normal;
        margin: 0;
    }
</style>
