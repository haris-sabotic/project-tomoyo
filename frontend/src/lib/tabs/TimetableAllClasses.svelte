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

    let selectedRoom = null;
    let selectedGroup = "single";
    let selectedGroupTeacher = "single";
    let selectedSwapSlots = "single";
    let selectedTeacher = null;
    let selectedGroupAdd = null;
    let selectedTeacherAdd = null;
    let selectedSubjectAdd = null;

    let showFirstShift = true;
    let showSecondShift = true;

    let makeDoubleBefore = 0;
    let makeDoubleAfter = 0;

    let onClassSlotClick = (shift, class_index, slot_index) => {
        if (selectedClass == null) {
            selectedShift = shift;
            selectedClass = class_index;
            selectedSlot = slot_index;
        } else {
            // only swap if it's 2 slots within the same class
            if (selectedClass == class_index) {
                if (shift == 1) {
                    if (selectedSwapSlots == "single") {
                        let tmp =
                            timetable.table1[selectedClass].slots[selectedSlot];
                        timetable.table1[selectedClass].slots[selectedSlot] =
                            timetable.table1[class_index].slots[slot_index];
                        timetable.table1[class_index].slots[slot_index] = tmp;
                    } else if (selectedSwapSlots == "first") {
                        let tmp =
                            timetable.table1[selectedClass].slots[selectedSlot][
                                "Double"
                            ]["first"];
                        timetable.table1[selectedClass].slots[selectedSlot][
                            "Double"
                        ]["first"] =
                            timetable.table1[class_index].slots[slot_index][
                                "Double"
                            ]["first"];
                        timetable.table1[class_index].slots[slot_index][
                            "Double"
                        ]["first"] = tmp;
                    } else if (selectedSwapSlots == "second") {
                        let tmp =
                            timetable.table1[selectedClass].slots[selectedSlot][
                                "Double"
                            ]["second"];
                        timetable.table1[selectedClass].slots[selectedSlot][
                            "Double"
                        ]["second"] =
                            timetable.table1[class_index].slots[slot_index][
                                "Double"
                            ]["second"];
                        timetable.table1[class_index].slots[slot_index][
                            "Double"
                        ]["second"] = tmp;
                    }
                } else if (shift == 2) {
                    if (selectedSwapSlots == "single") {
                        let tmp =
                            timetable.table2[selectedClass].slots[selectedSlot];
                        timetable.table2[selectedClass].slots[selectedSlot] =
                            timetable.table2[class_index].slots[slot_index];
                        timetable.table2[class_index].slots[slot_index] = tmp;
                    } else if (selectedSwapSlots == "first") {
                        let tmp =
                            timetable.table2[selectedClass].slots[selectedSlot][
                                "Double"
                            ]["first"];
                        timetable.table2[selectedClass].slots[selectedSlot][
                            "Double"
                        ]["first"] =
                            timetable.table2[class_index].slots[slot_index][
                                "Double"
                            ]["first"];
                        timetable.table2[class_index].slots[slot_index][
                            "Double"
                        ]["first"] = tmp;
                    } else if (selectedSwapSlots == "second") {
                        let tmp =
                            timetable.table2[selectedClass].slots[selectedSlot][
                                "Double"
                            ]["second"];
                        timetable.table2[selectedClass].slots[selectedSlot][
                            "Double"
                        ]["second"] =
                            timetable.table2[class_index].slots[slot_index][
                                "Double"
                            ]["second"];
                        timetable.table2[class_index].slots[slot_index][
                            "Double"
                        ]["second"] = tmp;
                    }
                }
            }

            selectedShift = null;
            selectedClass = null;
            selectedSlot = null;
        }
    };

    let handleChangeRoom = (group) => {
        if (selectedClass != null && selectedRoom != null) {
            if (selectedShift == 1) {
                if (selectedGroup == "single") {
                    timetable.table1[selectedClass].slots[selectedSlot][
                        "Single"
                    ]["PartiallyFilled"].room = selectedRoom;
                } else if (selectedGroup == "first") {
                    timetable.table1[selectedClass].slots[selectedSlot][
                        "Double"
                    ]["first"]["PartiallyFilled"].room = selectedRoom;
                } else if (selectedGroup == "second") {
                    timetable.table1[selectedClass].slots[selectedSlot][
                        "Double"
                    ]["second"]["PartiallyFilled"].room = selectedRoom;
                }
            } else if (selectedShift == 2) {
                if (selectedGroup == "single") {
                    timetable.table2[selectedClass].slots[selectedSlot][
                        "Single"
                    ]["PartiallyFilled"].room = selectedRoom;
                } else if (selectedGroup == "first") {
                    timetable.table2[selectedClass].slots[selectedSlot][
                        "Double"
                    ]["first"]["PartiallyFilled"].room = selectedRoom;
                } else if (selectedGroup == "second") {
                    timetable.table2[selectedClass].slots[selectedSlot][
                        "Double"
                    ]["second"]["PartiallyFilled"].room = selectedRoom;
                }
            }
        }
    };

    let handleChangeTeacher = () => {
        if (selectedClass != null && selectedTeacher != null) {
            if (selectedShift == 1) {
                if (selectedGroupTeacher == "single") {
                    timetable.table1[selectedClass].slots[selectedSlot][
                        "Single"
                    ]["PartiallyFilled"].teacher = selectedTeacher;
                } else if (selectedGroupTeacher == "first") {
                    timetable.table1[selectedClass].slots[selectedSlot][
                        "Double"
                    ]["first"]["PartiallyFilled"].teacher = selectedTeacher;
                } else if (selectedGroupTeacher == "second") {
                    timetable.table1[selectedClass].slots[selectedSlot][
                        "Double"
                    ]["second"]["PartiallyFilled"].teacher = selectedTeacher;
                }
            } else if (selectedShift == 2) {
                if (selectedGroupTeacher == "single") {
                    timetable.table2[selectedClass].slots[selectedSlot][
                        "Single"
                    ]["PartiallyFilled"].teacher = selectedTeacher;
                } else if (selectedGroupTeacher == "first") {
                    timetable.table2[selectedClass].slots[selectedSlot][
                        "Double"
                    ]["first"]["PartiallyFilled"].teacher = selectedTeacher;
                } else if (selectedGroupTeacher == "second") {
                    timetable.table2[selectedClass].slots[selectedSlot][
                        "Double"
                    ]["second"]["PartiallyFilled"].teacher = selectedTeacher;
                }
            }
        }
    };

    let handleSwitchGroups = () => {
        if (selectedShift == 1) {
            let tmp =
                timetable.table1[selectedClass].slots[selectedSlot]["Double"][
                    "first"
                ];
            timetable.table1[selectedClass].slots[selectedSlot]["Double"][
                "first"
            ] =
                timetable.table1[selectedClass].slots[selectedSlot]["Double"][
                    "second"
                ];
            timetable.table1[selectedClass].slots[selectedSlot]["Double"][
                "second"
            ] = tmp;
        } else if (selectedShift == 2) {
            let tmp =
                timetable.table2[selectedClass].slots[selectedSlot]["Double"][
                    "first"
                ];
            timetable.table2[selectedClass].slots[selectedSlot]["Double"][
                "first"
            ] =
                timetable.table2[selectedClass].slots[selectedSlot]["Double"][
                    "second"
                ];
            timetable.table2[selectedClass].slots[selectedSlot]["Double"][
                "second"
            ] = tmp;
        }
    };

    let handleAddSlot = () => {
        if (selectedShift == 1) {
            if (selectedGroupAdd == "single") {
                timetable.table1[selectedClass].slots[selectedSlot]["Single"] =
                    {
                        PartiallyFilled: {
                            room: null,
                            subject: selectedSubjectAdd,
                            teacher: selectedTeacherAdd,
                        },
                    };
            } else if (selectedGroupAdd == "first") {
                timetable.table1[selectedClass].slots[selectedSlot]["Double"][
                    "first"
                ] = {
                    PartiallyFilled: {
                        room: null,
                        subject: selectedSubjectAdd,
                        teacher: selectedTeacherAdd,
                    },
                };
            } else if (selectedGroupAdd == "second") {
                timetable.table1[selectedClass].slots[selectedSlot]["Double"][
                    "second"
                ] = {
                    PartiallyFilled: {
                        room: null,
                        subject: selectedSubjectAdd,
                        teacher: selectedTeacherAdd,
                    },
                };
            }
        } else if (selectedShift == 2) {
            if (selectedGroupAdd == "single") {
                timetable.table2[selectedClass].slots[selectedSlot]["Single"] =
                    {
                        PartiallyFilled: {
                            room: null,
                            subject: selectedSubjectAdd,
                            teacher: selectedTeacherAdd,
                        },
                    };
            } else if (selectedGroupAdd == "first") {
                timetable.table2[selectedClass].slots[selectedSlot]["Double"][
                    "first"
                ] = {
                    PartiallyFilled: {
                        room: null,
                        subject: selectedSubjectAdd,
                        teacher: selectedTeacherAdd,
                    },
                };
            } else if (selectedGroupAdd == "second") {
                timetable.table2[selectedClass].slots[selectedSlot]["Double"][
                    "second"
                ] = {
                    PartiallyFilled: {
                        room: null,
                        subject: selectedSubjectAdd,
                        teacher: selectedTeacherAdd,
                    },
                };
            }
        }
    };

    let handleMakeDouble = () => {
        if (selectedShift == 1) {
            timetable.table1[selectedClass].slots[selectedSlot] = {
                Double: {
                    first: "Empty",
                    second: "Empty",
                    before: makeDoubleBefore,
                    after: makeDoubleAfter,
                },
            };
        } else if (selectedShift == 2) {
            timetable.table2[selectedClass].slots[selectedSlot] = {
                Double: {
                    first: "Empty",
                    second: "Empty",
                    before: makeDoubleBefore,
                    after: makeDoubleAfter,
                },
            };
        }
    };

    let handleMakeEmpty = () => {
        if (selectedShift == 1) {
            timetable.table1[selectedClass].slots[selectedSlot] = {
                Single: "Empty",
            };
        } else if (selectedShift == 2) {
            timetable.table2[selectedClass].slots[selectedSlot] = {
                Single: "Empty",
            };
        }
    };
