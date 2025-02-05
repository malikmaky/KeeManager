<script lang="ts">
  import type { IdentityEntry } from "../../routes/Main.svelte"; // Adjust path as needed

  export let showModal = false;
  export let onSave: (entry: IdentityEntry) => void;
  export let onClose: () => void;

  let title = "";
  let full_name = "";
  let date_of_birth = "";
  let nationality = "";
  let identification_number = "";
  let issue_date = "";
  let expiry_date = "";
  let issuer = "";
  let notes = "";
  let errorMessage = "";

  // Reactive statement to clear form fields when the modal is shown
  $: if (showModal) {
    title = "";
    full_name = "";
    date_of_birth = "";
    nationality = "";
    identification_number = "";
    issue_date = "";
    expiry_date = "";
    issuer = "";
    notes = "";
    errorMessage = "";
  }

  function handleSave() {
    errorMessage = "";

    // Regular expressions for validation
    const fullNamePattern = /^[A-Za-z\s]+$/;
    const idNumberPattern = /^[\w\s-]+$/;

    // Field validation
    if (
      !title ||
      !full_name ||
      !date_of_birth ||
      !nationality ||
      !identification_number ||
      !issue_date ||
      !expiry_date ||
      !issuer
    ) {
      errorMessage = "Please fill in all required fields.";
      return;
    }

    // Validate full name
    if (!fullNamePattern.test(full_name)) {
      errorMessage = "Full name must only contain letters and spaces.";
      return;
    }

    // Validate identification number
    if (!idNumberPattern.test(identification_number)) {
      errorMessage = "Identification number must be valid.";
      return;
    }

    const newEntry: IdentityEntry = {
      type: "identity",
      id: "", 
      title,
      full_name,
      date_of_birth,
      nationality,
      identification_number,
      issue_date,
      expiry_date,
      issuer,
      notes,
      last_updated: "",
    };

    onSave(newEntry);
    onClose();
  }
</script>

{#if showModal}
  <div
    class="no-drag fixed inset-0 bg-gray-50/0 flex items-center justify-center z-50"
  >
    <div
      class="bg-white p-6 rounded-lg shadow-lg w-full max-w-lg max-h-full overflow-y-auto"
    >
      <h2 class="draggable text-2xl font-extrabold mb-4 text-gray-800">
        Add New Identity
      </h2>

      {#if errorMessage}
        <div
          class="mb-4 p-2 bg-red-100 text-red-700 border border-red-300 rounded"
        >
          {errorMessage}
        </div>
      {/if}

      <div class="bg-inherit">
        <div class="bg-inherit relative mb-6">
          <input
            type="text"
            id="title"
            name="title"
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="Title"
            bind:value={title}
          />
          <label
            for="title"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >Title</label
          >
        </div>

        <div class="bg-inherit relative mb-6">
          <input
            type="text"
            id="full_name"
            name="full_name"
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="Full Name"
            bind:value={full_name}
          />
          <label
            for="full_name"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >Full Name</label
          >
        </div>

        <div class="bg-inherit relative mb-6">
          <input
            type="date"
            id="date_of_birth"
            name="date_of_birth"
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="Date of Birth"
            bind:value={date_of_birth}
          />
          <label
            for="date_of_birth"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >Date of Birth</label
          >
        </div>

        <div class="bg-inherit relative mb-6">
          <input
            type="text"
            id="nationality"
            name="nationality"
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="Nationality"
            bind:value={nationality}
          />
          <label
            for="nationality"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >Nationality</label
          >
        </div>

        <div class="bg-inherit relative mb-6">
          <input
            type="text"
            id="identification_number"
            name="identification_number"
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="Identification Number"
            bind:value={identification_number}
          />
          <label
            for="identification_number"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >Identification Number</label
          >
        </div>

        <div class="bg-inherit relative mb-6">
          <input
            type="date"
            id="issue_date"
            name="issue_date"
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="Issue Date"
            bind:value={issue_date}
          />
          <label
            for="issue_date"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >Issue Date</label
          >
        </div>

        <div class="bg-inherit relative mb-6">
          <input
            type="date"
            id="expiry_date"
            name="expiry_date"
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="Expiry Date"
            bind:value={expiry_date}
          />
          <label
            for="expiry_date"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >Expiry Date</label
          >
        </div>

        <div class="bg-inherit relative mb-6">
          <input
            type="text"
            id="issuer"
            name="issuer"
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="Issuer"
            bind:value={issuer}
          />
          <label
            for="issuer"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >Issuer</label
          >
        </div>

        <div class="bg-inherit relative mb-6">
          <textarea
            id="notes"
            name="notes"
            class="peer bg-transparent h-24 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="Notes"
            bind:value={notes}
          ></textarea>
          <label
            for="notes"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >Notes</label
          >
        </div>
      </div>

      <div class="flex gap-4">
        <button
          class="w-full bg-blue-700 hover:bg-blue-800 text-white font-bold py-2 px-4 mb-3 rounded-lg transition-transform transform hover:scale-105 hover:shadow-lg active:scale-95 active:shadow-md focus:ring-2 focus:ring-blue-500 focus:outline-none"
          on:click={handleSave}
        >
          Save
        </button>
        <button
          class="w-full bg-gray-200 hover:bg-gray-300 text-gray-800 font-bold py-2 px-4 mb-3 rounded-lg transition-transform transform hover:scale-105 hover:shadow-lg active:scale-95 active:shadow-md focus:ring-2 focus:ring-blue-500 focus:outline-none"
          on:click={onClose}
        >
          Cancel
        </button>
      </div>
    </div>
  </div>
{/if}