<script lang="ts">
  import type { LoginEntry } from "../../routes/Main.svelte";
  import ShowPasswordIcon from "../../static/img/openEyeIcon.svg";
  import HidePasswordIcon from "../../static/img/closedEyeIcon.svg";
  import { slide } from "svelte/transition";

  export let showModal = false;
  export let onSave: (entry: LoginEntry) => void;
  export let onClose = () => {};

  let title = "";
  let username = "";
  let password = "";
  let url = "";
  let notes = "";
  let passwordInput: HTMLInputElement;

  let showPassword = false;
  let showPasswordGeneration = false;
  let passwordLength = 12;
  let includeLowercase = true;
  let includeUppercase = true;
  let includeNumbers = true;
  let includeSymbols = true;
  let passwordScore = 0;
  let errorMessage = "";

  // Reactive statement to clear form fields when the modal is shown
  $: if (showModal) {
    title = "";
    username = "";
    password = "";
    url = "";
    notes = "";
    errorMessage = "";
    showPassword = false;
    showPasswordGeneration = false;
    passwordLength = 12;
    includeLowercase = true;
    includeUppercase = true;
    includeNumbers = true;
    includeSymbols = true;
    passwordScore = 0;
  }

  // Reactive statements to update password strength
  $: if (password) {
    checkStrength();
  }

  $: if (password === "") {
    passwordScore = 0;
  }

  function handleSave() {
    errorMessage = "";

    // Field validation
    if (!title || !username || !password || !url) {
      errorMessage = "Please fill in all required fields.";
      return;
    }

    const newEntry: LoginEntry = {
      type: "login",
      id: "",
      title,
      username,
      password: password,
      salt: "",
      url,
      notes,
      last_updated: "",
    };

    onSave(newEntry);
    onClose();
  }

  // Character sets for password generation
  const lowerChars = "abcdefghijklmnopqrstuvwxyz";
  const upperChars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  const numericChars = "0123456789";
  const symbolChars = "!\"#$%&'()*+,-./:;<=>?@[\\]^_{|}~";

  function togglePasswordVisibility() {
    showPassword = !showPassword;
    if (passwordInput) {
      passwordInput.type = showPassword ? "text" : "password";
    }
  }

  function togglePasswordGeneration() {
    showPasswordGeneration = !showPasswordGeneration;
  }

  function generatePassword() {
    let charPool = "";
    if (includeLowercase) charPool += lowerChars;
    if (includeUppercase) charPool += upperChars;
    if (includeNumbers) charPool += numericChars;
    if (includeSymbols) charPool += symbolChars;

    let generatedPassword = "";
    for (let i = 0; i < passwordLength; i++) {
      generatedPassword += charPool.charAt(
        Math.floor(Math.random() * charPool.length)
      );
    }
    password = generatedPassword;
    checkStrength();
  }

  function checkStrength() {
    let score = 0;
    // Length
    if (password.length >= 8) score += 1;
    if (password.length >= 12) score += 1;
    if (password.length >= 16) score += 1;

    // Character variety
    if (/[a-z]/.test(password)) score += 1;
    if (/[A-Z]/.test(password)) score += 1;
    if (/[0-9]/.test(password)) score += 1;
    if (/[^A-Za-z0-9]/.test(password)) score += 1;

    passwordScore = score;
  }

  function getStrengthColor(score: number): string {
    if (score <= 2) return "bg-red-500"; // Weak
    if (score <= 4) return "bg-yellow-500"; // Medium
    return "bg-green-500"; // Strong
  }

  generatePassword();
</script>

