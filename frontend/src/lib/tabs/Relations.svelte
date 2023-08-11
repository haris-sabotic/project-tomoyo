<script>
    // @ts-nocheck

    import {
        socket,
        STORE_relations,
        STORE_teachers,
        STORE_subjects,
        STORE_classes,
    } from "../store";

    let relationShift = 1;
    let relationTeacher = "";
    let relationSubject = "";
    let relationClasses = "";
    let relationPerWeek = 0;
    let relations = [];

    STORE_relations.subscribe((value) => {
        relations = value;
        console.log(relations);
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

    let editing = null;

    function convertRelationToIndices(relation) {
        return {
            shift: relation.shift,
            teacher: teachers.findIndex(
                (teacher) => teacher == relation.teacher
            ),
            subject: subjects.findIndex((subject) => {
                return subject.name == relation.subject;
            }),
            class_: classes.findIndex((class_) => class_ == relation.class_),
            perWeekFirst: relation.perWeekFirst,
            perWeekSecond: relation.perWeekSecond,
        };
    }

    function convertRelationFromIndices(relation) {
        return {
            shift: relation.shift,
            teacher: teachers[relation.teacher],
            subject: subjects[relation.subject].name,
            class_: classes[relation.class_],
            perWeekFirst: relation.perWeekFirst,
            perWeekSecond: relation.perWeekSecond,
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
        let shift = parseInt(relationShift.trim());
        let teacher = relationTeacher.trim();
        let subject = relationSubject.trim();
        let classes = relationClasses.trim();
        let perWeek = relationPerWeek.toString();

        if (teacher.length > 0 && subject.length > 0 && classes.length > 0) {
            let data = [];
            classes.split(",").forEach((c) => {
                let perWeekFirst = perWeek;
                let perWeekSecond = null;
                console.log(perWeek);
                if (perWeek.split("/").length == 2) {
                    perWeekFirst = perWeek.split("/")[0];
                    perWeekSecond = perWeek.split("/")[1];
                }

                if (editing == null) {
                    data = [
                        {
                            shift: shift,
                            teacher: teacher,
                            subject: subject,
                            class_: c,
                            perWeekFirst: parseInt(perWeekFirst),
                            perWeekSecond: parseInt(perWeekSecond),
                        },
                    ].concat(data);
                } else {
                    relations[editing] = {
                        shift: shift,
                        teacher: teacher,
                        subject: subject,
                        class_: c,
                        perWeekFirst: parseInt(perWeekFirst),
                        perWeekSecond: parseInt(perWeekSecond),
                    };
                }
            });

            socket.send(
                JSON.stringify({
                    kind: "list",
                    tab: "relations",
                    data: data.concat(relations).map(convertRelationToIndices),
                })
            );

            console.log({
                kind: "list",
                tab: "relations",
                data: data.concat(relations).map(convertRelationToIndices),
            });

            editing = null;
        }
    }}
>
    <select bind:value={relationShift}>
        <option value="1">FIRST</option>
        <option value="2">SECOND</option>
    </select>

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
        type="text"
        placeholder="Per week"
        id="per-week"
    />
    <button>OK</button>
</form>

<div class="list-wrapper">
    <div class="list">
        {#each relations as { shift, teacher, subject, class_, perWeekFirst, perWeekSecond }, key}
            <p>{shift}</p>
            <p>{teacher}</p>
            <p>{subject}</p>
            <p>{class_}</p>
            <p>
                {perWeekSecond == null
                    ? perWeekFirst
                    : perWeekFirst + " / " + perWeekSecond}
            </p>
            <button
                on:click={() => {
                    socket.send(
                        JSON.stringify({
                            kind: "list",
                            tab: "relations",
                            data: relations
                                .filter((_, i) => i != key)
                                .map(convertRelationToIndices),
                        })
                    );
                    editing = null;
                }}>DELETE</button
            >

            <button
                on:click={() => {
                    editing = key;

                    relationShift = relations[key].shift.toString();
                    relationTeacher = relations[key].teacher;
                    relationSubject = relations[key].subject;
                    relationClasses = relations[key].class_;
                    if (relations[key].perWeekSecond == null) {
                        relationPerWeek = relations[key].perWeekFirst;
                    } else {
                        relationPerWeek =
                            relations[key].perWeekFirst +
                            "/" +
                            relations[key].perWeekSecond;
                    }
                }}>EDIT</button
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
        grid-template-columns: 1fr 1fr 1fr 1fr 1fr 100px 100px;
        gap: 10px;
    }

    .list p {
        min-width: 200px;
    }
</style>
