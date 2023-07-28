<script>
    // @ts-nocheck

    import { socket, STORE_rooms } from "../store";

    let roomName = "";
    let roomKinds = "";
    let rooms = [];

    STORE_rooms.subscribe((value) => {
        rooms = value;
    });

    socket.addEventListener("message", (raw) => {
        let message = JSON.parse(raw.data);

        if (message.kind == "list" && message.tab == "rooms") {
            STORE_rooms.set(message.data);
        }
    });

    function handleImport() {
        socket.send(
            JSON.stringify({
                kind: "import",
                tab: "rooms",
                data: null,
            })
        );
    }

    function handleExport() {
        socket.send(
            JSON.stringify({
                kind: "export",
                tab: "rooms",
                data: rooms,
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
        let name = roomName.trim();
        let kinds = roomKinds.trim();

        if (name.length > 0 && kinds.length > 0) {
            socket.send(
                JSON.stringify({
                    kind: "list",
                    tab: "rooms",
                    data: [{ name: name, kinds: kinds }].concat(rooms),
                })
            );
        }
    }}
>
    <input bind:value={roomName} type="text" placeholder="Room name" />
    <input bind:value={roomKinds} type="text" placeholder="Room kind" />
    <button>OK</button>
</form>

<div class="list-wrapper">
    <div class="list">
        {#each rooms as { name, kinds }, key}
            <p>{name}</p>
            <p>{kinds}</p>
            <button
                on:click={() => {
                    socket.send(
                        JSON.stringify({
                            kind: "list",
                            tab: "rooms",
                            data: rooms.filter((_, i) => i != key),
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
        grid-template-columns: 1fr 1fr 100px;
        gap: 10px;
    }

    .list p {
        min-width: 200px;
    }
</style>
