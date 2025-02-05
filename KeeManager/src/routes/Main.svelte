<script lang="ts" context="module">
  export interface LoginEntry {
    type: "login";
    id: string;
    title: string;
    username: string;
    password: string;
    salt: string;
    url: string;
    notes: string;
    last_updated: string;
  }

  export interface CreditCardEntry {
    type: "credit_card";
    id: string;
    title: string;
    card_number: string;
    expiry_date: string;
    cardholder_name: string;
    cvv: string;
    last_updated: string;
  }

  export interface NoteEntry {
    type: "note";
    id: string;
    title: string;
    content: string;
    last_updated: string;
  }

  export interface IdentityEntry {
    type: "identity";
    id: string;
    title: string;
    full_name: string;
    date_of_birth: string;
    nationality: string;
    identification_number: string;
    issue_date: string;
    expiry_date: string;
    issuer: string;
    notes: string;
    last_updated: string;
  }

  export type Entry = LoginEntry | CreditCardEntry | NoteEntry | IdentityEntry;
</script>

<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { push } from "svelte-spa-router";
  import { database, entriesStore, selectedEntryStore, filterStore } from "../reactiveStores";
  import Sidebar from "../components/Sidebar.svelte";
  import Entries from "../components/Entries.svelte";
  import LoginDetail from "../components/EntryDetail/LoginDetail.svelte";
  import CardDetail from "../components/EntryDetail/CardDetail.svelte";
  import NoteDetail from "../components/EntryDetail/NoteDetail.svelte";
  import IdentityDetail from "../components/EntryDetail/IdentityDetail.svelte";
  import { getCurrentWindow, LogicalSize, PhysicalSize } from "@tauri-apps/api/window";
  import { fly } from "svelte/transition";

  let selectedEntry: Entry | null = null;
  let filter: string;
  let entries: Entry[] = [];
  const dbPath = $database.dbPath;
  const masterKey = $database.masterKey;

  const unsubscribeSelectedEntry = selectedEntryStore.subscribe((value) => {
    selectedEntry = value;
  });

  const unsubscribeFilter = filterStore.subscribe((value) => {
    filter = value.filter;
  });

  onMount(async () => {
    await getCurrentWindow().setSize(new LogicalSize(1200, 700));
    await getCurrentWindow().center();
    await getCurrentWindow().setMinSize(new PhysicalSize(1215, 750));
    await getCurrentWindow().setResizable(true);
    await getCurrentWindow().setMaximizable(true);
    try {
      filterStore.set({ filter: "All" });
    } catch (error) {
      alert("Could not get entries");
    }
  });

  onDestroy(() => {
    unsubscribeSelectedEntry();
    unsubscribeFilter();
  });

  //
  // Reactive statement to call updateEntries whenever filter changes
  //
  $: filter, updateEntries();

  async function updateEntries() {
    entries = [];

    if (filter === "Logins" || filter === "All") {
      await getLogins();
    }
    if (filter === "Credit Cards" || filter === "All") {
      await getCreditCards();
    }
    if (filter === "Private Notes" || filter === "All") {
      await getNotes();
    }
    if (filter === "Identities" || filter === "All") {
      await getIdentities();
    }

    // Sorts entries by last_updated in descending order
    entries.sort((a, b) => {
      const dateA = parseDate(a.last_updated);
      const dateB = parseDate(b.last_updated);
      return dateB.getTime() - dateA.getTime();
    });
    entriesStore.set(entries);
  }

  // DD.MM.YY HH:MM:SS format
  function parseDate(dateStr: string): Date {
    const [datePart, timePart] = dateStr.split(" ");
    const [day, month, year] = datePart.split(".").map(Number);
    const [hours, minutes, seconds] = timePart.split(":").map(Number);
    const fullYear = year + 2000;

    return new Date(fullYear, month - 1, day, hours, minutes, seconds);
  }

  async function getLogins() {
    try {
      const result: LoginEntry[] = (await invoke("get_logins", {
        dbPath,
        masterKey,
      })) as LoginEntry[];

      const transformedEntries: Entry[] = result.map((entry) => ({
        ...entry,
        type: "login",
      }));

      entries = [...entries, ...transformedEntries];
    } catch (error) {
      alert("Failed to get logins.");
    }
  }

  async function getCreditCards() {
    try {
      const result: CreditCardEntry[] = (await invoke("get_credit_cards", {
        dbPath,
        masterKey,
      })) as CreditCardEntry[];

      const transformedEntries: Entry[] = result.map((entry) => ({
        ...entry,
        type: "credit_card",
      }));

      entries = [...entries, ...transformedEntries];
    } catch (error) {
      alert("Failed to get credit cards.");
    }
  }

  async function getNotes() {
    try {
      const result: NoteEntry[] = (await invoke("get_notes", {
        dbPath,
        masterKey,
      })) as NoteEntry[];

      const transformedEntries: Entry[] = result.map((entry) => ({
        ...entry,
        type: "note",
      }));

      entries = [...entries, ...transformedEntries];
    } catch (error) {
      alert("Failed to get notes.");
    }
  }

  async function getIdentities() {
    try {
      const result: IdentityEntry[] = (await invoke("get_identities", {
        dbPath,
        masterKey,
      })) as IdentityEntry[];

      const transformedEntries: Entry[] = result.map((entry) => ({
        ...entry,
        type: "identity",
      }));

      entries = [...entries, ...transformedEntries];
    } catch (error) {
      alert("Failed to get identities.");
    }
  }

  function handleEntryClick(event: CustomEvent<Entry>) {
    if (selectedEntry === event.detail) {
      selectedEntryStore.set(null);
    } else {
      selectedEntryStore.set(event.detail);
    }
  }

  async function handleDeleteEntry() {
    if (!selectedEntry) return;
    if (selectedEntry.type === "login") {
      await invoke("delete_login", { dbPath, masterKey, id: selectedEntry.id });
    } else if (selectedEntry.type === "credit_card") {
      await invoke("delete_credit_card", { dbPath, masterKey, id: selectedEntry.id });
    } else if (selectedEntry.type === "note") {
      await invoke("delete_note", { dbPath, masterKey, id: selectedEntry.id });
    } else if (selectedEntry.type === "identity") {
      await invoke("delete_identity", { dbPath, masterKey, id: selectedEntry.id });
    }
    selectedEntryStore.set(null);
    await updateEntries();
  }

  async function handleLogout() {
    await invoke("logout", { dbPath, masterKey });
    database.set({ dbPath: "", masterKey: "" });

    selectedEntryStore.set(null);
    entriesStore.set([]);
    filterStore.set({ filter: "All" });

    push("/");
  }

  async function handleEntryUpdate(event: CustomEvent<Entry>) {
    const entry: Entry = event.detail;
    const selectedID = entry.id;
    const selectedType = entry.type;

    if (entry.type === "login") {
      await invoke("update_login", { dbPath, masterKey, entry });
    } else if (entry.type === "credit_card") {
      await invoke("update_credit_card", { dbPath, masterKey, entry });
    } else if (entry.type === "note") {
      await invoke("update_note", { dbPath, masterKey, entry });
    } else if (entry.type === "identity") {
      await invoke("update_identity", { dbPath, masterKey, entry });
    }

    await updateEntries();

    // Re-select the updated entry
    const updatedEntry =
      entries.find((e) => e.type === selectedType && e.id === selectedID) ||
      null;

    selectedEntryStore.set(updatedEntry);
  }

  async function handleAddEntryFromChild(event: CustomEvent<Entry>) {
    const entry: Entry = event.detail;
    const selectedID = selectedEntry ? selectedEntry.id : null;
    const selectedType = selectedEntry ? selectedEntry.type : null;
    if (entry.type === "login") {
      await invoke("add_login", { dbPath, masterKey, entry });
    } else if (entry.type === "credit_card") {
      await invoke("add_credit_card", { dbPath, masterKey, entry });
    } else if (entry.type === "note") {
      await invoke("add_note", { dbPath, masterKey, entry });
    } else if (entry.type === "identity") {
      await invoke("add_identity", { dbPath, masterKey, entry });
    }
    await updateEntries();

    // Re-select the last selected entry
    if (selectedID && selectedType) {
      const lastEntry =
        entries.find((e) => e.type === selectedType && e.id === selectedID) ||
        null;
      selectedEntryStore.set(lastEntry);
    }
  }
