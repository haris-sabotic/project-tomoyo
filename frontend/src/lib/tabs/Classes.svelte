<script>
    // @ts-nocheck

    import { socket, STORE_classes } from "../store";

    let className = "";
    let classes = [];

    STORE_classes.subscribe((value) => {
        classes = value;
    });

    socket.addEventListener("message", (raw) => {
        let message = JSON.parse(raw.data);

        if (message.kind == "list" && message.tab == "classes") {
            STORE_classes.set(message.data);
        }
    });

    function handleImport() {
        socket.send(
            JSON.stringify({
                kind: "import",
                tab: "classes",
                data: null,
            })
        );
    }

    function handleExport() {
        socket.send(
            JSON.stringify({
                kind: "export",
                tab: "classes",
                data: classes,
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
        let name = className.trim();

        if (name.length > 0) {
            socket.send(
                JSON.stringify({
                    kind: "list",
                    tab: "classes",
                    data: [name].concat(classes),
                })
            );
        }
    }}
>
    <input bind:value={className} type="text" placeholder="Class name" />
    <button>OK</button>
</form>

<div class="list-wrapper">
    <div class="list">
        {#each classes as c, key}
            <p>{c}</p>
            <button
                on:click={() => {
                    socket.send(
                        JSON.stringify({
                            kind: "list",
                            tab: "classes",
                            data: classes.filter((_, i) => i != key),
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
