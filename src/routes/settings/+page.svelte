<script lang="ts">
  import Header from "$lib/Header.svelte";
  import { setTheme, getTheme } from "$lib/state.svelte";
  import SunIcon from "@iconify-svelte/mdi/white-balance-sunny";
  import NightIcon from "@iconify-svelte/mdi/weather-night";
  import ComputerIcon from "@iconify-svelte/mdi/computer";

  let themes = [
    { name: "System", icon: ComputerIcon },
    { name: "Light", icon: SunIcon },
    { name: "Dark", icon: NightIcon }
  ];
</script>

<main>
  <Header text="Settings"></Header>
  <div class="mx-4">
    <h2 class="text-lg">Theme</h2>
    <div class="flex gap-3">
      {#each themes as theme}
        <button
          onclick={() => setTheme(theme.name)}
          class="btn {getTheme() === theme.name ? 'btn-primary' : ''}"
        >
          <theme.icon class="h-5" />{theme.name}
        </button>
      {/each}
    </div>
  </div>
</main>
