<script lang="ts">
  import { onMount } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import Header from "$lib/Header.svelte";
  import { setTheme, getTheme } from "$lib/state.svelte";
  import SunIcon from "@iconify-svelte/mdi/white-balance-sunny";
  import NightIcon from "@iconify-svelte/mdi/weather-night";
  import ComputerIcon from "@iconify-svelte/mdi/computer";
  import GitHubIcon from "@iconify-svelte/mdi/github";

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

<main class="flex h-screen flex-col">
  <Header text="Settings" />
  <div class="mx-4 flex grow flex-col">
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
      <legend class="fieldset-legend text-lg">Shortcuts</legend>
      <ul class="list-outside list-disc pl-5 text-lg">
        <li>
          Use <b>1 ~ 9</b> to start a timer. Starting a new timer will automatically stop the current
          one.
        </li>
        <li>Use <b>space</b> to stop the current timer.</li>
      </ul>
    </fieldset>
    <fieldset class="fieldset flex flex-col rounded-box border border-base-300 bg-base-200 p-4">
      <legend class="fieldset-legend text-lg">About</legend>
      <span class="text-lg">Version: v{version}</span>
      <div class="flex">
        <label class="flex link gap-0.5 text-lg text-blue-500">
          <div class="flex flex-col justify-center"><GitHubIcon class="h-6" /></div>
          GitHub
          <input
            type="button"
            onclick={() => {
              openUrl("https://github.com/JalonWong/work_timer_tauri");
            }}
          />
        </label>
      </div>
    </fieldset>
  </div>
</main>
