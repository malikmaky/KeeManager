<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from "svelte";
  import type { Entry, IdentityEntry } from "../../routes/Main.svelte";
  import { selectedEntryStore } from "../../reactiveStores";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import CopyIcon from "../../static/img/copyIcon.svg";

  let selectedEntry: Entry | null = null;
  let selectedIdentity: IdentityEntry;

  const dispatch = createEventDispatcher();
  const unsubscribeSelectedEntry = selectedEntryStore.subscribe((value) => {
    selectedEntry = value;
  });

  onDestroy(() => {
    unsubscribeSelectedEntry();
  });

  let title: string = "";
  let fullName: string = "";
  let dateOfBirth: string = "";
  let nationality: string = "";
  let identificationNumber: string = "";
  let issueDate: string = "";
  let expiryDate: string = "";
  let issuer: string = "";
  let notes: string = "";
  let lastUpdated: string = "";
  let isEditing: boolean = false;
  let errorMessage = "";

  // Store original values ( before edit )
  let originalTitle: string = "";
  let originalFullName: string = "";
  let originalDateOfBirth: string = "";
  let originalNationality: string = "";
  let originalIdentificationNumber: string = "";
  let originalIssueDate: string = "";
  let originalExpiryDate: string = "";
  let originalIssuer: string = "";
  let originalNotes: string = "";
  let originalLastUpdated: string = "";

  // Load entry details
  onMount(() => {
    if (selectedEntry) {
      selectedIdentity = selectedEntry as IdentityEntry;
      loadOriginalValues();
      errorMessage = "";
    }
  });

  $: {
    if (selectedEntry && !isEditing) {
      selectedIdentity = selectedEntry as IdentityEntry;
      loadOriginalValues();
      errorMessage = "";
    }
  }

  function loadOriginalValues() {
    title = selectedIdentity.title || "";
    fullName = selectedIdentity.full_name || "";
    dateOfBirth = selectedIdentity.date_of_birth || "";
    nationality = selectedIdentity.nationality || "";
    identificationNumber = selectedIdentity.identification_number || "";
    issueDate = selectedIdentity.issue_date || "";
    expiryDate = selectedIdentity.expiry_date || "";
    issuer = selectedIdentity.issuer || "";
    notes = selectedIdentity.notes || "";
    lastUpdated = selectedIdentity.last_updated || "";
    errorMessage = "";

    originalTitle = title;
    originalFullName = fullName;
    originalDateOfBirth = dateOfBirth;
    originalNationality = nationality;
    originalIdentificationNumber = identificationNumber;
    originalIssueDate = issueDate;
    originalExpiryDate = expiryDate;
    originalIssuer = issuer;
    originalNotes = notes;
    originalLastUpdated = lastUpdated;
  }

  function editEntry() {
    isEditing = true;
    errorMessage = "";
  }

  // Validate and save changes if made
  function saveChanges() {
    const fullNamePattern = /^[A-Za-z\s]+$/;
    const identificationNumberPattern = /^[A-Za-z0-9]+$/;

    if (selectedEntry && (hasChanges() || isExpiryChanged())) {
      if (
        !title ||
        !fullName ||
        !dateOfBirth ||
        !nationality ||
        !identificationNumber ||
        !issueDate ||
        !expiryDate ||
        !issuer
      ) {
        errorMessage = "Please fill in all required fields.";
        return;
      }

      if (!fullNamePattern.test(fullName)) {
        errorMessage = "Full name must only contain letters and spaces.";
        return;
      }

      if (!identificationNumberPattern.test(identificationNumber)) {
        errorMessage = "Identification number must be alphanumeric.";
        return;
      }

      selectedIdentity.title = title;
      selectedIdentity.full_name = fullName;
      selectedIdentity.date_of_birth = dateOfBirth;
      selectedIdentity.nationality = nationality;
      selectedIdentity.identification_number = identificationNumber;
      selectedIdentity.issue_date = issueDate;
      selectedIdentity.expiry_date = expiryDate;
      selectedIdentity.issuer = issuer;
      selectedIdentity.notes = notes;
      selectedIdentity.last_updated = new Date().toISOString();

      dispatch("update", selectedEntry);
    }
    isEditing = false;
  }

  function cancelEdit() {
    loadOriginalValues();
    isEditing = false;
  }

  function hasChanges() {
    return (
      title !== originalTitle ||
      fullName !== originalFullName ||
      dateOfBirth !== originalDateOfBirth ||
      nationality !== originalNationality ||
      identificationNumber !== originalIdentificationNumber ||
      issueDate !== originalIssueDate ||
      expiryDate !== originalExpiryDate ||
      issuer !== originalIssuer ||
      notes !== originalNotes
    );
  }

  function isExpiryChanged() {
    return expiryDate !== originalExpiryDate;
  }

  // Updates variables according to new input
  // ( Instead of bind:value )
  function handleInputChange(
    event: Event,
    field:
      | "title"
      | "fullName"
      | "identificationNumber"
      | "notes"
      | "nationality"
      | "issuer"
  ) {
    const input = (event.target as HTMLInputElement).value;
    if (field === "title") title = input;
    else if (field === "fullName") fullName = input;
    else if (field === "identificationNumber") identificationNumber = input;
    else if (field === "notes") notes = input;
    else if (field === "nationality") nationality = input;
    else if (field === "issuer") issuer = input;
  }


  // Updates variables according to new input
  // ( Instead of bind:value )
  function handleDateChange(
    event: Event,
    field: "dateOfBirth" | "issueDate" | "expiryDate"
  ) {
    const value = (event.target as HTMLInputElement).value;
    if (field === "dateOfBirth") dateOfBirth = value;
    else if (field === "issueDate") issueDate = value;
    else if (field === "expiryDate") expiryDate = value;
  }

  function formatDate(dateString: string) {
    if (!dateString) return "";
    const [year, month, day] = dateString.split("-");
    return `${day}.${month}.${year}`;
  }

  const copyToClipboard = async (
    field:
      | "fullName"
      | "dateOfBirth"
      | "identificationNumber"
      | "issueDate"
      | "expiryDate"
      | "issuer"
  ) => {
    try {
      if (field === "fullName") {
        await writeText(fullName);
      } else if (field === "dateOfBirth") {
        await writeText(formatDate(dateOfBirth));
      } else if (field === "identificationNumber") {
        await writeText(identificationNumber);
      } else if (field === "issueDate") {
        await writeText(formatDate(issueDate));
      } else if (field === "expiryDate") {
        await writeText(formatDate(expiryDate));
      } else if (field === "issuer") {
        await writeText(issuer);
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
    {#if errorMessage}
      <div
        class="mb-4 p-2 bg-red-100 text-red-700 border border-red-300 rounded"
      >
        {errorMessage}
      </div>
    {/if}

    <!-- Editable Fields -->
    <div class="w-full">
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

      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <div class="flex flex-row">
          <label class="block text-gray-500 text-sm font-medium mb-2">
            Full Name
          </label>
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("fullName")}
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
            value={fullName}
            on:input={(e) => handleInputChange(e, "fullName")}
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
          />
        {:else}
          <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
            {fullName}
          </div>
        {/if}
      </div>

      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <div class="flex flex-row">
          <label class="block text-gray-500 text-sm font-medium mb-2">
            Date of Birth
          </label>
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("dateOfBirth")}
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
            type="date"
            bind:value={dateOfBirth}
            on:change={(e) => handleDateChange(e, "dateOfBirth")}
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
            readonly={!isEditing}
          />
        {:else}
          <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
            {formatDate(dateOfBirth)}
          </div>
        {/if}
      </div>

      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="block text-gray-500 text-sm font-medium mb-2">
          Nationality
        </label>
        {#if isEditing}
          <input
            type="text"
            value={nationality}
            on:input={(e) => handleInputChange(e, "nationality")}
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
          />
        {:else}
          <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
            {nationality}
          </div>
        {/if}
      </div>

      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <div class="flex flex-row">
          <label class="block text-gray-500 text-sm font-medium mb-2">
            Identification Number
          </label>
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("identificationNumber")}
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
            value={identificationNumber}
            on:input={(e) => handleInputChange(e, "identificationNumber")}
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
          />
        {:else}
          <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
            {identificationNumber}
          </div>
        {/if}
      </div>

      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <div class="flex flex-row">
          <label class="block text-gray-500 text-sm font-medium mb-2">
            Issue Date
          </label>
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("issueDate")}
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
            type="date"
            bind:value={issueDate}
            on:change={(e) => handleDateChange(e, "issueDate")}
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
            readonly={!isEditing}
          />
        {:else}
          <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
            {formatDate(issueDate)}
          </div>
        {/if}
      </div>

      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <div class="flex flex-row">
          <label class="block text-gray-500 text-sm font-medium mb-2">
            Expiry Date
          </label>
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("expiryDate")}
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
            type="date"
            bind:value={expiryDate}
            on:change={(e) => handleDateChange(e, "expiryDate")}
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
            readonly={!isEditing}
          />
        {:else}
          <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
            {formatDate(expiryDate)}
          </div>
        {/if}
      </div>

      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <div class="flex flex-row">
          <label class="block text-gray-500 text-sm font-medium mb-2">
            Issuer
          </label>
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("issuer")}
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
            value={issuer}
            on:input={(e) => handleInputChange(e, "issuer")}
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
          />
        {:else}
          <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
            {issuer}
          </div>
        {/if}
      </div>

      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="block text-gray-500 text-sm font-medium mb-2">Notes</label>
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

      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="block text-gray-500 text-sm font-medium mb-2">
          Last Updated
          </label>
        <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
          {lastUpdated}
        </div>
      </div>
    </div>
  </div>
</div>
