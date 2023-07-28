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
                data: timetable.table,
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
                data: null,
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
        <input type="text" name="alpha">
        <input type="text" name="t0">
        <input type="text" name="sa_max">
    </div>
</div>

    {#if selectedView == "all-classes"}
        <TimetableAllClasses {timetable} {classes} {subjects} {teachers} {rooms} />
    {:else if selectedView == "all-teachers"}
        <TimetableAllTeachers {timetable} {classes} {subjects} {teachers} {rooms} />
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
        margin: 50px 0;
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
</style>
