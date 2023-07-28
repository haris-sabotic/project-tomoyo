<script>
    // @ts-nocheck

    import { socket, STORE_subjects, STORE_rooms } from "../store";

    let subjectName = "";
    let subjectKind = "";
    let subjects = [];

    STORE_subjects.subscribe((value) => {
        subjects = value;
    });

    let rooms = [];
    STORE_rooms.subscribe((value) => {
        rooms = value;
    });

    function getRoomKinds(roomArray) {
        let kinds = [];
        roomArray.forEach(room => {
            console.log(room);
            room.kinds.split(' ').forEach(kind => kinds.push(kind))
        });

        return kinds
    }

    // rooms unique by their kind, used for the select element
    $: uniqueRoomKinds = getRoomKinds(rooms).reduce(
        (acc, cur) => [...acc.filter((kind) => kind !== cur), cur],
        []
    );

    socket.addEventListener("message", (raw) => {
        let message = JSON.parse(raw.data);

        if (message.kind == "list" && message.tab == "subjects") {
            STORE_subjects.set(message.data);
        }
    });

    function handleImport() {
        socket.send(
            JSON.stringify({
                kind: "import",
                tab: "subjects",
                data: null,
            })
        );
    }

    function handleExport() {
        socket.send(
            JSON.stringify({
                kind: "export",
                tab: "subjects",
                data: subjects,
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
        let name = subjectName.trim();
        let kind = subjectKind.trim();

        if (name.length > 0 && kind.length > 0) {
            socket.send(
                JSON.stringify({
                    kind: "list",
                    tab: "subjects",
                    data: [{ name: name, kind: kind }].concat(subjects),
                })
            );
        }
    }}
>
    <input bind:value={subjectName} type="text" placeholder="Subject name" />

    <select bind:value={subjectKind}>
        {#each uniqueRoomKinds as kind}
            <option value={kind}>
                {kind}
            </option>
        {/each}
    </select>

    <button>OK</button>
</form>

<div class="list-wrapper">
    <div class="list">
        {#each subjects as { name, kind }, key}
            <p>{name}</p>
            <p>{kind}</p>
            <button
                on:click={() => {
                    socket.send(
                        JSON.stringify({
                            kind: "list",
                            tab: "subjects",
                            data: subjects.filter((_, i) => i != key),
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
