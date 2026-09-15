<script lang="ts">
  import { onMount } from "svelte";
  import { loadTheme } from "$lib/store";
  import {
    cmdGetTimerCount,
    cmdGetTimerStatus,
    cmdStartTimer,
    cmdStopTimer,
    cmdGetTimerList
  } from "$lib/gen";
  import type { TimerSetting } from "$lib/gen/types";
  import IconVar from "$lib/IconVar.svelte";

  let countString = $state("");
  let timerName = $state("");
  let isTimeout = $state(false);
  let limitMins = $state(0);
  let intervalId: number | null = null;
  let timerList: TimerSetting[] = $state([]);
  let totalTime = $state("0m");

  async function update() {
    ({ is_time_out: isTimeout, count_string: countString } = await cmdGetTimerCount());
  }

  async function updateTimerStatus() {
    let is_running: boolean;
    let total_time: number;
    ({
      is_running,
      name: timerName,
      limit_mins: limitMins,
      total_time
    } = await cmdGetTimerStatus());
    if (is_running) {
      if (intervalId === null) {
        intervalId = setInterval(() => {
          update();
        }, 500);
      }
    } else if (intervalId) {
      clearInterval(intervalId);
      intervalId = null;
    }

    const HOUR_SEC = 60 * 60;
    if (total_time >= HOUR_SEC) {
      totalTime = `${Math.floor(total_time / HOUR_SEC)}h${Math.floor((total_time % HOUR_SEC) / 60)}m`;
    } else {
      totalTime = `${Math.floor(total_time / 60)}m`;
    }
  }

  async function updateTimerList() {
    timerList = await cmdGetTimerList();
  }

  async function start(name: string) {
    await cmdStartTimer({ name });
    updateTimerStatus();
    update();
  }

  async function stop() {
    await cmdStopTimer();
    updateTimerStatus();
    update();
  }

  onMount(async () => {
    loadTheme();
    updateTimerList();
    updateTimerStatus();
    update();
  });
</script>

<main class="flex h-screen flex-col">
  <div class="flex grow flex-col items-center justify-center">
    <span class="font-mono text-[max(20px,18vw)] {isTimeout ? 'text-error' : ''}">
      {countString}
    </span>
    <span>Limit: {limitMins}m</span>
  </div>

  <div class="m-3">
    <div class="flex justify-evenly gap-1">
      {#each timerList as timer}
        <button
          onclick={() => (timerName === timer.name ? stop() : start(timer.name))}
          class="btn flex-auto {timerName === timer.name ? '' : 'btn-soft'} btn-primary"
        >
          <IconVar name={timer.icon} class="h-5" />{timer.name}
        </button>
      {/each}
    </div>
    <div class="mt-2 flex justify-center">
      <span class="text-base-content/60">Total Working Time: {totalTime}</span>
    </div>
  </div>
</main>
