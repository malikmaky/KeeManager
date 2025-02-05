<script lang="ts">
  import { Sidebar, SidebarGroup, SidebarItem } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";
  import { writable } from "svelte/store";
  import { filterStore, selectedEntryStore } from "../reactiveStores";
  import { fade, slide, fly } from "svelte/transition";
  import { cubicIn, sineIn } from "svelte/easing";
  import LogoIcon from '../static/img/logo.png';
  import AllIcon from '../static/img/Sidebar/allIcon.svg';
  import LockIcon from '../static/img/Sidebar/lockIcon.svg';
  import NoteIcon from '../static/img/Sidebar/noteIcon.svg';
  import CardIcon from '../static/img/Sidebar/cardIcon.svg';
  import IdentityIcon from '../static/img/Sidebar/identityIcon.svg';
  import DeleteIcon from '../static/img/Sidebar/deleteIcon.svg';
  import LogoutIcon from '../static/img/Sidebar/logoutIcon.svg';


  let activeClass =
    "flex items-center p-2 text-base font-normal text-primary-900 bg-blue-600 dark:bg-primary-700 rounded-lg dark:text-white hover:bg-primary-100 dark:hover:bg-gray-700";
  let nonActiveClass =
    "flex items-center p-2 text-base font-normal text-black-900 rounded-lg dark:text-white hover:bg-blue-900 dark:hover:bg-blue-200";
  let deleteClass =
    "flex items-center p-2 text-base font-normal text-black-900 rounded-lg dark:text-white hover:bg-red-700 dark:hover:bg-blue-700";

  let activeItem = writable("All");
  let showSidebar = true;
  const dispatch = createEventDispatcher();

  function setActive(item: string) {
    activeItem.set(item);
    selectedEntryStore.set(null);
    filterStore.set({ filter: item });
  }

  function toggleSidebarVisibility() {
    showSidebar = !showSidebar;
  }
  
</script>

