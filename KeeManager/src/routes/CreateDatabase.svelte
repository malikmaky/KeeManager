<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { push } from "svelte-spa-router";
  import { open } from "@tauri-apps/plugin-dialog";
  import { database } from "../reactiveStores";
  import LogoIcon from '../static/img/logo.png';
  import ShowPasswordIcon from '../static/img/openEyeIcon.svg';
  import HidePasswordIcon from '../static/img/closedEyeIcon.svg';
  import ReturnIcon from '../static/img/CreateDB/backReturnIcon.svg';
  import UploadIcon from '../static/img/CreateDB/uploadCircleIcon.svg';
  import { slide } from "svelte/transition";

  let databaseName = "";
  let folderPath = "";
  let masterKey = "";
  let isDirectorySelected = false;
  let showPassword = false;
  let passwordInput: HTMLInputElement;

  //
  // Function to handle folder selection using Tauri's dialog API
  //
  const handleSelectFolder = async () => {
    try {
      const selectedPath = await open({
        directory: true,
        multiple: false,
      });

      if (selectedPath && selectedPath.length > 0) {
        folderPath = selectedPath;
        isDirectorySelected = true;
      }
    } catch (error) {
      alert("Error selecting folder.");
    }
  };

  const handleCreate = async () => {
    if (databaseName && folderPath && masterKey) {
      const dbPath = `${folderPath}/${databaseName}.db`;
      const isDatabaseCreated = await invoke("create_database", {
        dbPath,
        masterKey,
      });
      if (isDatabaseCreated) {
        await invoke("set_database_path", { dbPath });
        database.set({ dbPath, masterKey });
        push("/main");
      } else {
        alert("Database name already exists, please try a different name.");
      }
    } else {
      alert(
        "Please enter a database name, select a folder, and enter a master key."
      );
    }
  };

  $: canCreate = databaseName && folderPath && masterKey;

  const togglePasswordVisibility = () => {
    showPassword = !showPassword;
    if (passwordInput) {
      passwordInput.type = showPassword ? "text" : "password";
    }
  };
</script>

<div class="draggable w-full max-w-xs m-auto bg-blue-100 rounded-lg p-6 relative">
  <!-- Header with back, minimize, and close buttons -->
  <header class="flex justify-end items-center mb-2 space-x-1">
    <button
      class="no-drag absolute top-4 left-4 p-2 bg-inherit rounded-full hover:bg-gray-300 transition-colors"
      aria-label="Back"
      on:click={() => push("/")}
    >
      <img src={ReturnIcon} alt="Back" class="w-6 h-6" />
    </button>

    <button
      type="button"
      class="no-drag bg-white rounded-md p-1 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-100 hover:outline hover:outline-2 hover:outline-gray-300"
      on:click={() => invoke("minimize_window")}
    >
      <span class="sr-only">Minimize Window</span>
      <svg
        class="h-5 w-5"
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
      class="no-drag bg-white rounded-md p-1 inline-flex items-center justify-center text-gray-400 hover:text-red-500 hover:bg-red-300 hover:outline hover:outline-2 hover:outline-gray-300 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-red-500"
      on:click={() => invoke("close_window")}
    >
      <span class="sr-only">Close Window</span>
      <svg
        class="h-5 w-5"
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
  </header>

  <div class="text-center mb-6">
    <img
      class="w-1/3 mx-auto mb-5 max-w-[150px]"
      src={LogoIcon}
      alt="Logo"
    />
    <p class="focus:outline-none text-2xl font-extrabold leading-6 text-gray-800">
      Create New Database
    </p>
  </div>

  <form class="no-drag bg-inherit mt-6">
    <div class="bg-inherit relative mb-6">
      <input
        type="text"
        id="databaseName"
        name="databaseName"
        class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
        autocomplete="off"
        placeholder="Database Name"
        bind:value={databaseName}
      />
      <label
        for="databaseName"
        class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
        >Database Name</label
      >
    </div>

    <div class="bg-inherit relative mb-6">
      <input
        type="password"
        id="masterkey"
        name="masterkey"
        class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
        placeholder="Master Key"
        bind:value={masterKey}
        bind:this={passwordInput}
      />
      <button
        type="button"
        class="absolute inset-y-0 right-0 px-3 flex items-center justify-center text-gray-500 hover:text-gray-700 focus:outline-none"
        on:click={togglePasswordVisibility}
      >
        {#if showPassword}
          <img
            class="w-6 h-6 bg-blue-100"
            src={ShowPasswordIcon}
            alt="hide"
          />
        {:else}
          <img
            class="w-6 h-6 bg-blue-100"
            src={HidePasswordIcon}
            alt="show"
          />
        {/if}
      </button>
      <label
        for="masterkey"
        class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all"
        >
        Master Key
        </label>
    </div>

    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="relative mb-5" on:click={handleSelectFolder}>
      <label
        title="Click to select"
        for="upload"
        class="cursor-pointer flex items-center gap-4 px-6 py-2 before:border-gray-400 hover:before:border-gray-800 group before:bg-gray-100 before:absolute before:inset-0 before:rounded-3xl before:border before:border-dashed before:transition-transform before:duration-300 hover:before:scale-110 active:duration-75 active:before:scale-95"
      >
        <div class="w-max relative">
          <img
            class="w-12"
            src={UploadIcon}
            alt="Click to  select"
            width="512"
            height="512"
          />
        </div>
        <div class="relative">
          {#if isDirectorySelected}
            <span
              class="block text-base font-semibold relative text-blue-900 group-hover:text-blue-500"
            >
              Selected Directory
            </span>
            <span class="mt-0.5 block text-sm text-gray-500">{folderPath}</span>
          {:else}
            <span
              class="block text-base font-semibold relative text-blue-900 group-hover:text-blue-500"
            >
              Select Directory
            </span>
            <span class="mt-0.5 block text-sm text-gray-500"
              >Select the directory of the database file
            </span>
          {/if}
        </div>
      </label>
    </div>

    {#if canCreate}
      <div transition:slide={{ duration: 400 }}>
        <button
          class="w-full bg-blue-700 hover:bg-blue-800 text-white font-bold py-2 px-4 mb-3 rounded-lg transition-transform transform hover:scale-105 hover:shadow-lg active:scale-95 active:shadow-md focus:ring-2 focus:ring-blue-500 focus:outline-none"
          on:click={handleCreate}
        >
          Ok
        </button>
      </div>
    {/if}
  </form>
</div>
