<script lang="ts">
  import { onMount } from "svelte";
  import { gUserState, stopTimer } from "$lib/state.svelte";
  import { cmdGetTimerCount, cmdGetTimerStatus, cmdStartTimer, cmdPlayASound } from "$lib/gen";
  import {
    isPermissionGranted,
    requestPermission,
    sendNotification
  } from "@tauri-apps/plugin-notification";

  let timerLabel = $state("");
  let limitMins = $state(0);
  let totalTime = $state("0m");

  onMount(async () => {
    updateTimerStatus();
    update();
  });

  async function update() {
    const { is_time_out, count_string } = await cmdGetTimerCount();
    console.log("uu {} {}", is_time_out, count_string);
    gUserState.countString = count_string;
    if (gUserState.isTimeout != is_time_out) {
      gUserState.isTimeout = is_time_out;
      notify();
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

  async function notify() {
    const status = await cmdGetTimerStatus();

    // Do you have permission to send a notification?
    let permissionGranted = await isPermissionGranted();

    // If not we need to request it
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }

    // Once permission has been granted we can send the notification
    // if (permissionGranted) {
    //   sendNotification({
    //     title: "Tauri",
    //     body: "Tauri is awesome!"
    //   });
    // }

    if (status.play_a_sound) {
      cmdPlayASound();
    }
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
