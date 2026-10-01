<script lang="ts">
  import { onMount } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import Header from "$lib/Header.svelte";
  import { setTheme, getTheme } from "$lib/state.svelte";
  import SunIcon from "~icons/mdi/white-balance-sunny";
  import NightIcon from "~icons/mdi/weather-night";
  import ComputerIcon from "~icons/mdi/computer";
  import GitHubIcon from "~icons/mdi/github";

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

<main class="flex h-screen flex-col overflow-hidden">
  <Header text="Settings" />
  <div class="mx-4 flex grow flex-col overflow-auto">
    <fieldset class="fieldset rounded-box border border-base-300 bg-base-200 p-4">
      <legend class="fieldset-legend text-lg">Theme</legend>
      <div class="flex gap-4">
        {#each themes as theme}
          <button
            onclick={() => setTheme(theme.value)}
            class="btn {getTheme() === theme.value ? 'btn-primary' : ''}"
          >
            <theme.icon />{theme.label}
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
          <div class="flex flex-col justify-center"><GitHubIcon /></div>
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
