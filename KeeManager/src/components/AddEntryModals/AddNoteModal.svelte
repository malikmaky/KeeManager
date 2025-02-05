<script lang="ts">
    import type { NoteEntry } from '../../routes/Main.svelte';
  
    export let showModal = false;
    export let onSave: (entry: NoteEntry) => void;
    export let onClose = () => {};
  
    let title = '';
    let content = '';
    let errorMessage = '';
  
    // Reactive statement to clear form fields when the modal is shown
    $: if (showModal) {
      title = '';
      content = '';
      errorMessage = '';
    }
  
    function handleSave() {
      errorMessage = '';
  
      // Field validation
      if (!title || !content) {
        errorMessage = 'Please fill in all required fields.';
        return;
      }
      
      const newEntry: NoteEntry = {
        type: 'note',
        id: '',
        title,
        content,
        last_updated: ''
      };
      
      onSave(newEntry);
      onClose();
    }
  </script>
  
  {#if showModal}
    <div class="no-drag fixed inset-0 bg-gray-50/0 flex items-center justify-center">
      <div class="bg-white p-6 rounded-lg shadow-lg w-full max-w-lg max-h-full overflow-y-auto">
        <h2 class="draggable text-2xl font-extrabold mb-4 text-gray-800">Add New Note</h2>
  
        {#if errorMessage}
          <div class="mb-4 p-2 bg-red-100 text-red-700 border border-red-300 rounded">
            {errorMessage}
          </div>
        {/if}
        
        <div class = "bg-inherit">
          <div class="bg-inherit relative mb-6">
            <input type="text" id="title" name="title" class="peer bg-transparent h-10 w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600" autocomplete="off" placeholder="Title" bind:value={title} />
            <label for="title" class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all">Title</label>
          </div>
    
          <div class="bg-inherit relative mb-6">
            <textarea id="content" name="content" rows="6" class="peer bg-transparent w-full rounded-lg text-gray-800 placeholder-transparent ring-2 px-2 ring-gray-500 focus:ring-sky-600 focus:outline-none focus:border-sky-600" autocomplete="off" placeholder="Note" bind:value={content}></textarea>
            <label for="content" class="absolute cursor-text left-0 -top-3 text-sm text-gray-500 bg-inherit mx-1 px-1 peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-2 peer-focus:-top-3 peer-focus:text-sky-600 peer-focus:text-sm transition-all">Note</label>
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
  