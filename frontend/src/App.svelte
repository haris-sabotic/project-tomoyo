<script>
  // @ts-nocheck

  import Classes from "./lib/tabs/Classes.svelte";
  import Teachers from "./lib/tabs/Teachers.svelte";
  import Rooms from "./lib/tabs/Rooms.svelte";
  import Subjects from "./lib/tabs/Subjects.svelte";
  import Relations from "./lib/tabs/Relations.svelte";
  import Timetable from "./lib/tabs/Timetable.svelte";

  import { socket } from "./lib/store";

  const tabs = {
    Classes: Classes,
    Teachers: Teachers,
    Rooms: Rooms,
    Subjects: Subjects,
    Relations: Relations,
    Timetable: Timetable,
  };
  let currentTab = Classes;

  $: switchTab = (tab) => {
    currentTab = tab;
  };

  let socketLoaded = false;
  socket.addEventListener("open", () => {
    socketLoaded = true;
  });
</script>

<!-- display tab bar -->
<header>
  <nav>
    {#each Object.entries(tabs) as [name, value]}
      <button
        type="button"
        on:click={switchTab(value)}
        class={(currentTab == value ? "button-highlighted" : "") +
          (value == Timetable ? " button-timetable" : "")}
        >{name}
      </button>
    {/each}
  </nav>
</header>

<!-- display current tab -->
{#if socketLoaded}
  {#if currentTab == Classes}
    <Classes />
  {:else if currentTab == Teachers}
    <Teachers />
  {:else if currentTab == Subjects}
    <Subjects />
  {:else if currentTab == Rooms}
    <Rooms />
  {:else if currentTab == Relations}
    <Relations />
  {:else if currentTab == Timetable}
    <Timetable />
  {/if}
{:else}
  <p>Connecting to server...</p>
{/if}

<style>
  header {
    display: flex;
    justify-content: center;
  }

  nav {
    margin-bottom: 50px;
    display: flex;
    gap: 10px;
  }

  nav button {
    padding: 10px 40px;
    background-color: #f9f9f9;
    color: #213547;
    box-shadow: 0 0 2px #213547;
  }

  nav button:hover {
    border-color: #646cff;
  }

  .button-timetable {
    box-shadow: 0 0 5px #646cff;
  }

  .button-highlighted {
    box-shadow: 0 0 10px #213547;
  }
</style>
