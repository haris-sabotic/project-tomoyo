<script>
    // @ts-nocheck

    import {
        socket,
        STORE_relations,
        STORE_teachers,
        STORE_subjects,
        STORE_classes,
    } from "../store";

    let relationTeacher = "";
    let relationSubject = "";
    let relationClasses = "";
    let relationPerWeek = 0;
    let relations = [];

    STORE_relations.subscribe((value) => {
        relations = value;
    });

    let teachers = [];
    STORE_teachers.subscribe((value) => {
        teachers = value;
    });

    let subjects = [];
    STORE_subjects.subscribe((value) => {
        subjects = value;
    });

    let classes = [];
    STORE_classes.subscribe((value) => {
        classes = value;
    });

    socket.addEventListener("message", (raw) => {
        let message = JSON.parse(raw.data);

        if (message.kind == "list" && message.tab == "relations") {
            STORE_relations.set(message.data.map(convertRelationFromIndices));
        }
    });

    function convertRelationToIndices(relation) {
        return {
            teacher: teachers.findIndex(
                (teacher) => teacher == relation.teacher
            ),
            subject: subjects.findIndex((subject) => {
                return subject.name == relation.subject;
            }),
            class_: classes.findIndex((class_) => class_ == relation.class_),
            perWeek: relation.perWeek,
        };
    }

    function convertRelationFromIndices(relation) {
        return {
            teacher: teachers[relation.teacher],
            subject: subjects[relation.subject].name,
            class_: classes[relation.class_],
            perWeek: relation.perWeek,
        };
    }

    function handleImport() {
        socket.send(
            JSON.stringify({
                kind: "import",
                tab: "relations",
                data: null,
            })
        );
    }

    function handleExport() {
        socket.send(
            JSON.stringify({
                kind: "export",
                tab: "relations",
                data: relations.map(convertRelationToIndices),
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
        let teacher = relationTeacher.trim();
        let subject = relationSubject.trim();
        let classes = relationClasses.trim();
        let perWeek = relationPerWeek;

        if (teacher.length > 0 && subject.length > 0 && classes.length > 0) {
            let data = [];
            classes.split(",").forEach((c) => {
                data = [
                    {
                        teacher: teacher,
                        subject: subject,
                        class_: c,
                        perWeek: perWeek,
                    },
                ].concat(data);
            });

            socket.send(
                JSON.stringify({
                    kind: "list",
                    tab: "relations",
                    data: data.concat(relations).map(convertRelationToIndices),
                })
            );
        }
    }}
>
    <select bind:value={relationTeacher}>
        {#each teachers as teacher}
            <option value={teacher}>
                {teacher}
            </option>
        {/each}
    </select>

    <select bind:value={relationSubject}>
        {#each subjects as subject}
            <option value={subject.name}>
                {subject.name}
            </option>
        {/each}
    </select>

    <input bind:value={relationClasses} type="text" placeholder="Classes" />
    <input
        bind:value={relationPerWeek}
        type="number"
        placeholder="Per week"
        id="per-week"
    />
    <button>OK</button>
</form>

<div class="list-wrapper">
    <div class="list">
        {#each relations as { teacher, subject, class_, perWeek }, key}
            <p>{teacher}</p>
            <p>{subject}</p>
            <p>{class_}</p>
            <p>{perWeek}</p>
            <button
                on:click={() => {
                    socket.send(
                        JSON.stringify({
                            kind: "list",
                            tab: "relations",
                            data: relations.filter((_, i) => i != key).map(convertRelationToIndices),
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

    #per-week {
        width: 60px;
    }

    .list-wrapper {
        display: flex;
        justify-content: center;
    }

    .list {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr 1fr 100px;
        gap: 10px;
    }

    .list p {
        min-width: 200px;
    }
</style>
