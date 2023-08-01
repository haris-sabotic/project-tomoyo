<script>
    import TimetableAllClasses from "./TimetableAllClasses.svelte";
    import TimetableAllTeachers from "./TimetableAllTeachers.svelte";
    import TimetableClass from "./TimetableClass.svelte";
    import {
        STORE_timetable,
        STORE_teachers,
        STORE_subjects,
        STORE_rooms,
        STORE_classes,
        socket,
    } from "../store";

    let timetable = {};

    STORE_timetable.subscribe((value) => {
        timetable = value;
        console.log(timetable);
    });

    let teachers = [];
    STORE_teachers.subscribe((value) => {
        teachers = value;
    });

    let subjects = [];
    STORE_subjects.subscribe((value) => {
        subjects = value;
    });

    let rooms = [];
    STORE_rooms.subscribe((value) => {
        rooms = value;
    });

    let classes = [];
    STORE_classes.subscribe((value) => {
        classes = value;
    });

    let alpha = 0.97;
    let t0 = 1.0;
    let sa_max = 10000;

    socket.addEventListener("message", (raw) => {
        let message = JSON.parse(raw.data);

        if (message.kind == "timetable" && message.tab == "timetable") {
            STORE_timetable.set(message.data);
        }
    });

    function handleInitialTimetable() {
        socket.send(
            JSON.stringify({
                kind: "initial_timetable",
                tab: "timetable",
                data: null,
            })
        );
    }

    function handlePlay() {
        socket.send(
            JSON.stringify({
                kind: "play",
                tab: "timetable",
                data: {
                    table: timetable.table,
                    alpha: alpha,
                    t0: t0,
                    sa_max: sa_max,
                },
            })
        );
    }

    function handlePause() {
        socket.send(
            JSON.stringify({
                kind: "pause",
                tab: "timetable",
                data: null,
            })
        );
    }

    function handleFillRooms() {
        socket.send(
            JSON.stringify({
                kind: "fill_rooms",
                tab: "timetable",
                data: null,
            })
        );
    }

    function handleImport() {
        socket.send(
            JSON.stringify({
                kind: "import",
                tab: "timetable",
                data: null,
            })
        );
    }
    function handleExport() {
        socket.send(
            JSON.stringify({
                kind: "export",
                tab: "timetable",
                data: null,
            })
        );
    }

    function handleDetailedCost() {
        socket.send(
            JSON.stringify({
                kind: "detailed_cost",
                tab: "timetable",
                data: timetable.table,
            })
        );
    }

    let selectedView = "all-classes";
</script>

<select bind:value={selectedView} on:change={() => {}}>
    <option value="all-classes">ALL CLASSES</option>
    <option value="all-teachers">ALL TEACHERS</option>

    {#each classes as c, key}
        <option value={key}>
            {c}
        </option>
    {/each}
</select>
<button on:click={handleInitialTimetable}>Initial timetable</button>
{#if timetable.table}
    <div class="controls">
        <div class="buttons">
            <div class="play-pause">
                <button on:click={handlePlay}>Play</button>
                <button on:click={handlePause}>Pause</button>
            </div>

            <button on:click={handleFillRooms}>Fill rooms</button>
            <button on:click={handleDetailedCost}>Detailed cost</button>

            <div class="import-export">
                <button on:click={handleImport}>Import</button>
                <button on:click={handleExport}>Export</button>
            </div>
        </div>

        <div class="inputs">
            <div>
                <label for="alpha">Alpha</label>
                <label for="t0">T0</label>
                <label for="sa_max">SA max</label>
                <input
                    type="number"
                    name="alpha"
                    id="alpha"
                    bind:value={alpha}
                    step="0.01"
                />
                <input
                    type="number"
                    name="t0"
                    id="t0"
                    bind:value={t0}
                    step="0.01"
                />
                <input
                    type="number"
                    name="sa_max"
                    id="sa_max"
                    bind:value={sa_max}
                    step="1"
                />
            </div>
        </div>
    </div>

    {#if selectedView == "all-classes"}
        <TimetableAllClasses
            {timetable}
            {classes}
            {subjects}
            {teachers}
            {rooms}
        />
    {:else if selectedView == "all-teachers"}
        <TimetableAllTeachers
            {timetable}
            {classes}
            {subjects}
            {teachers}
            {rooms}
        />
    {:else}
        <TimetableClass
            class_index={selectedView}
            {timetable}
            {subjects}
            {teachers}
            {rooms}
        />
    {/if}
{:else}
    <p>...</p>
{/if}

<style>
    .buttons {
        margin-top: 50px;
        margin-bottom: 20px;
        display: flex;
        gap: 100px;
        flex-direction: row;
        justify-content: center;
    }
    .import-export {
        display: flex;
        gap: 20px;
        justify-content: center;
    }
    .play-pause {
        display: flex;
        gap: 20px;
        justify-content: center;
    }

    .inputs {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;

        margin-bottom: 20px;
    }
    .inputs > div {
        width: 1000px;
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        column-gap: 10px;
    }
</style>
