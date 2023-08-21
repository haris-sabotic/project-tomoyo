<script>
    export let day_separators;
    export let slot_index;
    export let slot;
    export let subjects;
    export let teachers;
    export let rooms;
    export let max_periods_per_day;

    export let selected;
    export let onClick;

    $: htmlClass = () => {
        let result = "";

        if (day_separators) {
            result +=
                slot_index % max_periods_per_day == 0
                    ? "day-begin"
                    : (slot_index + 1) % max_periods_per_day == 0
                    ? "day-end"
                    : "";
        }

        if (selected) {
            result += " selected";
        }

        return result;
    };
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
{#if slot["Single"]}
    {#if slot["Single"]["PartiallyFilled"]}
        <td class={htmlClass()} on:click={onClick}>
            <div class="content {(slot["Single"]["PartiallyFilled"].room != null) ? "" : " no-room"}">
                <p class="subject">
                    {subjects[slot["Single"]["PartiallyFilled"].subject].name}
                </p>
                <p class="teacher">
                    {teachers[slot["Single"]["PartiallyFilled"].teacher]}
                </p>
                {#if slot["Single"]["PartiallyFilled"].room != null}
                    <p class="room">
                        {rooms[slot["Single"]["PartiallyFilled"].room].name}
                    </p>
                {:else}
                    <p>NO ROOM</p>
                {/if}
            </div>
        </td>
    {:else}
        <td class={htmlClass()} on:click={onClick} />
    {/if}
{:else}
    <td
        class="split {htmlClass()} before-{slot['Double'][
            'before'
        ]} after-{slot['Double']['after']}"
        on:click={onClick}
    >
        <div class="split-content">
            {#if slot["Double"]["first"]["PartiallyFilled"]}
                <div class="content {(slot["Double"]["first"]["PartiallyFilled"].room != null) ? "" : " no-room"}">
                    <p class="subject">
                        {subjects[
                            slot["Double"]["first"]["PartiallyFilled"].subject
                        ].name}
                    </p>
                    <p class="teacher">
                        {teachers[
                            slot["Double"]["first"]["PartiallyFilled"].teacher
                        ]}
                    </p>
                    {#if slot["Double"]["first"]["PartiallyFilled"].room != null}
                        <p class="room">
                            {rooms[
                                slot["Double"]["first"]["PartiallyFilled"].room
                            ].name}
                        </p>
                    {:else}
                        <p>NO ROOM</p>
                    {/if}
                </div>
            {:else}
                <p />
            {/if}

            {#if slot["Double"]["second"]["PartiallyFilled"]}
                <div class="content {(slot["Double"]["second"]["PartiallyFilled"].room != null) ? "" : " no-room"}">
                    <p class="subject">
                        {subjects[
                            slot["Double"]["second"]["PartiallyFilled"].subject
                        ].name}
                    </p>
                    <p class="teacher">
                        {teachers[
                            slot["Double"]["second"]["PartiallyFilled"].teacher
                        ]}
                    </p>
                    {#if slot["Double"]["second"]["PartiallyFilled"].room != null}
                        <p class="room">
                            {rooms[
                                slot["Double"]["second"]["PartiallyFilled"].room
                            ].name}
                        </p>
                    {:else}
                        <p>NO ROOM</p>
                    {/if}
                </div>
            {:else}
                <p />
            {/if}
        </div>
    </td>
{/if}

<style>
    .no-room {
        background-color: lightblue;
    }

    td {
        border: 1px solid black;
        margin: 0;
        padding: 10px;
    }

    td:hover {
        background-color: rgb(230, 230, 230);
    }

    .selected {
        background-color: rgb(200, 200, 200) !important;
    }

    .content {
        margin: 0;
        display: grid;
        grid-template-columns: 1fr;
    }

    .split {
        margin: 0;
        padding: 0;
    }
    .split-content {
        margin: 0;
        display: grid;
        grid-template-rows: 1fr 1fr;
    }

    .split-content > * {
        margin: 0;
        padding: 10px;
    }

    .split-content > p {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .split-content > *:first-child {
        border-bottom: 1px solid gray;
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
