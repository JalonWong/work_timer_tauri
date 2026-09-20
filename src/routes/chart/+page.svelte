<script lang="ts">
  import { onMount } from "svelte";
  import Header from "$lib/Header.svelte";
  import type { ChartData } from "$lib/gen/types";
  import { cmdGetHistoryForChart } from "$lib/gen";
  import { BarChart } from "layerchart";
  import { format, PeriodType } from "@layerstack/utils";
  import { secsToString } from "$lib/state.svelte";

  const filters = ["1 Day", "7 Days", "All"];
  let selected_filter = $state("7 Days");
  let data: ChartData[] = $state([]);

  async function loadHistory(days: string) {
    let offsetDays = null;
    if (days === "1 Day") {
      offsetDays = 0;
    } else if (days === "7 Days") {
      offsetDays = -6;
    }

    data = await cmdGetHistoryForChart({ offsetDays });
    // console.log("data: {}", data);
  }

  onMount(async () => {
    loadHistory(selected_filter);
  });
</script>

<main class="flex h-screen flex-col overflow-hidden">
  <Header text="Chart"
    ><div class="mr-10 ml-6 flex grow gap-3">
      {#each filters as filter}
        <label class="whitespace-nowrap">
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
    </div></Header
  >
  <div class="mx-4 flex grow flex-col justify-center overflow-auto">
    <BarChart
      {data}
      x="d"
      y="v"
      c="l"
      cRange={[
        "var(--color-primary)",
        "var(--color-secondary)",
        "var(--color-accent)",
        "var(--color-info)",
        "var(--color-success)",
        "var(--color-warning)",
        "var(--color-error)"
      ]}
      props={{
        xAxis: {
          format: (d) => format(new Date(d + "T00:00:00"), PeriodType.Day, { variant: "short" })
        },
        yAxis: {
          format: (t) => (t / 3600).toFixed(1) + "h"
        },
        tooltip: {
          header: {
            format: (d) => format(new Date(d + "T00:00:00"), PeriodType.Day, { variant: "long" })
          },
          item: {
            format: (t) => secsToString(t)
          }
        }
      }}
      legend
      height={300}
    />
  </div>
</main>