</script>

<div class="visibility">
    <button on:click={() => (showFirstShift = !showFirstShift)}
        >SHOW FIRST</button
    >
    <button on:click={() => (showSecondShift = !showSecondShift)}
        >SHOW SECOND</button
    >
</div>
<div class="table-controls">
    <div class="change-room">
        <p>Change room:</p>
        <select bind:value={selectedRoom}>
            {#each rooms as room, room_id}
                <option value={room_id}>
                    {room.name}
                </option>
            {/each}
        </select>
        <select bind:value={selectedGroup}>
            <option value="single">Single</option>
            <option value="first">First</option>
            <option value="second">Second</option>
        </select>
        <button on:click={handleChangeRoom}>OK</button>
    </div>

    <div class="swap-slots">
        <p>Swap slots:</p>
        <select bind:value={selectedSwapSlots}>
            <option value="single">Single</option>
            <option value="first">First</option>
            <option value="second">Second</option>
        </select>
    </div>

    <div class="switch-groups">
        <p>Switch groups:</p>
        <button on:click={handleSwitchGroups}>OK</button>
    </div>

    <div class="make-double">
        <p>Make double:</p>
        <input
            type="number"
            bind:value={makeDoubleBefore}
            placeholder="before"
        />
        <input type="number" bind:value={makeDoubleAfter} placeholder="after" />
        <button on:click={handleMakeDouble}>OK</button>
    </div>

    <div class="change-teacher">
        <p>Change teacher:</p>
        <select bind:value={selectedTeacher}>
            {#each teachers as teacher, teacher_id}
                <option value={teacher_id}>
                    {teacher}
                </option>
            {/each}
        </select>
        <select bind:value={selectedGroupTeacher}>
            <option value="single">Single</option>
            <option value="first">First</option>
            <option value="second">Second</option>
        </select>
        <button on:click={handleChangeTeacher}>OK</button>
    </div>

    <div class="add-single">
        <p>Add slot:</p>
        <select bind:value={selectedGroupAdd}>
            <option value="single">Single</option>
            <option value="first">First</option>
            <option value="second">Second</option>
        </select>
        <select bind:value={selectedTeacherAdd}>
            {#each teachers as teacher, teacher_id}
                <option value={teacher_id}>
                    {teacher}
                </option>
            {/each}
        </select>
        <select bind:value={selectedSubjectAdd}>
            {#each subjects as subject, subject_id}
                <option value={subject_id}>
                    {subject.name}
                </option>
            {/each}
        </select>
        <button on:click={handleAddSlot}>OK</button>
    </div>

    <div class="make-empty">
        <p>Make empty:</p>
        <button on:click={handleMakeEmpty}>OK</button>
    </div>
</div>

{#if showFirstShift}
    <h1>FIRST SHIFT</h1>

    <table>
        <tr>
            <th />
            <th
                class="day-begin day-end"
                colspan={timetable.max_periods_per_day}>Mon</th
            >
            <th
                class="day-begin day-end"
                colspan={timetable.max_periods_per_day}>Tue</th
            >
            <th
                class="day-begin day-end"
                colspan={timetable.max_periods_per_day}>Wed</th
            >
            <th
                class="day-begin day-end"
                colspan={timetable.max_periods_per_day}>Thu</th
            >
            <th
                class="day-begin day-end"
                colspan={timetable.max_periods_per_day}>Fri</th
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
                        onClick={() =>
                            onClassSlotClick(1, class_index, slot_index)}
                    />
                {/each}
            </tr>
        {/each}
    </table>
{/if}

{#if showSecondShift}
    <h1>SECOND SHIFT</h1>

    <table>
        <tr>
            <th />
            <th
                class="day-begin day-end"
                colspan={timetable.max_periods_per_day}>Mon</th
            >
            <th
                class="day-begin day-end"
                colspan={timetable.max_periods_per_day}>Tue</th
            >
            <th
                class="day-begin day-end"
                colspan={timetable.max_periods_per_day}>Wed</th
            >
            <th
                class="day-begin day-end"
                colspan={timetable.max_periods_per_day}>Thu</th
            >
            <th
                class="day-begin day-end"
                colspan={timetable.max_periods_per_day}>Fri</th
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
                        onClick={() =>
                            onClassSlotClick(2, class_index, slot_index)}
                    />
                {/each}
            </tr>
        {/each}
    </table>
{/if}

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

    .table-controls {
        display: flex;
        flex-direction: row;
        justify-content: center;
        gap: 200px;
    }
</style>
