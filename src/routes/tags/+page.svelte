<script lang="ts">
  import Header from "$lib/Header.svelte";
  import { gUserState, newTag, moveTag, deleteTag } from "$lib/state.svelte";
  import TrashIcon from "@iconify-svelte/mdi/trash-can";
  import AddIcon from "@iconify-svelte/mdi/add";
  import UpIcon from "@iconify-svelte/mdi/chevron-up";
  import DownIcon from "@iconify-svelte/mdi/chevron-down";
</script>

<main>
  <Header text="Tags">
    <button class="btn ml-6 h-7" onclick={() => newTag()}>
      <AddIcon class="h-5" />
      Add a Tag
    </button>
  </Header>
  <div class="mx-4">
    <div class="flex flex-col gap-2">
      {#each gUserState.tags as tag, index}
        <div class="flex justify-center">
          <fieldset class="fieldset grow gap-3 rounded-box border border-base-300 bg-base-200 p-4">
            <div class="flex gap-1">
              <button class="btn btn-square h-8" onclick={() => moveTag(index, true)}>
                <UpIcon class="h-8" />
              </button>
              <button class="btn btn-square h-8" onclick={() => moveTag(index, false)}>
                <DownIcon class="h-8" />
              </button>
              <div class="grow"></div>
              <button class="btn btn-square h-8" onclick={() => deleteTag(index)}>
                <TrashIcon class="h-5" />
              </button>
            </div>
            <div class="flex">
              <label class="input grow text-base-content input-primary">
                <input type="text" class="grow" bind:value={tag} />
              </label>
            </div>
          </fieldset>
        </div>
      {/each}
    </div>
  </div>
</main>