{#if showModal}
  <div
    class="no-drag fixed inset-0 bg-gray-50/0 flex items-center justify-center z-50"
  >
    <div
      class="bg-white p-6 rounded-lg shadow-lg w-full max-w-lg max-h-full overflow-y-auto"
    >
      <h2 class="draggable text-2xl font-extrabold mb-4 text-gray-800">
        Add New Login
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
            id="username"
            name="username"
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="Username"
            bind:value={username}
          />
          <label
            for="username"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >Username</label
          >
        </div>

        <div class="bg-inherit relative mb-6">
          <input
            type="password"
            id="password"
            name="password"
            bind:value={password}
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            placeholder="Password"
            autocomplete="off"
            bind:this={passwordInput}
          /><button
            type="button"
            class="absolute inset-y-0 right-0 px-3 flex items-center justify-center text-gray-500 hover:text-gray-700 focus:outline-none"
            on:click={togglePasswordVisibility}
          >
            {#if showPassword}
              <img class="w-6 h-6 bg-white" src={ShowPasswordIcon} alt="hide" />
            {:else}
              <img class="w-6 h-6 bg-white" src={HidePasswordIcon} alt="show" />
            {/if}
          </button>
          <label
            for="password"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >Password</label
          >
        </div>

        <div class="flex -mx-1 mb-4">
          {#each [1, 2, 3, 4] as n, i}
            <div class="w-1/4 px-1">
              <div
                class={`h-2 rounded-xl transition-colors ${i < passwordScore ? getStrengthColor(passwordScore) : "bg-gray-200"}`}
              ></div>
            </div>
          {/each}
        </div>

        <button
          type="button"
          class="px-4 py-2 mb-4 bg-gray-200 text-gray-800 rounded-md hover:bg-gray-300 w-full transition-transform transform hover:scale-105 hover:shadow-lg active:scale-95 active:shadow-md"
          on:click={togglePasswordGeneration}
        >
          {showPasswordGeneration
            ? "Hide Password Generator"
            : "Show Password Generator"}
        </button>

        {#if showPasswordGeneration}
          <div transition:slide={{ duration: 300 }} class="mb-4">
            <div class="flex flex-col gap-2 mb-4">
              <!-- svelte-ignore a11y-label-has-associated-control -->
              <label class="text-xs font-semibold text-gray-500">
                Password Length: {passwordLength}
              </label>
              <div class="flex items-center gap-2">
                <input
                  type="number"
                  min="8"
                  max="36"
                  step="1"
                  bind:value={passwordLength}
                  class="w-16 px-2 py-1 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"
                  autocomplete="off"
                  on:input={generatePassword}
                />
                <input
                  type="range"
                  min="8"
                  max="36"
                  step="1"
                  bind:value={passwordLength}
                  class="flex-1"
                  autocomplete="off"
                  on:input={generatePassword}
                />
              </div>
            </div>

            <!-- Character Types Section -->
            <div class="flex items-center gap-4 mb-4">
              <label class="flex items-center gap-2">
                <input
                  type="checkbox"
                  bind:checked={includeLowercase}
                  on:change={generatePassword}
                  autocomplete="off"
                  class="form-checkbox rounded-full h-6 w-6 text-sky-600 border-gray-300 focus:ring-sky-600"
                />
                <span class="text-gray-700">Lowercase</span>
              </label>
              <label class="flex items-center gap-2">
                <input
                  type="checkbox"
                  bind:checked={includeUppercase}
                  on:change={generatePassword}
                  autocomplete="off"
                  class="form-checkbox rounded-full h-6 w-6 text-sky-600 border-gray-300 focus:ring-sky-600"
                />
                <span class="text-gray-700">Uppercase</span>
              </label>
              <label class="flex items-center gap-2">
                <input
                  type="checkbox"
                  bind:checked={includeNumbers}
                  on:change={generatePassword}
                  autocomplete="off"
                  class="form-checkbox rounded-full h-6 w-6 text-sky-600 border-gray-300 focus:ring-sky-600"
                />
                <span class="text-gray-700">Numbers</span>
              </label>
              <label class="flex items-center gap-2">
                <input
                  type="checkbox"
                  bind:checked={includeSymbols}
                  on:change={generatePassword}
                  autocomplete="off"
                  class="form-checkbox rounded-full h-6 w-6 text-sky-600 border-gray-300 focus:ring-sky-600"
                />
                <span class="text-gray-700">Symbols</span>
              </label>
            </div>
          </div>
        {/if}

        <div class="bg-inherit relative mb-6">
          <input
            type="text"
            id="url"
            name="url"
            class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="URL"
            bind:value={url}
          />
          <label
            for="url"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >URL</label
          >
        </div>

        <div class="bg-inherit relative mb-6">
          <textarea
            id="notes"
            name="notes"
            rows="3"
            class="peer bg-transparent w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
            autocomplete="off"
            placeholder="Notes"
            bind:value={notes}
          ></textarea>
          <label
            for="notes"
            class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
            >
            Notes
          </label>
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
