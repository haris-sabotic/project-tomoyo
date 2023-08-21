<script>
    import TimetableClass from "./TimetableClass.svelte";

    export let timetable;
    export let classes;
    export let subjects;
    export let teachers;
    export let rooms;
</script>

<h1>FIRST SHIFT</h1>

<div class="classes">
    {#each classes as c, key}
        <div class="class">
            <h2>{c}</h2>
            <div class="table">
                <TimetableClass
                    class_index={key}
                    shift={1}
                    {timetable}
                    {subjects}
                    {teachers}
                    {rooms}
                />
            </div>
        </div>
    {/each}
</div>

<h1>SECOND SHIFT</h1>

<div class="classes">
    {#each classes as c, key}
        <div class="class">
            <h2>{c}</h2>
            <div class="table">
                <TimetableClass
                    class_index={key}
                    shift={2}
                    {timetable}
                    {subjects}
                    {teachers}
                    {rooms}
                />
            </div>
        </div>
    {/each}
</div>

<style>
    .classes {
        display: grid;
        grid-template-columns: 1fr 1fr;
        column-gap: 50px;
        row-gap: 50px;
    }
</style>
