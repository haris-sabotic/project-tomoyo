<script>
    export let day_separators;
    export let slot_index;
    export let slot;
    export let subjects;
    export let classes;
    export let rooms;
    export let max_periods_per_day;

    function htmlClass() {
        if (day_separators) {
            return slot_index % max_periods_per_day == 0
                ? "day-begin"
                : (slot_index + 1) % max_periods_per_day == 0
                ? "day-end"
                : "";
        }

        return "";
    }
</script>

{#if slot["PartiallyFilled"]}
    <td class={htmlClass()}>
        <div class="content">
            <p class="subject">
                {subjects[slot["PartiallyFilled"].subject].name}
            </p>
            <p class="class">
                {classes[slot["PartiallyFilled"].class]}
            </p>
        </div>
    </td>
{:else if slot["Filled"]}
    <td class={htmlClass()}>
        <div class="content">
            <p class="subject">
                {subjects[slot["Filled"].subject].name}
            </p>
            <p class="class">
                {classes[slot["Filled"].class]}
            </p>
            <p class="room">{rooms[slot["Filled"].room].name}</p>
        </div>
    </td>
{:else}
    <td class="{htmlClass()} empty"></td>
{/if}

<style>
    td {
        border: 1px solid black;
        margin: 0;
        padding: 10px;
    }

    .empty {
        background-color: lightsteelblue;
    }

    .content {
        margin: 0;
        display: grid;
        grid-template-columns: 1fr;
    }

    .subject {
        margin: 0;
        font-weight: bold;
    }

    .teacher {
        margin: 0;
        margin-top: 10px;
        font-style: italic;
    }

    .room {
        margin: 0;
        margin-top: 10px;
        text-align: end;
    }

    .day-begin {
        border-left: 5px solid red;
    }
    .day-end {
        border-right: 5px solid red;
    }
</style>
