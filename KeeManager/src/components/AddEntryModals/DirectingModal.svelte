<script lang="ts">
  import AddLoginModal from './AddLoginModal.svelte';
  import AddCreditCardModal from './AddCreditCardModal.svelte';
  import AddNoteModal from './AddNoteModal.svelte';
  import AddIdentityModal from './AddIdentityModal.svelte';
  import type { Entry } from '../../routes/Main.svelte';
  import BackIcon from '../../static/img/AddModals/backIcon.svg';
  import AddLoginIcon from '../../static/img/AddModals/addLoginIcon.svg';
  import AddCreditCardIcon from '../../static/img/AddModals/addCreditCardIcon.svg';
  import AddNoteIcon from '../../static/img/AddModals/addNoteIcon.svg';
  import AddIdentityIcon from '../../static/img/AddModals/addIdentityIcon.svg';

  export let showModal = false;
  let showSelectionScreen = true;
  let currentModal: 'login' | 'credit_card' | 'note' | 'identity' | null = null;

  export let onClose: () => void;
  export let onSave: (newEntry: Entry) => void;

  function handleSelectLogin() {
    currentModal = 'login';
    showSelectionScreen = false;
  }

  function handleSelectCreditCard() {
    currentModal = 'credit_card';
    showSelectionScreen = false;
  }

  function handleSelectNote() {
    currentModal = 'note';
    showSelectionScreen = false;
  }

  function handleSelectIdentity() {
    currentModal = 'identity';
    showSelectionScreen = false;
  }

  function handleSaveEntry(newEntry: Entry) {
    onSave(newEntry);
    // Close the modal after saving
    closeModal();
  }

  function closeModal() {
      showModal = false;
      showSelectionScreen = true;
      currentModal = null;
      onClose();
  }
</script>

{#if showModal}
  <div class="draggable fixed inset-0 bg-gray-50/80 flex items-center justify-center z-50">
    
    <div class="bg-white p-6 rounded-lg shadow-lg w-full max-w-lg max-h-full overflow-y-auto">
      {#if showSelectionScreen}
        <div class="w-full mx-auto p-4">
          <h2 class="text-2xl font-semibold mb-4 text-center">Select Entry Type</h2>
          <div class="no-drag flex flex-wrap justify-center items-center w-full gap-2">
              <button 
              class="flex items-center justify-center gap-1 text-xs sm:text-sm font-medium bg-white hover:bg-gray-100 h-9 rounded-md px-2 transition-colors focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 disabled:opacity-50 group"
              on:click={closeModal}
              >
                  <img src= {BackIcon} alt="Back" class="h-4 w-4 sm:h-5 sm:w-5">
                  <span class="hidden sm:inline">Back</span>
              </button>
              
              <button 
              class="flex items-center justify-center gap-1 text-xs sm:text-sm font-medium bg-white hover:bg-gray-100 hover:text-[#06B6D4] h-9 rounded-md px-2 transition-colors focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 disabled:opacity-50"
              on:click={handleSelectLogin}
              >
                  <img src={AddLoginIcon} alt="Add Login" class="h-4 w-4 sm:h-5 sm:w-5">
                  <span class="hidden sm:inline">Add Login</span>
              </button>
              
              <button 
              class="flex items-center justify-center gap-1 text-xs sm:text-sm font-medium bg-white hover:bg-gray-100 hover:text-[#60A5FA] h-9 rounded-md px-2 transition-colors focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 disabled:opacity-50"
              on:click={handleSelectCreditCard}
              >
                  <img src={AddCreditCardIcon} alt="Add Credit Card" class="h-4 w-4 sm:h-5 sm:w-5">
                  <span class="hidden sm:inline">Add Credit Card</span>
              </button>
              
              <button 
              class="flex items-center justify-center gap-1 text-xs sm:text-sm font-medium bg-white hover:bg-gray-100 hover:text-[#FACC14] h-9 rounded-md px-2 transition-colors focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 disabled:opacity-50"
              on:click={handleSelectNote}
              >
                  <img src={AddNoteIcon} alt="Add Note" class="h-4 w-4 sm:h-5 sm:w-5">
                  <span class="hidden sm:inline">Add Note</span>
              </button>
              
              <button 
              class="flex items-center justify-center gap-1 text-xs sm:text-sm font-medium bg-white hover:bg-gray-100 hover:text-[#FB923C] h-9 rounded-md px-2 transition-colors focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 disabled:opacity-50"
              on:click={handleSelectIdentity}
              >
                  <img src={AddIdentityIcon} alt="Add Identity" class="h-4 w-4 sm:h-5 sm:w-5">
                  <span class="hidden sm:inline">Add Identity</span>
              </button>
          </div>
      </div>
      {:else if currentModal === 'login'}
        <AddLoginModal
          showModal={true}
          onSave={handleSaveEntry}
          onClose={closeModal}
        />
      {:else if currentModal === 'credit_card'}
        <AddCreditCardModal
          showModal={true}
          onSave={handleSaveEntry}
          onClose={closeModal}
        />
      {:else if currentModal === 'note'}
        <AddNoteModal
          showModal={true}
          onSave={handleSaveEntry}
          onClose={closeModal}
        />
      {:else if currentModal === 'identity'}
        <AddIdentityModal
          showModal={true}
          onSave={handleSaveEntry}
          onClose={closeModal}
        />
      {/if}
    </div>
  </div>
{/if}