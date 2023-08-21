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
            {#if slot["PartiallyFilled"].room != null}
                <p class="room">{rooms[slot["PartiallyFilled"].room].name}</p>
            {:else}
                <p class="room">NO ROOM</p>
            {/if}
        </div>
    </td>
{:else}
    <td class="{htmlClass()} empty">{((slot_index + 1) % max_periods_per_day) == 0 ? 7 : ((slot_index + 1) % max_periods_per_day)}</td>
{/if}

<style>
    td {
        border: 1px solid black;
        margin: 0;
        padding: 10px;
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

    .empty {
        background-color: lightcoral;
        font-size: 2em;
        font-weight: bold;
    }
</style>
