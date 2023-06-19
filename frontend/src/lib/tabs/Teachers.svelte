<script>
    // @ts-nocheck

    import { socket, STORE_teachers } from "../store";

    let teacherName = "";
    let teachers = [];

    STORE_teachers.subscribe((value) => {
        teachers = value;
    });

    socket.addEventListener("message", (raw) => {
        let message = JSON.parse(raw.data);

        if (message.kind == "list" && message.tab == "teachers") {
            STORE_teachers.set(message.data);
        }
    });

    function handleImport() {
        socket.send(
            JSON.stringify({
                kind: "import",
                tab: "teachers",
                data: null,
            })
        );
    }

    function handleExport() {
        socket.send(
            JSON.stringify({
                kind: "export",
                tab: "teachers",
                data: teachers,
            })
        );
    }
</script>

<div class="import-export">
    <button on:click={handleImport}>Import</button>
    <button on:click={handleExport}>Export</button>
</div>

<form
    on:submit|preventDefault={() => {
        let name = teacherName.trim();

        if (name.length > 0) {
            socket.send(
                JSON.stringify({
                    kind: "list",
                    tab: "teachers",
                    data: [name].concat(teachers),
                })
            );
        }
    }}
>
    <input bind:value={teacherName} type="text" placeholder="Teacher name" />
    <button>OK</button>
</form>

<div class="list-wrapper">
    <div class="list">
        {#each teachers as teacher, key}
            <p>{teacher}</p>
            <button
                on:click={() => {
                    socket.send(
                        JSON.stringify({
                            kind: "list",
                            tab: "teachers",
                            data: teachers.filter((_, i) => i != key),
                        })
                    );
                }}>DELETE</button
            >
        {/each}
    </div>
</div>

<style>
    .import-export {
        margin-bottom: 50px;
        display: flex;
        gap: 20px;
        justify-content: center;
    }

    form {
        margin-bottom: 20px;
    }

    .list-wrapper {
        display: flex;
        justify-content: center;
    }

    .list {
        display: grid;
        grid-template-columns: 1fr 100px;
        gap: 10px;
    }

    .list p {
        min-width: 200px;
    }
</style>
