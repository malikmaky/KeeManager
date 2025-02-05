<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from "svelte";
  import type { Entry, NoteEntry } from "../../routes/Main.svelte";
  import { selectedEntryStore } from "../../reactiveStores";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import CopyIcon from "../../static/img/copyIcon.svg";

  let selectedEntry: Entry | null = null;
  let selectedNote: NoteEntry;

  const dispatch = createEventDispatcher();
  const unsubscribeSelectedEntry = selectedEntryStore.subscribe((value) => {
    selectedEntry = value;
  });

  onDestroy(() => {
    unsubscribeSelectedEntry();
  });

  let title: string = "";
  let content: string = "";
  let lastUpdated: string = "";
  let isEditing: boolean = false;

  // Store original values ( before edit )
  let originalTitle: string = "";
  let originalContent: string = "";
  let originalLastUpdated: string = "";

  // Load entry details
  onMount(() => {
    if (selectedEntry) {
      selectedNote = selectedEntry as NoteEntry;

      title = selectedNote.title;
      content = selectedNote.content;
      lastUpdated = selectedNote.last_updated;

      originalTitle = selectedNote.title;
      originalContent = selectedNote.content;
      originalLastUpdated = selectedNote.last_updated;
    }
  });

  $: {
    if (selectedEntry) {
      selectedNote = selectedEntry as NoteEntry;

      title = selectedNote.title;
      content = selectedNote.content;
      lastUpdated = selectedNote.last_updated;

      originalTitle = selectedNote.title;
      originalContent = selectedNote.content;
      originalLastUpdated = selectedNote.last_updated;

      cancelEdit();
    }
  }

  function editEntry() {
    isEditing = !isEditing;
  }

  // Save changes if made
  function saveChanges() {
    if (selectedEntry) {
      selectedNote = selectedEntry as NoteEntry;
      if (title !== originalTitle || content !== originalContent) {
        selectedNote.title = title;
        selectedNote.content = content;
        selectedNote.last_updated = new Date().toISOString();
        dispatch("update", selectedEntry);
      }
      isEditing = false;
    }
  }

  function cancelEdit() {
    if (selectedEntry) {
      title = originalTitle;
      content = originalContent;
      lastUpdated = originalLastUpdated;
    }
    isEditing = false;
  }

  // Updates variables according to new input
  // ( Instead of bind:value )
  function handleInputChange(event: Event, field: "title" | "content") {
    const input = (event.target as HTMLInputElement).value;
    if (field === "title") title = input;
    else if (field === "content") content = input;
  }

  const copyToClipboard = async (field: "title" | "note") => {
    try {
      if (field === "title") {
        await writeText(title);
      } else if (field === "note") {
        await writeText(content);
      }
    } catch (error) {
      alert("Failed to copy.");
    }
  };
</script>

<div class="bg-white w-full shadow-lg rounded-lg relative flex flex-col">
  <header
    class="draggable flex flex-row w-full h-auto p-6 border-b border-gray-200"
  >
    <div class="w-full mt-1">
      <h2 class="flex justify-start text-2xl font-bold text-gray-800 truncate">
        {title || "Untitled"}
      </h2>
    </div>

    <div class="no-drag w-auto flex flex-row space-x-1">
      <button
        class="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-800"
        on:click={isEditing ? saveChanges : editEntry}
      >
        {isEditing ? "Save" : "Edit"}
      </button>
      <button
        class="bg-red-600 text-white px-4 py-2 rounded-lg hover:bg-red-800"
        on:click={isEditing ? cancelEdit : () => dispatch("close")}
      >
        {isEditing ? "Cancel" : "Close"}
      </button>
    </div>
  </header>

  <div class="flex-1 overflow-y-auto p-6">
    {#if isEditing || content}
      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <div class="flex flex-row">
          <label class="block text-gray-500 text-sm font-medium mb-2"
            >Title</label
          >
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("title")}
          >
            <img
              class="w-5 h-5 opcaity-50"
              src={CopyIcon}
              alt="Copy To Clipboard"
            />
          </button>
        </div>
        {#if isEditing}
          <input
            type="text"
            value={title}
            on:input={(e) => handleInputChange(e, "title")}
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full min-w-[00px] border border-gray-300"
          />
        {:else}
          <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
            {title}
          </div>
        {/if}
      </div>
    {/if}

    {#if isEditing || content}
      <div class="mb-6 text-wrap">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <div class="flex flex-row">
          <label class="text-wrapblock text-gray-500 text-sm font-medium mb-2"
            >Note</label
          >
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("note")}
          >
            <img
              class="w-5 h-5 opcaity-50"
              src={CopyIcon}
              alt="Copy To Clipboard"
            />
          </button>
        </div>
        {#if isEditing}
          <textarea
            value={content}
            rows="10"
            on:input={(e) => handleInputChange(e, "content")}
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
          />
        {:else}
          <article
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full whitespace-pre-wrap break-words mt-1"
          >
            {content}
          </article>
        {/if}
      </div>
    {/if}

    {#if isEditing || lastUpdated}
      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="block text-gray-500 text-sm font-medium mb-2"
          >Last Updated</label
        >
        <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
          {lastUpdated || "N/A"}
        </div>
      </div>
    {/if}
  </div>
</div>
