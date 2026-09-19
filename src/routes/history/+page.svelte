<script lang="ts">
  import { onMount } from "svelte";
  import { save } from "@tauri-apps/plugin-dialog";
  import Header from "$lib/Header.svelte";
  import type { HistoryInfo } from "$lib/gen/types";
  import { cmdGetHistory, cmdDeleteRecord, cmdModifyRecord, cmdExportToCsv } from "$lib/gen";
  import EditIcon from "@iconify-svelte/mdi/edit";
  import Modal from "$lib/Modal.svelte";
  import { secsToString } from "$lib/state.svelte";

  const filters = ["1 Day", "7 Days", "All"];
  let selected_filter = $state("1 Day");
  let history: HistoryInfo[] = $state([]);
  let record: HistoryInfo | null = $state(null);
  let tmp_record = $state({ duration: 0, label: "" });
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

  async function exportToCsv() {
    const fileName = await save({
      filters: [
        {
          name: "*.csv",
          extensions: ["csv"]
        }
      ]
    });
    if (fileName) {
      cmdExportToCsv({ fileName });
    }
  }

  onMount(async () => {
    loadHistory("1 Day");
  });
</script>

<main class="flex h-screen flex-col">
  <Header text="History"
    ><div class="mr-10 ml-6 flex grow gap-3">
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

      <div class="grow"></div>
      <button class="btn h-7" onclick={() => exportToCsv()}>Export to CSV</button>
    </div></Header
  >
  <div class="mx-3 mb-3 flex grow flex-col">
    <div class="table-pin-rows h-[calc(100vh-110px)] overflow-x-auto overflow-y-auto">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>Label</th>
            <th>Start Time</th>
            <th>Duration</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          {#each history as item, i}
            <tr>
              <td>{item.l}</td>
              <th>{item.s}</th>
              <td class="font-mono">{secsToString(item.d)}</td>
              <td
                ><label class="cursor-pointer"
                  ><input
                    class="hidden"
                    onclick={() => {
                      record = item;
                      tmp_record.duration = item.d;
                      tmp_record.label = item.l;
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
      <p>{record.l} | {record.s} | {secsToString(record.d)}</p>
      <div class="my-3 grid grid-cols-3 gap-4">
        <div class="flex flex-col justify-center"><span>Duration:</span></div>
        <label class="input col-span-2 text-base-content/60">
          <input type="number" class="grow text-base-content" bind:value={tmp_record.duration} />
          seconds
        </label>
        <div class="flex flex-col justify-center"><span>Label:</span></div>
        <input type="text" class="input col-span-2" bind:value={tmp_record.label} />
      </div>
    {/if}
  </div>

  <div class="flex justify-end gap-1">
    <button
      class="btn btn-soft btn-error"
      onclick={async () => {
        showModal = false;
        if (record) {
          await cmdDeleteRecord({ key: record.k });
          loadHistory(selected_filter);
        }
      }}>Delete</button
    >
    <div class="grow"></div>
    <button class="btn btn-soft btn-primary" onclick={() => (showModal = false)}>Cancel</button>
    <button
      class="btn btn-soft btn-primary"
      onclick={async () => {
        showModal = false;
        if (record && tmp_record.label) {
          await cmdModifyRecord({
            key: record.k,
            duration: tmp_record.duration,
            label: tmp_record.label
          });
          loadHistory(selected_filter);
        }
      }}>OK</button
    >
  </div>
</Modal>
