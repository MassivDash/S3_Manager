<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import IconButton from "../iconButton/iconButton.svelte";
  import AddFolder from "../icons/addFolder.svelte";
  import Close from "../icons/close.svelte";
  import Check from "../icons/check.svelte";
  import Loader from "../loader/loader.svelte";
  export let bucketName = "";
  export let handleSync: () => Promise<void>;
  let value = "";
  let visible = false;
  let uploading = false;

  async function handleAddFolder(
    bucketName: string,
    value: string
  ): Promise<void> {
    uploading = true;
    const res = await invoke("put_folder", {
      bucketName,
      key: value,
    });
    res && (await handleSync());
    uploading = false;
    visible = false;
  }
</script>

<form
  class="flex gap-2"
  on:submit|preventDefault={async () =>
    await handleAddFolder(bucketName, value)}
>
  {#if !visible}
    <IconButton dataTestId="add-folder-form" onClick={() => (visible = true)}>
      <AddFolder />
    </IconButton>
  {/if}
  {#if visible}
    <IconButton dataTestId="add-folder-close" onClick={() => (visible = false)}>
      <Close />
    </IconButton>
    <input
      bind:value
      placeholder="Enter folder name"
      class="placeholder-gray-500 h-14 border-orange-100 text-gray-900 dark:text-white bg-orange-50 dark:bg-slate-800 dark:placeholder-white appearance-none outline-none border-2 border-transparent border-spacing-1 focus:border-orange-600 rounded p-2"
    />
    <IconButton
      type={"submit"}
      dataTestId="add-folder-form"
      onClick={async () => await handleAddFolder(bucketName, value)}
    >
      {#if uploading}
        <div class="flex flex-shrink h-6 justify-center items-center">
          <Loader width={24} height={24} />
        </div>
      {:else}
        <Check /> Submit
      {/if}
    </IconButton>
  {/if}
</form>
