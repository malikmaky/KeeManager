<script lang="ts">
  import type { CreditCardEntry } from "../../routes/Main.svelte"; // Adjust path as needed

  export let showModal = false;
  export let onSave: (entry: CreditCardEntry) => void;
  export let onClose = () => {};

  let title = "";
  let card_number = "";
  let expiryMonth = "";
  let expiryYear = "";
  let cardholder_name = "";
  let cvv = "";
  let errorMessage = "";

  // Range of years for the expiry year
  const years = Array.from({ length: 30 }, (_, i) =>
    (new Date().getFullYear() + i).toString().slice(2)
  );

  // Reactive statement to clear form fields when the modal is shown
  $: if (showModal) {
    title = "";
    card_number = "";
    expiryMonth = "";
    expiryYear = "";
    cardholder_name = "";
    cvv = "";
    errorMessage = "";
  }

  function handleSave() {
    errorMessage = "";

    // Regular expressions for validation
    const cardNumberPattern = /^(\d{4} \d{4} \d{4} \d{4}|\d{16})$/;
    const cardholderNamePattern = /^[A-Za-z\s]+$/;
    const cvvPattern = /^\d{3}$/;

    // Field validation
    if (
      !title ||
      !card_number ||
      !expiryMonth ||
      !expiryYear ||
      !cardholder_name ||
      !cvv
    ) {
      errorMessage = "Please fill in all required fields.";
      return;
    }

    // Validate card number
    if (!cardNumberPattern.test(card_number.replace(/\s+/g, ""))) {
      errorMessage =
        "Card number must be 16 digits long, with or without spaces.";
      return;
    }

    // Validate cardholder name
    if (!cardholderNamePattern.test(cardholder_name)) {
      errorMessage = "Cardholder name must only contain letters and spaces.";
      return;
    }

    // Validate CVV
    if (!cvvPattern.test(cvv)) {
      errorMessage = "CVV must be exactly 3 digits.";
      return;
    }

    // Remove spaces from card number for storage
    const formattedCardNumber = card_number.replace(/\s+/g, "");

    // Format expiry date as MM/YY
    const expiry_date = `${expiryMonth}/${expiryYear}`;

    const newEntry: CreditCardEntry = {
      type: "credit_card",
      id: "",
      title,
      card_number: formattedCardNumber,
      expiry_date,
      cardholder_name,
      cvv,
      last_updated: ""
    };

    onSave(newEntry);
    onClose();
  }


  // Updates the `expiryMonth` or `expiryYear` variable based on selection
  // ( Instead of bind:value )
  function handleSelectChange( event: Event, field: "expiryMonth" | "expiryYear")
  {
    const selectElement = event.target as HTMLSelectElement;
    if (field === "expiryMonth") {
      expiryMonth = selectElement.value;
    } else if (field === "expiryYear") {
      expiryYear = selectElement.value;
    }
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
        Add New Credit Card
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
            id="card_number"
            name="card_number"
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="Card Number"
            maxlength="19"
            bind:value={card_number}
          />
          <label
            for="card_number"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >Card Number</label
          >
        </div>

        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="block text-gray-500 text-sm font-medium mb-2"
          >Expiry Date</label
        >
        <div class="flex items-center mb-6">
          <select
            value={expiryMonth}
            on:change={(e) => handleSelectChange(e, "expiryMonth")}
            class="bg-gray-100 p-2 rounded border border-gray-300 max-h-40 overflow-y-auto"
          >
            <option value="" disabled>Select Month</option>
            {#each Array.from({ length: 12 }, (_, i) => i + 1) as month}
              <option value={month.toString().padStart(2, "0")}
                >{month.toString().padStart(2, "0")}</option
              >
            {/each}
          </select>
          <span class="mx-2">/</span>
          <select
            value={expiryYear}
            on:change={(e) => handleSelectChange(e, "expiryYear")}
            class="bg-gray-100 p-2 rounded border border-gray-300 max-h-40 overflow-y-auto"
          >
            <option value="" disabled>Select Year</option>
            {#each years as year}
              <option value={year}>{year}</option>
            {/each}
          </select>
        </div>

        <div class="bg-inherit relative mb-6">
          <input
            type="text"
            id="cardholder_name"
            name="cardholder_name"
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="Cardholder Name"
            bind:value={cardholder_name}
          />
          <label
            for="cardholder_name"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >Cardholder Name</label
          >
        </div>

        <div class="bg-inherit relative mb-6">
          <input
            type="text"
            id="cvv"
            name="cvv"
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="CVV"
            bind:value={cvv}
            maxlength="3"
          />
          <label
            for="cvv"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >CVV</label
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
