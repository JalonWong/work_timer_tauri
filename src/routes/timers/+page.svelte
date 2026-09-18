<script lang="ts">
  import { onDestroy } from "svelte";
  import Header from "$lib/Header.svelte";
  import { gUserState, newTimer, moveTimer, deleteTimer, saveSettings } from "$lib/state.svelte";
  import TrashIcon from "@iconify-svelte/mdi/trash-can";
  import AddIcon from "@iconify-svelte/mdi/add";
  import UpIcon from "@iconify-svelte/mdi/chevron-up";
  import DownIcon from "@iconify-svelte/mdi/chevron-down";

  onDestroy(() => saveSettings());
</script>

<main>
  <Header text="Timers">
    <button class="btn ml-6 h-7" onclick={() => newTimer()}>
      <AddIcon class="h-5" />
      Add a Timer
    </button>
  </Header>
  <div class="mx-4">
    <div class="flex flex-col gap-2">
      {#each gUserState.timers as timer, index}
        <div class="flex justify-center">
          <fieldset class="fieldset grow gap-3 rounded-box border border-base-300 bg-base-200 p-4">
            <!-- <legend class="fieldset-legend">Timer</legend> -->
            <div class="flex gap-1">
              <button class="btn btn-square h-8" onclick={() => moveTimer(index, true)}>
                <UpIcon class="h-8" />
              </button>
              <button class="btn btn-square h-8" onclick={() => moveTimer(index, false)}>
                <DownIcon class="h-8" />
              </button>
              <div class="grow"></div>
              <button class="btn btn-square h-8" onclick={() => deleteTimer(index)}>
                <TrashIcon class="h-5" />
              </button>
            </div>
            <div class="flex">
              <label class="input grow text-base-content/60 input-primary">
                Label:
                <input type="text" class="grow text-base-content" bind:value={timer.label} />
              </label>
            </div>

            <div class="flex gap-3">
              <label class="input text-base-content/60 input-primary">
                Limit time:
                <input type="number" class="grow text-base-content" bind:value={timer.limit_time} />
                minutes
              </label>
              <label class="label text-base-content">
                <input
                  type="checkbox"
                  class="checkbox checkbox-primary"
                  bind:checked={timer.for_work}
                />
                Work timer
              </label>
              <label class="label text-base-content">
                <input
                  type="checkbox"
                  class="checkbox checkbox-primary"
                  bind:checked={timer.count_up}
                />
                Count up
              </label>
              <label class="label">
                <input
                  type="checkbox"
                  class="checkbox checkbox-primary"
                  bind:checked={timer.play_a_sound}
                />
                Play a sound on timeout
              </label>
            </div>
          </fieldset>
        </div>
      {/each}
    </div>
  </div>
</main>
