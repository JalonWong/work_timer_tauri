<script lang="ts">
  import type { Snippet } from "svelte";
  import BackIcon from "@iconify-svelte/mdi/chevron-left";

  interface Props {
    text: string;
    children?: Snippet;
  }
  let { text, children }: Props = $props();
</script>

<div class="sticky top-0 bg-base-100 px-2 pt-2">
  <div class="flex">
    <a href="/" class="btn btn-square h-7 btn-ghost"><BackIcon class="h-8" /></a>
    <h1 class="text-xl">
      {text}
    </h1>
    {#if children}
      {@render children()}
    {/if}
  </div>

  <div class="divider my-0"></div>
</div>
