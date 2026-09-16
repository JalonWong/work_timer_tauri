<script lang="ts">
  import BackIcon from "@iconify-svelte/mdi/chevron-left";

  interface Props {
    text: string;
  }
  let { text }: Props = $props();
</script>

<div class="sticky top-0 bg-base-100 px-2 pt-2">
  <div class="flex">
    <a href="/" class="btn btn-square h-7 btn-ghost"><BackIcon class="h-8" /></a>
    <h1 class="text-xl">
      {text}
    </h1>
  </div>

  <div class="divider my-0"></div>
</div>
