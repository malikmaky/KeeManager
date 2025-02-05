<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from "svelte";
  import type { Entry, LoginEntry } from "../../routes/Main.svelte";
  import { selectedEntryStore } from "../../reactiveStores";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import CopyIcon from "../../static/img/copyIcon.svg";
  import ShowPasswordIcon from "../../static/img/openEyeIcon.svg";
  import HidePasswordIcon from "../../static/img/closedEyeIcon.svg";

  let selectedEntry: Entry | null = null;
  let selectedLogin: LoginEntry;

  const dispatch = createEventDispatcher();
  const unsubscribeSelectedEntry = selectedEntryStore.subscribe((value) => {
    selectedEntry = value;
  });

  onDestroy(() => {
    unsubscribeSelectedEntry();
  });

  let title: string = "";
  let username: string = "";
  let password: string = "";
  let url: string = "";
  let notes: string = "";
  let lastUpdated: string = "";
  let showPassword = false;
  let isEditing: boolean = false;

  // Store original values ( before edit )
  let originalTitle: string = "";
  let originalUsername: string = "";
  let originalPassword: string = "";
  let originalUrl: string = "";
  let originalNotes: string = "";
  let originalLastUpdated: string = "";

  // Load entry details
  onMount(() => {
    if (selectedEntry) {
      selectedLogin = selectedEntry as LoginEntry;

      title = selectedLogin.title;
      username = selectedLogin.username;
      password = selectedLogin.password;
      url = selectedLogin.url;
      notes = selectedLogin.notes;
      lastUpdated = selectedLogin.last_updated;

      originalTitle = selectedLogin.title;
      originalUsername = selectedLogin.username;
      originalPassword = selectedLogin.password;
      originalUrl = selectedLogin.url;
      originalNotes = selectedLogin.notes;
      originalLastUpdated = selectedLogin.last_updated;
    }
  });

  $: {
    if (selectedEntry) {
      selectedLogin = selectedEntry as LoginEntry;

      title = selectedLogin.title;
      username = selectedLogin.username;
      password = selectedLogin.password;
      url = selectedLogin.url;
      notes = selectedLogin.notes;
      lastUpdated = selectedLogin.last_updated;

      originalTitle = selectedLogin.title;
      originalUsername = selectedLogin.username;
      originalPassword = selectedLogin.password;
      originalUrl = selectedLogin.url;
      originalNotes = selectedLogin.notes;
      originalLastUpdated = selectedLogin.last_updated;

      cancelEdit();
    }
  }

  async function editEntry() {
    isEditing = !isEditing;
  }

  // Save changes if made
  function saveChanges() {
    if (selectedEntry) {
      selectedLogin = selectedEntry as LoginEntry;
      if (
        title !== originalTitle ||
        username !== originalUsername ||
        password !== originalPassword ||
        url !== originalUrl ||
        notes !== originalNotes
      ) {
        selectedLogin.title = title;
        selectedLogin.username = username;
        selectedLogin.password = password;
        selectedLogin.url = url;
        selectedLogin.notes = notes;
        selectedLogin.last_updated = new Date().toISOString();

        selectedEntry = selectedLogin;
        dispatch("update", selectedEntry);
      }
      isEditing = false;
    }
  }

  function cancelEdit() {
    if (selectedEntry) {
      title = originalTitle;
      username = originalUsername;
      password = originalPassword;
      url = originalUrl;
      notes = originalNotes;
      lastUpdated = originalLastUpdated;
    }
    isEditing = false;
  }

  // Updates variables according to new input
  // ( Instead of bind:value )
  function handleInputChange(
    event: Event,
    field: "title" | "username" | "password" | "url" | "notes"
  ) {
    const input = (event.target as HTMLInputElement).value;
    if (field === "title") title = input;
    else if (field === "username") username = input;
    else if (field === "password") password = input;
    else if (field === "url") url = input;
    else if (field === "notes") notes = input;
  }

  function toggleShowPassword() {
    showPassword = !showPassword;
  }

  function getObscuredPassword() {
    if (password.length < 6) {
      return "•".repeat(6);
    }
    return "•".repeat(password.length);
  }

  const copyToClipboard = async (field: "username" | "password" | "url") => {
    try {
      if (field === "username") {
        await writeText(username);
      } else if (field === "password") {
        await writeText(password);
      } else if (field === "url") {
        await writeText(url);
      }
    } catch (error) {
      alert("Failed to copy.");
    }
  };
</script>

<div class="bg-white shadow-lg rounded-lg w-full relative flex flex-col">
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
    {#if isEditing}
      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="block text-gray-500 text-sm font-medium mb-2">
        Title
        </label>
        <input
          type="text"
          value={title}
          on:input={(e) => handleInputChange(e, "title")}
          class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
        />
      </div>
    {/if}

    {#if isEditing || username}
      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <div class="flex flex-row">
          <label class="block text-gray-500 text-sm font-medium mb-2">
            Username
            </label>
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("username")}
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
            value={username}
            on:input={(e) => handleInputChange(e, "username")}
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
          />
        {:else}
          <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
            {username}
          </div>
        {/if}
      </div>
    {/if}

    {#if isEditing || password}
      <div class="flex-1 mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <div class="flex flex-row">
          <label class="block text-gray-500 text-sm font-medium mb-2">
            Password
            </label>
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("password")}
          >
            <img
              class="w-5 h-5 opcaity-50"
              src={CopyIcon}
              alt="Copy To Clipboard"
            />
          </button>
        </div>
        {#if isEditing}
          <div class="relative">
            <input
              type={showPassword ? "text" : "password"}
              value={password}
              on:input={(e) => handleInputChange(e, "password")}
              class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
            />
            <button
              type="button"
              class="absolute right-2 top-1/2 transform -translate-y-1/2"
              on:click={toggleShowPassword}
            >
              <img
                class="w-6 h-6"
                src={showPassword
                  ? ShowPasswordIcon
                  : HidePasswordIcon }
                alt={showPassword ? "Hide password" : "Show password"}
              />
            </button>
          </div>
        {:else}
          <div class="relative">
            <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
              {showPassword ? password : getObscuredPassword()}
            </div>
            <button
              type="button"
              class="absolute right-2 top-1/2 transform -translate-y-1/2"
              on:click={toggleShowPassword}
            >
              <img
                class="w-6 h-6"
                src={showPassword
                  ? ShowPasswordIcon
                  : HidePasswordIcon }
                alt={showPassword ? "Hide password" : "Show password"}
              />
            </button>
          </div>
        {/if}
      </div>
    {/if}

    {#if isEditing || url}
      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <div class="flex flex-row">
          <label class="block text-gray-500 text-sm font-medium mb-2">URL</label
          >
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("url")}
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
            type="url"
            value={url}
            on:input={(e) => handleInputChange(e, "url")}
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
          />
        {:else}
          <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
            {url}
          </div>
        {/if}
      </div>
    {/if}

    {#if isEditing || notes}
      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="block text-gray-500 text-sm font-medium mb-2">Notes</label
        >
        {#if isEditing}
          <textarea
            value={notes}
            on:input={(e) => handleInputChange(e, "notes")}
            rows="3"
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
          />
        {:else}
          <article
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded whitespace-pre-wrap overflow-hidden break-words"
          >
            {notes}
          </article>
        {/if}
      </div>
    {/if}

    {#if isEditing || lastUpdated}
      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="block text-gray-500 text-sm font-medium mb-2">
          Last Updated
          </label>
        <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
          {lastUpdated || "N/A"}
        </div>
      </div>
    {/if}
  </div>
</div>
