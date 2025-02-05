<script lang="ts">
  import { onDestroy } from 'svelte';
  import { createEventDispatcher } from 'svelte';
  import type { Entry } from '../routes/Main.svelte';
  import DirectingModal from './AddEntryModals/DirectingModal.svelte';
  import loginIcon from '../static/img/Entries/loginEntryIcon.svg';
  import creditCardIcon from '../static/img/Entries/creditCardEntryIcon.svg';
  import noteIcon from '../static/img/Entries/noteEntryIcon.svg';
  import identityIcon from '../static/img/Entries/identityEntryIcon.svg';
  import { entriesStore, selectedEntryStore } from '../reactiveStores';

  const dispatch = createEventDispatcher();
  let showModal = false;
  let entries: Entry[] = [];
  let selectedEntry: Entry | null = null;

  const unsubscribeEntries = entriesStore.subscribe(value => {
    entries = value;
  });

  const unsubscribeSelectedEntry = selectedEntryStore.subscribe(value => {
    selectedEntry = value;
  });

  onDestroy(() => {
    unsubscribeEntries();
    unsubscribeSelectedEntry();
  });

  function handleClick(entry: Entry) {
    dispatch('entryclick', entry);
  }

  function handleAddEntry() {
    showModal = true;
  }

  function handleAddEntryFromModal(newEntry: Entry) {
    dispatch('addentry', newEntry);
    showModal = false;
  }

  function closeModal() {
    showModal = false;
  }
</script>

<div class="relative w-full h-full">
  <div class="h-full overflow-y-auto overflow-x-hidden p-2">
    <!-- Entries Panel -->
    <ul class="space-y-2 ml-2 mr-2">
      {#each entries as entry }
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
        <li
          class="flex items-center p-4 mt-1 rounded shadow cursor-pointer transition duration-300 {selectedEntry === entry ? 'bg-blue-200 shadow-md shadow-gray-900/50 border-2 border-blue-400' : 'bg-white hover:bg-gray-100 hover:border border-gray-400'}"
          on:click={() => handleClick(entry)}
        >
          <img
            src={
              entry.type === 'login' ? loginIcon : 
              entry.type === 'credit_card' ? creditCardIcon :
              entry.type === 'note' ? noteIcon :
              entry.type === 'identity' ? identityIcon : ''
            }
            alt={entry.type}
            class="mr-4 w-8 h-8"
          />
          <div class="flex-grow">
            {#if entry.type === 'login'}
              <h3 class="text-lg font-semibold text-gray-900">{entry.title || 'Untitled'}</h3>
              <p class="truncate text-sm text-gray-600">{entry.url}</p>
            {/if}
            {#if entry.type === 'credit_card'}
              <h3 class="text-lg font-semibold text-gray-900">{entry.title}</h3>
              <p class="truncate text-sm text-gray-600">{entry.cardholder_name}</p>
            {/if}
            {#if entry.type === 'note'}
              <h3 class="text-lg font-semibold text-gray-900">{entry.title}</h3>
              <p class="truncate text-sm text-gray-600">{entry.content.split('\n',1)}</p>
            {/if}
            {#if entry.type === 'identity'}
              <h3 class="text-lg font-semibold text-gray-900">{entry.title}</h3>
              <p class="truncate text-sm text-gray-600">{entry.full_name}</p>
            {/if}
          </div>
          <span class="text-gray-500 hover:text-gray-700 transition duration-300">
            &#10095;
          </span>
        </li>
      {/each}
    </ul>
  </div>

  <!-- Hovering Add New Entry Button -->
  <button
  title="Add New Entry"
  class="absolute bottom-6 right-1 bg-blue-500 mr-4 rounded-full shadow-lg group cursor-pointer outline-none transition-transform duration-300 hover:scale-110 hover:rotate-90 hover:shadow-[0_0_10px_rgba(0,130,255,0.7)]"
  on:click={handleAddEntry}
  >
    <svg
    xmlns="http://www.w3.org/2000/svg"
    width="50px"
    height="50px"
    viewBox="0 0 24 24"
    class="stroke-white fill-none group-hover:fill-blue-800 group-active:stroke-zinc-200 group-active:fill-zinc-600 group-active:duration-0 duration-300"
  >
    
    <path d="M8 12H16" stroke-width="1.2"></path>
    <path d="M12 16V8" stroke-width="1.2"></path>
  </svg>
</button>
  <DirectingModal
    showModal={showModal}
    onSave={handleAddEntryFromModal}
    onClose={closeModal}
  />
</div>

<style>
  li:hover {
    transform: scale(1.02);
  }
</style>
