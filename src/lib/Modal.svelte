<script>
  let { showModal = $bindable(), tittle, children } = $props();

  let dialog = $state(); // HTMLDialogElement

  $effect(() => {
    showModal ? dialog.showModal() : dialog.close();
  });
</script>

<dialog bind:this={dialog} class="modal">
  <div class="modal-box">
    <h2 class="text-lg font-bold">{tittle}</h2>
    {@render children?.()}
  </div>
</dialog>
