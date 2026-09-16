<script lang="ts">
  import Header from "$lib/Header.svelte";
  import { setTheme, getTheme } from "$lib/state.svelte";
  import SunIcon from "@iconify-svelte/mdi/white-balance-sunny";
  import NightIcon from "@iconify-svelte/mdi/weather-night";
  import ComputerIcon from "@iconify-svelte/mdi/computer";

  let themes = [
    { value: "", label: "System", icon: ComputerIcon },
    { value: "light", label: "Light", icon: SunIcon },
    { value: "dark", label: "Dark", icon: NightIcon }
  ];
</script>

<main>
  <Header text="Settings" />
  <div class="mx-4">
    <h2 class="text-lg">Theme</h2>
    <div class="flex gap-3">
      {#each themes as theme}
        <button
          onclick={() => setTheme(theme.value)}
          class="btn {getTheme() === theme.value ? 'btn-primary' : ''}"
        >
          <theme.icon class="h-5" />{theme.label}
        </button>
      {/each}
    </div>
  </div>
</main>