<!-- Maximized Sidebar -->
{#if showSidebar}
  <div
    in:slide={{ delay: 0, duration: 750, easing: cubicIn, axis: "x" }}
    out:slide={{ delay: 0, duration: 600, easing: sineIn, axis: "x" }}
    class="h-full"
  >
    <!-- Sidebar Content Container -->
    <Sidebar
      class="draggable inset-y-2 left-3 flex flex-col h-[calc(102%-2rem)] bg-gray-800 shadow-lg shadow-gray-900/50 rounded-xl"
    >
      <div class="flex flex-col flex-grow">
        <!-- Sidebar Header  -->
        <div class="flex-none">
          <SidebarGroup class="" ulClass="bg-blue-600 rounded-t-xl">
            <div class="flex w-full items-center rounded-lg">
              <img
                src={LogoIcon}
                alt="KeeManager"
                class="m-2 w-14 h-auto"
              />
              <span class="text-white text-lg font-medium">KeeManager</span>
              <label class="w-auto no-drag ml-10 mt-1">
                <div
                  class="w-9 h-10 cursor-pointer flex flex-col items-center justify-center"
                >
                  <input
                    class="hidden peer"
                    type="checkbox"
                    on:click={toggleSidebarVisibility}
                    checked
                  />
                  <div
                    class="w-[50%] h-[2px] bg-white rounded-sm transition-all duration-300 origin-left translate-y-[0.45rem] peer-checked:rotate-[-45deg]"
                  ></div>
                  <div
                    class="w-[50%] h-[2px] bg-white rounded-md transition-all duration-300 origin-center peer-checked:hidden"
                  ></div>
                  <div
                    class="w-[50%] h-[2px] bg-white rounded-md transition-all duration-300 origin-left -translate-y-[0.45rem] peer-checked:rotate-[45deg]"
                  ></div>
                </div>
              </label>
            </div>
          </SidebarGroup>
        </div>

        <!-- Main Sidebar Items -->
        <div class="flex-grow">
          <SidebarGroup
            class="no-drag"
            ulClass="mt-3 ml-1 mr-1 text-white space-y-1"
          >
            <SidebarItem
              label="All"
              on:click={() => setActive("All")}
              class={$activeItem === "All" ? activeClass : nonActiveClass}
            >
              <svelte:fragment slot="icon">
                <img class="w-6 h-6" src={AllIcon} alt="Logo" />
              </svelte:fragment>
            </SidebarItem>

            <SidebarItem
              label="Logins"
              on:click={() => setActive("Logins")}
              class={$activeItem === "Logins" ? activeClass : nonActiveClass}
            >
              <svelte:fragment slot="icon">
                <img class="w-6 h-6" src={LockIcon} alt="Logo" />
              </svelte:fragment>
            </SidebarItem>

            <SidebarItem
              label="Private Notes"
              on:click={() => setActive("Private Notes")}
              class={$activeItem === "Private Notes"
                ? activeClass
                : nonActiveClass}
            >
              <svelte:fragment slot="icon">
                <img class="w-6 h-6" src={NoteIcon} alt="Logo" />
              </svelte:fragment>
            </SidebarItem>

            <SidebarItem
              label="Credit Cards"
              on:click={() => setActive("Credit Cards")}
              class={$activeItem === "Credit Cards"
                ? activeClass
                : nonActiveClass}
            >
              <svelte:fragment slot="icon">
                <img class="w-6 h-6" src={CardIcon} alt="Logo" />
              </svelte:fragment>
            </SidebarItem>

            <SidebarItem
              label="Identities"
              on:click={() => setActive("Identities")}
              class={$activeItem === "Identities"
                ? activeClass
                : nonActiveClass}
            >
              <svelte:fragment slot="icon">
                <img
                  class="w-6 h-6"
                  src={IdentityIcon}
                  alt="Logo"
                />
              </svelte:fragment>
            </SidebarItem>
          </SidebarGroup>
        </div>
      </div>

      <!-- Sidebar Group at the Bottom -->
      <div class="flex-none mt-auto">
        <div class="ml-2 mr-2 border-t border-gray-600"></div>
        <SidebarGroup
          class="no-drag"
          ulClass="text-white mt-2 mb-2 ml-1 mr-1 space-y-1"
        >
          <SidebarItem
            label="Delete"
            on:click={() => dispatch("delete")}
            class={deleteClass}
          >
            <svelte:fragment slot="icon">
              <img class="w-6 h-6" src={DeleteIcon} alt="Logo" />
            </svelte:fragment>
          </SidebarItem>
          <SidebarItem
            label="Logout"
            on:click={() => dispatch("logout")}
            class={nonActiveClass}
          >
            <svelte:fragment slot="icon">
              <img class="w-6 h-6" src={LogoutIcon} alt="Logo" />
            </svelte:fragment>
          </SidebarItem>
        </SidebarGroup>
      </div>
    </Sidebar>
  </div>
<!-- Minimized Sidebar -->
{:else}
  <div
    in:fade={{ delay: 400, duration: 500 }}
    out:fly={{ delay: 0, duration: 400 }}
    class="h-full"
  >
  <!-- Sidebar Content Container -->
    <Sidebar
      class="draggable inset-y-2 left-3 flex flex-col w-auto h-[calc(102%-2rem)] bg-gray-800 shadow-lg shadow-gray-900/50 rounded-xl"
      asideClass="w-32"
    >
      <div class="flex flex-col flex-grow">
        <!-- Minimized Sidebar Header -->
        <div class="flex-none">
          <SidebarGroup class="" ulClass="bg-blue-600 rounded-t-xl">
            <div class="flex flex-colrounded-lg">
              <img
                src={LogoIcon}
                alt="KeeManager"
                class="mt-1 ml-1 mr-2 mb-1 w-14 h-auto mx-auto"
              />
            </div>
          </SidebarGroup>
        </div>

        <!-- Minimized Sidebar Main Items -->
        <div class="flex-grow">
          <SidebarGroup
            class="no-drag"
            ulClass="w-[60%] h-auto mx-auto mt-2 text-white space-y-1"
          >
            <label
              class="bg-blue-900 flex items-center mx-auto text-base font-normal text-black-900 rounded-lg dark:text-white hover:bg-blue-900 dark:hover:bg-blue-200"
            >
              <div
                class="w-10 h-10 cursor-pointer flex flex-col items-center justify-center"
              >
                <input
                  class="hidden peer"
                  type="checkbox"
                  on:click={toggleSidebarVisibility}
                />
                <div
                  class="w-[50%] h-[2px] bg-white rounded-sm transition-all duration-300 origin-left translate-y-[0.45rem] peer-checked:rotate-[-45deg]"
                ></div>
                <div
                  class="w-[50%] h-[2px] bg-white rounded-md transition-all duration-300 origin-center peer-checked:hidden"
                ></div>
                <div
                  class="w-[50%] h-[2px] bg-white rounded-md transition-all duration-300 origin-left -translate-y-[0.45rem] peer-checked:rotate-[45deg]"
                ></div>
              </div>
            </label>
            <SidebarItem
              on:click={() => setActive("All")}
              class={$activeItem === "All" ? activeClass : nonActiveClass}
            >
              <svelte:fragment slot="icon">
                <img class="w-6 h-6" src={AllIcon} alt="Logo" />
              </svelte:fragment>
            </SidebarItem>

            <SidebarItem
              on:click={() => setActive("Logins")}
              class={$activeItem === "Logins" ? activeClass : nonActiveClass}
            >
              <svelte:fragment slot="icon">
                <img class="w-6 h-6" src={LockIcon} alt="Logo" />
              </svelte:fragment>
            </SidebarItem>

            <SidebarItem
              on:click={() => setActive("Private Notes")}
              class={$activeItem === "Private Notes"
                ? activeClass
                : nonActiveClass}
            >
              <svelte:fragment slot="icon">
                <img class="w-6 h-6" src={NoteIcon} alt="Logo" />
              </svelte:fragment>
            </SidebarItem>

            <SidebarItem
              on:click={() => setActive("Credit Cards")}
              class={$activeItem === "Credit Cards"
                ? activeClass
                : nonActiveClass}
            >
              <svelte:fragment slot="icon">
                <img class="w-6 h-6" src={CardIcon} alt="Logo" />
              </svelte:fragment>
            </SidebarItem>

            <SidebarItem
              on:click={() => setActive("Identities")}
              class={$activeItem === "Identities"
                ? activeClass
                : nonActiveClass}
            >
              <svelte:fragment slot="icon">
                <img
                  class="w-6 h-6"
                  src={IdentityIcon}
                  alt="Logo"
                />
              </svelte:fragment>
            </SidebarItem>
          </SidebarGroup>
        </div>
      </div>

      <!-- Minimized Sidebar Group at the Bottom -->
      <div class="flex-none mt-auto">
        <div class="mb-2 ml-1 mr-1 border-t border-gray-600"></div>

        <SidebarGroup
          class="no-drag"
          ulClass="w-[60%] mx-auto text-white mb-2 space-y-1"
        >
          <SidebarItem on:click={() => dispatch("delete")} class={deleteClass}>
            <svelte:fragment slot="icon">
              <img class="w-6 h-6" src={DeleteIcon} alt="Logo" />
            </svelte:fragment>
          </SidebarItem>
          <SidebarItem on:click={() => dispatch("logout")} class={nonActiveClass}>
            <svelte:fragment slot="icon">
              <img class="w-6 h-6" src={LogoutIcon} alt="Logo" />
            </svelte:fragment>
          </SidebarItem>
        </SidebarGroup>
      </div>
    </Sidebar>
  </div>
{/if}

