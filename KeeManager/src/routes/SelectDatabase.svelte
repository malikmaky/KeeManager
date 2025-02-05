<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { push } from "svelte-spa-router";
  import { open } from "@tauri-apps/plugin-dialog";
  import { database } from "../reactiveStores";
  import LogoIcon from '../static/img/logo.png';
  import ShowPasswordIcon from '../static/img/openEyeIcon.svg';
  import HidePasswordIcon from '../static/img/closedEyeIcon.svg';
  import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
  import { slide } from "svelte/transition";
  import "../global.css";

  let dbPath = "";
  let masterKey = "";
  let isDbSelected = false;
  let showPassword = false;
  let passwordInput: HTMLInputElement;

  //
  // Function to handle file selection using Tauri's dialog API
  //
  const handleSelectPath = async () => {
    try {
      const selectedFile = await open({
        filters: [{ name: "Database Files", extensions: ["db"] }],
        multiple: false,
      });
      if (selectedFile) {
        dbPath = selectedFile.path;
        isDbSelected = true;
      } else {
        isDbSelected = false;
        dbPath = "";
      }
    } catch (error) {
      isDbSelected = false;
      alert("Error selecting file.");
    }
  };

  const handleOpenDatabase = async () => {
    if (dbPath && masterKey) {
      const isDatabaseValid = (await invoke("is_database_valid", {dbPath})) as boolean;
      if (isDatabaseValid) {
        const isKeyCorrect = (await invoke("check_master_key_hash", {dbPath, masterKey})) as boolean;
        if (isKeyCorrect) {
          await invoke("set_database_path", { dbPath });
          database.set({ dbPath, masterKey });
          push("/main");
        } else {
          alert("The key entered is not valid.");
        }
      } else {
        alert("The database is not valid. Please try again.");
      }
    } else {
      alert("Please enter the master key.");
    }
  };

  const handleClearDatabase = () => {
    dbPath = "";
    masterKey = "";
    isDbSelected = false;
  };

  //
  // Check if a database path is saved 
  // in configuration file when the component mounts
  //
  onMount(async () => {
    await getCurrentWindow().setSize(new LogicalSize(400, 600));
    await getCurrentWindow().center();
    await getCurrentWindow().setMinSize(new LogicalSize(400, 600));
    await getCurrentWindow().setResizable(false);
    await getCurrentWindow().setMaximizable(false);
    try {
      const savedPath = (await invoke("get_database_path")) as string;
      if (savedPath) {
        dbPath = savedPath;
        const isDatabaseValid = (await invoke("is_database_valid", {dbPath})) as boolean;
        if (isDatabaseValid) {
          isDbSelected = true;
        } else {
          dbPath = "";
          isDbSelected = false;
        }
      }
    } catch (error) {
      alert("Error occured.");
    }
  });

  const togglePasswordVisibility = () => {
    showPassword = !showPassword;
    if (passwordInput) {
      passwordInput.type = showPassword ? "text" : "password";
    }
  };
</script>

<div
  class="full-page h-screen w-screen flex items-center justify-center overflow-x-hidden overflow-y-hidden"
>
  <div
    class="draggable bg-blue-100 min-w-[300px] max-w-[400px] rounded-lg p-4 mx-auto my-auto relative"
  >
    <!-- Header with minimize and close buttons -->
    <header class="flex justify-end items-center mb-2 space-x-1">
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

    <img
      class="draggable w-1/3 mx-auto mb-5 max-w-[150px]"
      src= {LogoIcon}
      alt="Logo"
    />

    <!-- Main Form Content -->
    <div class="draggable relative z-10">
      <p
        class="focus:outline-none text-2xl mb-5 font-extrabold leading-6 text-gray-800 text-center"
      >
        {#if isDbSelected}
          Welcome Back!
        {:else}
          Select Database{/if}
      </p>

      {#if isDbSelected}
        <div class="draggable" transition:slide={{ duration: 400 }}>
          <p class="focus:outline-none text-sm mt-4 font-medium leading-none text-black-500">
            Enter Master Key
          </p>
          <p class="text-center focus:outline-none text-sm mt-4 font-medium leading-none text-gray-500">
            {dbPath}
          </p>

          <form class="no-drag bg-blue-100 mt-6">
            <div class="bg-inherit relative mb-6">
              <input
                type="password"
                id="masterkey"
                name="masterkey"
                class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600"
                placeholder=""
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
                >Master Key</label
              >
            </div>

            <div>
              <button
                class="w-full bg-blue-700 hover:bg-blue-800 text-white font-bold py-2 px-4 mb-3 rounded-lg transition-transform transform hover:scale-105 hover:shadow-lg active:scale-95 active:shadow-md focus:ring-2 focus:ring-blue-500 focus:outline-none"
                on:click={handleOpenDatabase}
              >
                Ok
              </button>
            </div>
            <div class="text-center relative z-10">
              <button
                class="w-full border-2 border-blue-700 text-blue-700 font-bold py-2 px-4 mb-6 rounded-lg bg-transparent transition-transform transform hover:scale-105 hover:shadow-lg active:scale-95 active:shadow-md focus:ring-2 focus:ring-blue-500 focus:outline-none"
                on:click={handleClearDatabase}
              >
                Clear
              </button>
            </div>
          </form>
        </div>
      {:else}
        <div class="no-drag text-center relative z-20">
          <button
            class="w-full bg-blue-700 hover:bg-blue-800 text-white font-bold py-2 px-4 mb-6 rounded-lg transition-transform transform hover:scale-105 hover:shadow-lg active:scale-95 active:shadow-md focus:ring-2 focus:ring-blue-500 focus:outline-none"
            on:click={handleSelectPath}
          >
            Select An Existing Database
          </button>
        </div>
      {/if}

      <div class="draggable flex items-center justify-center">
        <div class="flex-1 border-t border-gray-400"></div>
        <div class="mx-2 text-center text-black-500">Or</div>
        <div class="flex-1 border-t border-gray-400"></div>
      </div>

      <form class="no-drag bg-inherit mt-6">
        <div class="text-center">
          <button
            class="w-full bg-blue-700 hover:bg-blue-800 text-white font-bold py-2 px-4 mb-6 rounded-lg transition-transform transform hover:scale-105 hover:shadow-lg active:scale-95 active:shadow-md focus:ring-2 focus:ring-blue-500 focus:outline-none"
            on:click={() => push('/create')}
          >
            Create A New Database
          </button>
        </div>
      </form>
    </div>
  </div>
</div>
