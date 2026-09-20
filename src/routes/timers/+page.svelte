<script lang="ts">
  import { onDestroy } from "svelte";
  import Header from "$lib/Header.svelte";
  import { gUserState, newTimer, moveTimer, deleteTimer, saveSettings } from "$lib/state.svelte";
  import TrashIcon from "@iconify-svelte/mdi/trash-can";
  import AddIcon from "@iconify-svelte/mdi/add";
  import MoveUpIcon from "@iconify-svelte/mdi/chevron-up";
  import MoveDownIcon from "@iconify-svelte/mdi/chevron-down";
  import UpIcon from "@iconify-svelte/mdi/arrow-up";
  import DownIcon from "@iconify-svelte/mdi/arrow-down";

  onDestroy(() => saveSettings());
</script>

<main class="flex h-screen flex-col overflow-hidden">
  <Header text="Timers">
    <button class="btn ml-6 h-7" onclick={() => newTimer()}>
      <AddIcon class="h-5" />
      Add a Timer
    </button>
  </Header>

  <div class="m-4 mt-3 flex flex-col gap-2 overflow-auto">
    {#each gUserState.timers as timer, index}
      <div class="flex justify-center">
        <fieldset class="fieldset grow gap-3 rounded-box border border-base-300 bg-base-200 p-4">
          <!-- <legend class="fieldset-legend">Timer</legend> -->
          <div class="flex gap-1">
            <button class="btn btn-square h-8" onclick={() => moveTimer(index, true)}>
              <MoveUpIcon class="h-8" />
            </button>
            <button class="btn btn-square h-8" onclick={() => moveTimer(index, false)}>
              <MoveDownIcon class="h-8" />
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
            <div class="dropdown">
              <div tabindex="0" role="button" class="btn m-1 btn-soft btn-primary">
                {#if timer.count_up}<UpIcon class="h-5" />Count up{:else}<DownIcon
                    class="h-5"
                  />Count down{/if}
              </div>
              <ul
                tabindex="-1"
                class="menu dropdown-content z-1 w-52 rounded-box bg-base-100 p-2 shadow-sm"
              >
                <li>
                  <button
                    onclick={() => {
                      timer.count_up = true;
                      if (document.activeElement instanceof HTMLElement) {
                        document.activeElement.blur();
                      }
                    }}><UpIcon class="h-5" />Count up</button
                  >
                </li>
                <li>
                  <button
                    onclick={() => {
                      timer.count_up = false;
                      if (document.activeElement instanceof HTMLElement) {
                        document.activeElement.blur();
                      }
                    }}><DownIcon class="h-5" />Count down</button
                  >
                </li>
              </ul>
            </div>
          </div>
          <div class="flex gap-3">
            <span class="text-lg text-base-content">Timeout:</span>
            <label class="label text-base-content">
              <input
                type="checkbox"
                class="checkbox checkbox-primary"
                bind:checked={timer.play_a_sound}
              />
              Play sound
            </label>
            <label class="label text-base-content">
              <input
                type="checkbox"
                class="checkbox checkbox-primary"
                bind:checked={timer.notification}
              />
              Show toast
            </label>
            <label class="label text-base-content">
              <input
                type="checkbox"
                class="checkbox checkbox-primary"
                bind:checked={timer.show_window}
              />
              Show window
            </label>
          </div>
        </fieldset>
      </div>
    {/each}
  </div>
</main>
