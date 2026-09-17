<script lang="ts">
  import { onMount } from "svelte";
  import { gUserState, stopTimer } from "$lib/state.svelte";
  import { cmdGetTimerCount, cmdGetTimerStatus, cmdStartTimer, cmdStopTimer } from "$lib/gen";
  import type { TimerSetting } from "$lib/gen/types";
  import IconVar from "$lib/IconVar.svelte";

  let countString = $state("");
  let timerLabel = $state("");
  let isTimeout = $state(false);
  let limitMins = $state(0);
  let intervalId: number | null = null;
  let totalTime = $state("0m");

  async function update() {
    ({ is_time_out: isTimeout, count_string: countString } = await cmdGetTimerCount());
  }

  async function updateTimerStatus() {
    let is_running: boolean;
    let total_time: number;
    ({
      is_running,
      label: timerLabel,
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

  async function start(label: string) {
    await cmdStartTimer({ label, tag: gUserState.tag });
    updateTimerStatus();
    update();
  }

  async function stop() {
    stopTimer();
    updateTimerStatus();
    update();
  }

  onMount(async () => {
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

  <div class="flex justify-center">
    <span class="label mr-1">Tag:</span>
    <select bind:value={gUserState.tag} class="select">
      {#each gUserState.tags as tag}
        <option>{tag}</option>
      {/each}
    </select>
  </div>
  <div class="m-3">
    <div class="flex justify-evenly gap-1">
      {#each gUserState.timers as timer}
        <button
          onclick={() => (timerLabel === timer.label ? stop() : start(timer.label))}
          class="btn flex-auto {timerLabel === timer.label ? '' : 'btn-soft'} btn-primary"
        >
          {timer.label}
        </button>
      {/each}
    </div>
    <div class="mt-2 flex justify-center">
      <span class="text-base-content/60">Total Working Time: {totalTime}</span>
    </div>
  </div>
</main>
