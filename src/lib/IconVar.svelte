<script lang="ts">
  import AtomIcon from "@iconify-svelte/mdi/atom-variant";
  import StandIcon from "@iconify-svelte/mdi/human-handsup";
  import MeditationIcon from "@iconify-svelte/mdi/human-meditation";
  import TimerIcon from "@iconify-svelte/mdi/timer";

  const icons: Record<string, any> = {
    atom: AtomIcon,
    stand: StandIcon,
    meditation: MeditationIcon,
    timer: TimerIcon
  } as const;

  interface Props {
    name: string;
    class?: string;
  }
  let { name, class: className = "" }: Props = $props();
</script>

{#if true}
  {@const Icon = icons[name] ?? TimerIcon}
  <Icon class={className} />
{/if}
