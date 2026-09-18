<script lang="ts">
  import { onMount } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
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
  let version = $state("0");

  onMount(async () => {
    version = await getVersion();
  });
</script>

<main>
  <Header text="Settings" />
  <div class="mr-4 ml-4 flex flex-col">
    <fieldset class="fieldset rounded-box border border-base-300 bg-base-200 p-4">
      <legend class="fieldset-legend text-lg">Theme</legend>
      <div class="flex gap-4">
        {#each themes as theme}
          <button
            onclick={() => setTheme(theme.value)}
            class="btn {getTheme() === theme.value ? 'btn-primary' : ''}"
          >
            <theme.icon class="h-5" />{theme.label}
          </button>
        {/each}
      </div>
    </fieldset>
    <fieldset class="fieldset rounded-box border border-base-300 bg-base-200 p-4">
      <legend class="fieldset-legend text-lg">About</legend>
      <span class="text-lg">Version: v{version}</span>
    </fieldset>
  </div>
</main>
