<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from "svelte";
  import type { Entry, CreditCardEntry } from "../../routes/Main.svelte";
  import { selectedEntryStore } from "../../reactiveStores";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import CopyIcon from "../../static/img/copyIcon.svg";
  import ShowPasswordIcon from "../../static/img/openEyeIcon.svg";
  import HidePasswordIcon from "../../static/img/closedEyeIcon.svg";
  import CreditCardBackground from "../../static/img/creditCardBackground.png";
  import PaymentServiceLogo from "../../static/img/paymentServiceLogo.png";

  let selectedEntry: Entry | null = null;
  let selectedCreditCard: CreditCardEntry;

  const dispatch = createEventDispatcher();
  const unsubscribeSelectedEntry = selectedEntryStore.subscribe((value) => {
    selectedEntry = value;
  });

  onDestroy(() => {
    unsubscribeSelectedEntry();
  });

  let title: string = "";
  let cardHolderName: string = "";
  let cardNumber: string = "";
  let expiry: string = "";
  let cvv: string = "";
  let lastUpdated: string = "";
  let showCvv = false;
  let showCardNumber = false;
  let isEditing: boolean = false;
  let errorMessage = "";

  // Store original values ( before edit )
  let originalTitle: string = "";
  let originalCardHolderName: string = "";
  let originalCardNumber: string = "";
  let originalExpiry: string = "";
  let originalCvv: string = "";
  let originalLastUpdated: string = "";

  let expiryYear = "";
  let expiryMonth = "";

  // Load entry details
  onMount(() => {
    if (selectedEntry) {
      selectedCreditCard = selectedEntry as CreditCardEntry;
      loadOriginalValues();
      updateExpiryFields();
      errorMessage = "";
    }
  });

  $: {
    if (selectedEntry && !isEditing) {
      selectedCreditCard = selectedEntry as CreditCardEntry;
      loadOriginalValues();
      updateExpiryFields();
      errorMessage = "";
    }
  }

  function loadOriginalValues() {
    title = selectedCreditCard.title || "";
    cardHolderName = selectedCreditCard.cardholder_name || "";
    cardNumber = selectedCreditCard.card_number || "";
    expiry = selectedCreditCard.expiry_date || "";
    cvv = selectedCreditCard.cvv || "";
    lastUpdated = selectedCreditCard.last_updated || "";
    errorMessage = "";

    originalTitle = title;
    originalCardHolderName = cardHolderName;
    originalCardNumber = cardNumber;
    originalExpiry = expiry;
    originalCvv = cvv;
    originalLastUpdated = lastUpdated;
  }

  function updateExpiryFields() {
    const [expMonth, expYear] = expiry.split("/");
    expiryMonth = expMonth || "";
    expiryYear = expYear || "";
  }

  function editEntry() {
    isEditing = true;
    errorMessage = "";
  }

  // Save changes if made
  function saveChanges() {
    // Regular expressions for validation
    const cardNumberPattern = /^(\d{4} \d{4} \d{4} \d{4}|\d{16})$/;
    const cardholderNamePattern = /^[A-Za-z\s]+$/;
    const cvvPattern = /^\d{3}$/;

    if (selectedEntry && (hasChanges() || isExpiryChanged())) {
      // Field validation
      if (
        !title ||
        !cardNumber ||
        !expiryMonth ||
        !expiryYear ||
        !cardHolderName ||
        !cvv
      ) {
        errorMessage = "Please fill in all required fields.";
        return;
      }

      // Validate card number
      if (!cardNumberPattern.test(cardNumber)) {
        errorMessage =
          "Card number must be 16 digits long, with or without spaces.";
        return;
      }

      // Validate cardholder name
      if (!cardholderNamePattern.test(cardHolderName)) {
        errorMessage = "Cardholder name must only contain letters and spaces.";
        return;
      }

      // Validate CVV
      if (!cvvPattern.test(cvv)) {
        errorMessage = "CVV must be exactly 3 digits.";
        return;
      }

      // MM/YY format
      const expiryDate = `${expiryMonth}/${expiryYear.slice(-2)}`;

      // Remove spaces from card number for storage
      const formattedCardNumber = cardNumber.replace(/\s+/g, "");

      selectedCreditCard.title = title;
      selectedCreditCard.cardholder_name = cardHolderName;
      selectedCreditCard.card_number = formattedCardNumber;
      selectedCreditCard.expiry_date = expiryDate;
      selectedCreditCard.cvv = cvv;
      selectedCreditCard.last_updated = new Date().toISOString();
      dispatch("update", selectedEntry);
    }
    isEditing = false;
  }

  function cancelEdit() {
    loadOriginalValues();
    updateExpiryFields();
    isEditing = false;
  }

  function hasChanges() {
    return (
      title !== originalTitle ||
      cardHolderName !== originalCardHolderName ||
      cardNumber !== originalCardNumber ||
      cvv !== originalCvv
    );
  }

  function isExpiryChanged() {
    return `${expiryMonth}/${expiryYear}` !== originalExpiry;
  }

  // Updates variables according to new input
  // ( Instead of bind:value )
  function handleInputChange(
    event: Event,
    field: "title" | "cardHolderName" | "cardNumber" | "cvv"
  ) {
    const input = (event.target as HTMLInputElement).value;
    if (field === "title") title = input;
    else if (field === "cardHolderName") cardHolderName = input;
    else if (field === "cardNumber") cardNumber = input;
    else if (field === "cvv") cvv = input;
  }

  // Updates variables according to new input
  // ( Instead of bind:value )
  function handleSelectChange(
    event: Event,
    field: "expiryYear" | "expiryMonth"
  ) {
    const value = (event.target as HTMLSelectElement).value;
    if (field === "expiryYear") expiryYear = value;
    else if (field === "expiryMonth") expiryMonth = value;
  }

  function toggleShowCvv() {
    showCvv = !showCvv;
  }

  function toggleShowCardNumber() {
    showCardNumber = !showCardNumber;
  }

  function formatCardNumber(cardNumber: string): string {
    // Remove non-digit characters from card number
    cardNumber = cardNumber.replace(/\D/g, "");

    // Add spaces every 4 digits
    return cardNumber.replace(/(\d{4})(?=\d)/g, "$1 ");
  }

  function getObscuredCardNumber() {
    if (cardNumber.length < 16) {
      return "•••• •••• •••• ••••";
    } else {
      return "•••• •••• •••• " + cardNumber.slice(-4);
    }
  }

  const copyToClipboard = async (
    field: "cardHolderName" | "cardNumber" | "cvv"
  ) => {
    try {
      if (field === "cardHolderName") {
        await writeText(cardHolderName);
      } else if (field === "cardNumber") {
        await writeText(cardNumber);
      } else if (field === "cvv") {
        await writeText(cvv);
      }
      console.log(`${field} copied to clipboard`);
    } catch (error) {
      console.error(`Failed to copy ${field}:`, error);
    }
  };

  const currentYear = new Date().getFullYear();
  const years = Array.from({ length: 30 }, (_, i) =>
    (currentYear + i).toString().slice(2)
  );
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
    <!-- Credit Card Preview -->
    <div
      class="mx-auto h-56 max-w-[400px] rounded-xl relative text-white shadow-2xl transition-transform transform hover:scale-105 mb-6"
    >
      <img
        class="relative object-cover w-full h-full rounded-xl"
        src={CreditCardBackground}
        alt="Credit Card Preview"
      />
      <div class="w-full px-8 absolute top-8">
        <div class="flex justify-between">
          <div>
            <p class="font-light">Cardholder Name</p>
            <p class="font-medium tracking-widest">{cardHolderName}</p>
          </div>
          <img class="w-14 h-14" src={PaymentServiceLogo} alt="Paymet Service" />
        </div>
        <div class="pt-1">
          <p class="font-light">Card Number</p>
          <p class="font-medium tracking-more-wider">
            {showCardNumber
              ? formatCardNumber(cardNumber)
              : getObscuredCardNumber()}
          </p>
        </div>
        <div class="pt-6 pr-6">
          <div class="flex justify-between items-center">
            <div>
              <p class="font-light text-xs">Expiry</p>
              <p class="font-medium tracking-wider text-sm">{expiry}</p>
            </div>
            <div>
              <p class="font-light text-xs">CVV</p>
              <p
                class="font-bold tracking-more-wider text-sm"
                style="width: 3ch; text-align: right;"
              >
                {showCvv ? cvv : "···"}
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>

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
            Cardholder Name
            </label>
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("cardHolderName")}
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
            value={cardHolderName}
            on:input={(e) => handleInputChange(e, "cardHolderName")}
            class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
          />
        {:else}
          <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
            {cardHolderName}
          </div>
        {/if}
      </div>

      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <div class="flex flex-row">
          <label class="block text-gray-500 text-sm font-medium mb-2">
            Card Number
          </label>
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("cardNumber")}
          >
            <img
              class="w-5 h-5 opcaity-50"
              src={CopyIcon}
              alt="Copy To Clipboard"
            />
          </button>
        </div>
        <div class="relative">
          {#if isEditing}
            <input
              type={showCardNumber ? "text" : "password"}
              value={cardNumber}
              maxlength="19"
              on:input={(e) => handleInputChange(e, "cardNumber")}
              class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
            />
            <button
              type="button"
              on:click={toggleShowCardNumber}
              class="absolute right-2 top-2 text-gray-600"
            >
              <img
                src={showCvv ? ShowPasswordIcon : HidePasswordIcon}
                alt="Toggle CVV visibility"
                class="h-6 w-6"
              />
            </button>
          {:else}
            <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
              {showCardNumber
                ? formatCardNumber(cardNumber)
                : "•••• •••• •••• " + cardNumber.slice(-4)}
            </div>
            <button
              type="button"
              on:click={toggleShowCardNumber}
              class="absolute right-2 top-2 text-gray-600"
            >
              <img
                src={showCvv ? ShowPasswordIcon : HidePasswordIcon}
                alt="Toggle CVV visibility"
                class="h-6 w-6"
              />
            </button>
          {/if}
        </div>
      </div>

      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="block text-gray-500 text-sm font-medium mb-2">
          Expiry Date
          </label>
        {#if isEditing}
          <div class="flex items-center">
            <select
              value={expiryMonth}
              on:change={(e) => handleSelectChange(e, "expiryMonth")}
              class="bg-gray-100 p-2 rounded border border-gray-300"
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
              class="bg-gray-100 p-2 rounded border border-gray-300"
            >
              <option value="" disabled>Select Year</option>
              {#each years as year}
                <option value={year}>{year}</option>
              {/each}
            </select>
          </div>
        {:else}
          <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
            {expiry}
          </div>
        {/if}
      </div>

      <div class="mb-6">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <div class="flex flex-row">
          <label class="block text-gray-500 text-sm font-medium mb-2">CVV</label
          >
          <button
            type="button"
            class="mb-2"
            on:click={() => copyToClipboard("cvv")}
          >
            <img
              class="w-5 h-5 opcaity-50"
              src={CopyIcon}
              alt="Copy To Clipboard"
            />
          </button>
        </div>
        <div class="relative">
          {#if isEditing}
            <input
              type={showCvv ? "text" : "password"}
              value={cvv}
              maxlength="3"
              on:input={(e) => handleInputChange(e, "cvv")}
              class="text-gray-900 font-semibold bg-gray-100 p-2 rounded w-full border border-gray-300"
            />
            <button
              type="button"
              on:click={toggleShowCvv}
              class="absolute right-2 top-2 text-gray-600"
            >
              <img
                src={showCvv ? ShowPasswordIcon : HidePasswordIcon}
                alt="Toggle CVV visibility"
                class="h-6 w-6"
              />
            </button>
          {:else}
            <div class="text-gray-900 font-semibold bg-gray-100 p-2 rounded">
              {showCvv ? cvv : "•••"}
            </div>
            <button
              type="button"
              on:click={toggleShowCvv}
              class="absolute right-2 top-2 text-gray-600"
            >
              <img
                src={showCvv ? ShowPasswordIcon : HidePasswordIcon}
                alt="Toggle CVV visibility"
                class="h-6 w-6"
              />
            </button>
          {/if}
        </div>
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
