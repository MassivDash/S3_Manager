<script lang="ts">
  import Modal from "svelte-simple-modal";
  import { modal } from "src/store/modal";

  let currentModal;

  const _unsubscribe = modal.subscribe((value: string) => {
    currentModal = value;
  });
</script>

<Modal show={currentModal} />
