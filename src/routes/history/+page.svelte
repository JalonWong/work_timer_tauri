<script lang="ts">
  import { onMount } from "svelte";
  import Header from "$lib/Header.svelte";
  import type { HistoryInfo } from "$lib/gen/types";
  import { cmdGetHistory, cmdDeleteRecord, cmdModifyRecord } from "$lib/gen";
  import EditIcon from "@iconify-svelte/mdi/edit";
  import Modal from "$lib/Modal.svelte";

  const filters = ["1 Day", "7 Days", "All"];
  let selected_filter = $state("1 Day");
  let history: HistoryInfo[] = $state([]);
  let record: HistoryInfo | null = $state(null);
  let tmp_record = $state({ duration: 0, tag: "" });
  let showModal = $state(false);

  async function loadHistory(days: string) {
    let offsetDays = null;
    if (days === "1 Day") {
      offsetDays = 0;
    } else if (days === "7 Days") {
      offsetDays = -6;
    }
    history = await cmdGetHistory({ offsetDays, reverse: true });
  }

  onMount(async () => {
    loadHistory("1 Day");
  });
</script>

<main class="flex h-screen flex-col">
  <Header text="History"></Header>
  <div class="mx-3 mb-3 flex grow flex-col">
    <div class="flex">
      <div class="mt-2 flex gap-3">
        {#each filters as filter}
          <label>
            <input
              type="radio"
              class="radio radio-xs radio-primary"
              name="filter"
              value={filter}
              onclick={() => loadHistory(filter)}
              bind:group={selected_filter}
            />
            {filter}
          </label>
        {/each}
      </div>
      <div class="grow"></div>
      <button class="btn">Export to CSV</button>
    </div>
    <div class="table-pin-rows h-[calc(100vh-110px)] overflow-x-auto overflow-y-auto">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>Start Time</th>
            <th>Duration</th>
            <th>Tag</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          {#each history as item, i}
            <tr>
              <th>{item.start_time}</th>
              <td class="font-mono">{item.duration}</td>
              <td>{item.tag}</td>
              <td
                ><label class="cursor-pointer"
                  ><input
                    class="hidden"
                    onclick={() => {
                      record = item;
                      tmp_record.duration = item.duration_secs;
                      tmp_record.tag = item.tag;
                      showModal = true;
                    }}
                  /><EditIcon class="h-4" />
                </label></td
              >
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</main>

<Modal bind:showModal tittle="Modify a Record">
  <div class="py-4">
    {#if record !== null}
      <p>{record.start_time} | {record.duration} | {record.tag}</p>
      <div class="my-3 grid grid-cols-2 gap-4">
        <p>Duration in seconds:</p>
        <input type="number" class="input" bind:value={tmp_record.duration} />
        <p>Tag:</p>
        <input type="text" class="input" bind:value={tmp_record.tag} />
      </div>
    {/if}
  </div>

  <div class="flex justify-end gap-1">
    <button
      class="btn btn-soft btn-error"
      onclick={async () => {
        showModal = false;
        if (record) {
          await cmdDeleteRecord({ key: record.key });
          loadHistory(selected_filter);
        }
      }}>Delete</button
    >
    <div class="grow"></div>
    <button
      class="btn btn-soft btn-primary"
      onclick={async () => {
        showModal = false;
        if (record && tmp_record.tag) {
          await cmdModifyRecord({
            key: record.key,
            duration: tmp_record.duration,
            tag: tmp_record.tag
          });
          loadHistory(selected_filter);
        }
      }}>OK</button
    >
    <button class="btn btn-soft btn-primary" onclick={() => (showModal = false)}>Cancel</button>
  </div>
</Modal>