</script>

<div
  class="flex flex-auto h-screen overflow-y-hidden overflow-x-hidden bg-gray-100 rounded-lg"
  in:fly={{ duration: 600 }}
>
  <!-- Sidebar -->
  <div class="w-auto h-full bg-gray-100 m-2">
    <Sidebar on:delete={handleDeleteEntry} on:logout={handleLogout} />
  </div>

  <!-- Main dashboard with header to minimize, maximize, and close buttons -->
  <div class="flex flex-col w-full h-12px">
    <div
      class="draggable flex flex-row w-auto h-auto bg-white shadow-lg shadow-gray-900/80 rounded-xl mt-2 ml-2 mr-1 p-1"
    >
      <div class="w-full">
        <h1 class="flex justify-start mt-1 mb-1 text-3xl font-bold">
          {$filterStore.filter} ({entries.length.toString()})
        </h1>
      </div>

      <div class="no-drag w-auto flex flex-row space-x-2">
        <div class="border-l-2 border-gray-200"></div>
        <button
          type="button"
          class="flex justify-center items-center bg-white rounded-md p-2 text-gray-400 hover:text-gray-500 hover:bg-gray-100 hover:outline hover:outline-2 hover:outline-gray-300"
          on:click={() => invoke("minimize_window")}
        >
          <span class="sr-only">Minimize Window</span>
          <svg
            class="h-4 w-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="gray"
            aria-hidden="true"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M4 12h16"
            />
          </svg>
        </button>

        <button
          type="button"
          class="flex justify-center items-center bg-white rounded-md p-2 text-gray-400 hover:text-gray-500 hover:bg-gray-100 hover:outline hover:outline-2 hover:outline-gray-300"
          on:click={() => invoke("maximize_window")}
        >
          <span class="sr-only">Maximize Window</span>
          <svg
            class="h-4 w-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="gray"
            aria-hidden="true"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M4 4h16v16H4z"
            />
          </svg>
        </button>

        <button
          type="button"
          class="flex justify-center items-center bg-white rounded-md p-2 text-gray-400 hover:text-red-500 hover:bg-red-300 hover:outline hover:outline-2 hover:outline-gray-300 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-red-500"
          on:click={() => invoke("close_window")}
        >
          <span class="sr-only">Close Window</span>
          <svg
            class="h-4 w-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="gray"
            aria-hidden="true"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>
    </div>

    <div
      class="flex flex-row h-full overflow-y-auto overflow-x-hidden mt-2 p-1"
    >
      <div class="w-full mt-2">
        <Entries
          on:entryclick={handleEntryClick}
          on:addentry={handleAddEntryFromChild}
        />
      </div>

      <!-- Side dashboard for entry details -->
      {#if selectedEntry}
        <div class="flex justify-end w-full mt-2 p-1">
          {#if selectedEntry?.type === "login"}
            <LoginDetail
              on:update={handleEntryUpdate}
              on:close={() => selectedEntryStore.set(null)}
            />
          {/if}
          {#if selectedEntry?.type === "credit_card"}
            <CardDetail
              on:update={handleEntryUpdate}
              on:close={() => selectedEntryStore.set(null)}
            />
          {/if}
          {#if selectedEntry?.type === "note"}
            <NoteDetail
              on:update={handleEntryUpdate}
              on:close={() => selectedEntryStore.set(null)}
            />
          {/if}
          {#if selectedEntry?.type === "identity"}
            <IdentityDetail
              on:update={handleEntryUpdate}
              on:close={() => (selectedEntry = null)}
            />
          {/if}
        </div>
      {/if}
    </div>
  </div>
</div>
