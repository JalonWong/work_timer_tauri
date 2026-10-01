<script lang="ts">
  import "./layout.css";
  import { onMount } from "svelte";
  import { onStart, onExit, stopTimer } from "$lib/state.svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import MenuIcon from "~icons/mdi/menu";
  import HomeIcon from "~icons/mdi/home";
  import SettingsIcon from "~icons/mdi/settings";
  import HistoryIcon from "~icons/mdi/clipboard-text-history-outline";
  import TimerIcon from "~icons/mdi/timer";
  import ChartIcon from "~icons/mdi/chart-bar-stacked";

  let { children } = $props();
  let showSidebar = $state(false);

  function toggleSidebar() {
    showSidebar = !showSidebar;
  }

  onMount(() => {
    void onStart();
    const unlisten = getCurrentWindow().onCloseRequested(async (event) => {
      event.preventDefault();
      try {
        await stopTimer();
        await onExit();
      } finally {
        await getCurrentWindow().destroy();
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
        <MenuIcon class="h-6 w-6" />
      </label>
    </div>

    {@render children()}
  </div>
  <div class="drawer-side">
    <label for="my-drawer-1" aria-label="close sidebar" class="drawer-overlay"></label>
    <ul class="menu min-h-full w-40 bg-base-200 p-4 text-lg">
      <!-- Sidebar content here -->
      <li>
        <a href="/" onclick={toggleSidebar}><HomeIcon />Home</a>
      </li>
      <li>
        <a href="/chart" onclick={toggleSidebar}><ChartIcon />Chart</a>
      </li>
      <li>
        <a href="/history" onclick={toggleSidebar}><HistoryIcon />History</a>
      </li>
      <li>
        <a href="/timers" onclick={toggleSidebar}><TimerIcon />Timers</a>
      </li>
      <div class="grow"></div>
      <li>
        <a href="/settings" onclick={toggleSidebar}><SettingsIcon />Settings</a>
      </li>
    </ul>
  </div>
</div>
