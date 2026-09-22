<script lang="ts">
  import { onMount } from "svelte";
  import { gUserState, stopTimer } from "$lib/state.svelte";
  import { cmdGetTimerCount, cmdGetTimerStatus, cmdStartTimer, cmdTimeout } from "$lib/gen";

  let timerLabel = $state("");
  let limitMins = $state(0);
  let totalTime = $state("0m");

  onMount(async () => {
    updateTimerStatus();
    update();
  });

  async function update() {
    const { is_time_out, count_string } = await cmdGetTimerCount();
    gUserState.countString = count_string;
    if (gUserState.isTimeout !== is_time_out) {
      gUserState.isTimeout = is_time_out;
      if (is_time_out) {
        cmdTimeout();
      }
    }
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
      if (gUserState.intervalId === null) {
        gUserState.intervalId = setInterval(() => {
          update();
        }, 500);
      }
    } else if (gUserState.intervalId) {
      clearInterval(gUserState.intervalId);
      gUserState.intervalId = null;
    }

    const HOUR_SEC = 60 * 60;
    if (total_time >= HOUR_SEC) {
      totalTime = `${Math.floor(total_time / HOUR_SEC)}h${Math.floor((total_time % HOUR_SEC) / 60)}m`;
    } else {
      totalTime = `${Math.floor(total_time / 60)}m`;
    }
  }

  async function start(label: string) {
    await cmdStartTimer({ label });
    updateTimerStatus();
    update();
  }

  async function stop() {
    stopTimer();
    updateTimerStatus();
    update();
  }
</script>

<main class="flex h-screen flex-col">
  <div class="flex grow flex-col items-center justify-center">
    <span class="font-mono text-[max(20px,18vw)] {gUserState.isTimeout ? 'text-error' : ''}">
      {gUserState.countString}
    </span>
    <span>Limit: {limitMins}m</span>
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
