<script>
    export let day_separators;
    export let slot_index;
    export let slot;
    export let subjects;
    export let teachers;
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

{#if slot["Single"]}
    {#if slot["Single"]["PartiallyFilled"]}
        <td class={htmlClass()}>
            <div class="content">
                <p class="subject">
                    {subjects[slot["Single"]["PartiallyFilled"].subject].name}
                </p>
                <p class="teacher">
                    {teachers[slot["Single"]["PartiallyFilled"].teacher]}
                </p>
            </div>
        </td>
    {:else if slot["Single"]["Filled"]}
        <td class={htmlClass()}>
            <div class="content">
                <p class="subject">
                    {subjects[slot["Single"]["Filled"].subject].name}
                </p>
                <p class="teacher">
                    {teachers[slot["Single"]["Filled"].teacher]}
                </p>
                <p class="room">{rooms[slot["Single"]["Filled"].room].name}</p>
            </div>
        </td>
    {:else}
        <td class={htmlClass()}><em>{slot["Single"]}</em></td>
    {/if}
{:else}
    <td class="split {htmlClass()}">
        <div class="split-content">
            {#if slot["Double"]["first"]["PartiallyFilled"]}
                <div class="content">
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
                </div>
            {:else if slot["Double"]["first"]["Filled"]}
                <div class="content">
                    <p class="subject">
                        {subjects[slot["Double"]["first"]["Filled"].subject]
                            .name}
                    </p>
                    <p class="teacher">
                        {teachers[slot["Double"]["first"]["Filled"].teacher]}
                    </p>
                    <p class="room">
                        {rooms[slot["Double"]["first"]["Filled"].room].name}
                    </p>
                </div>
            {:else}
                <p><em>{slot["Double"]["first"]}</em></p>
            {/if}

            {#if slot["Double"]["second"]["PartiallyFilled"]}
                <div class="content">
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
                </div>
            {:else if slot["Double"]["second"]["Filled"]}
                <div class="content">
                    <p class="subject">
                        {subjects[slot["Double"]["second"]["Filled"].subject]
                            .name}
                    </p>
                    <p class="teacher">
                        {teachers[slot["Double"]["second"]["Filled"].teacher]}
                    </p>
                    <p class="room">
                        {rooms[slot["Double"]["second"]["Filled"].room].name}
                    </p>
                </div>
            {:else}
                <p><em>{slot["Double"]["second"]}</em></p>
            {/if}
        </div>
    </td>
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

    .split {
        margin: 0;
        padding: 0;
    }
    .split-content {
        margin: 0;
        display: grid;
        grid-template-rows: 1fr 1fr;
    }

    .split-content>* {
        margin: 0;
        padding: 10px;
    }
    
    .split-content>p {
        display: flex;
        justify-content: center;
        align-items: center;
    }
    
    .split-content>*:first-child {
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
