<script lang="ts">
  import "./layout.css";
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { loadSettings, saveSettings, stopTimer } from "$lib/state.svelte";
  import MenuIcon from "@iconify-svelte/mdi/menu";
  import HomeIcon from "@iconify-svelte/mdi/home";
  import SettingsIcon from "@iconify-svelte/mdi/settings";
  import HistoryIcon from "@iconify-svelte/mdi/clipboard-text-history-outline";
  import TimerIcon from "@iconify-svelte/mdi/timer";
  import TagIcon from "@iconify-svelte/mdi/tag";

  let { children } = $props();
  let showSidebar = $state(false);

  function toggleSidebar() {
    showSidebar = !showSidebar;
  }

  onMount(() => {
    void loadSettings();
    const unlisten = getCurrentWindow().onCloseRequested(async (event) => {
      event.preventDefault();
      try {
        await stopTimer();
        await saveSettings();
      } finally {
        await getCurrentWindow().destroy(); // actually close
      }
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  });
</script>

<div class="drawer drawer-end">
  <input id="my-drawer-1" type="checkbox" bind:checked={showSidebar} class="drawer-toggle" />
  <div class="drawer-content">
    <!-- Page content here -->
    <div class="fixed top-0 right-0 z-10 flex">
      <label
        for="my-drawer-1"
        class="cursor-pointer p-1 text-base-content/40 hover:text-base-content xs:p-3"
      >
        <MenuIcon class="h-5" />
      </label>
    </div>

    {@render children()}
  </div>
  <div class="drawer-side">
    <label for="my-drawer-1" aria-label="close sidebar" class="drawer-overlay"></label>
    <ul class="menu min-h-full w-40 bg-base-200 p-4 text-base">
      <!-- Sidebar content here -->
      <li><a href="/" onclick={toggleSidebar}><HomeIcon class="h-5" />Home</a></li>
      <li>
        <a href="/history" onclick={toggleSidebar}><HistoryIcon class="h-5" />History</a>
      </li>
      <li>
        <a href="/timers" onclick={toggleSidebar}><TimerIcon class="h-5" />Timers</a>
      </li>
      <li>
        <a href="/tags" onclick={toggleSidebar}><TagIcon class="h-5" />Tags</a>
      </li>
      <div class="grow"></div>
      <li>
        <a href="/settings" onclick={toggleSidebar}><SettingsIcon class="h-5" />Settings</a>
      </li>
    </ul>
  </div>
</div>
