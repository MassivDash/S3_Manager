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
  export let visible = false;
  let uploading = false;
  let error = null;

  async function handleAddFolder(
    bucketName: string,
    value: string
  ): Promise<void> {
    if (value.length >= 1) {
      uploading = true;
      try {
        const res = await invoke("put_folder", {
          bucketName,
          key: value,
        });
        res && (await handleSync());
        uploading = false;
        visible = false;
      } catch (err) {
        console.log(err);
        uploading = false;
        visible = false;
      }
    } else {
      error = "Folder name must be at least 1 characters long";
    }
  }
</script>

<form
  class="flex gap-2 relative"
  on:submit|preventDefault={async () =>
    await handleAddFolder(bucketName, value)}
>
  {#if !visible}
    <IconButton
      className="rounded bg-orange-500 text-white p-4 dark:bg-orange-500 dark:text-white  gap-2 flex border-0 appearance-none outline-orange-500 bg-none transition-all hover:bg-gray-50 hover:dark:bg-slate-700 hover:dark:text-orange-50 hover:text-gray-800 active:bg-gray-200"
      dataTestId="add-folder-form"
      onClick={() => (visible = true)}
    >
      <AddFolder />
    </IconButton>
  {/if}
  {#if visible}
    <IconButton
      dataTestId="add-folder-close"
      onClick={() => {
        visible = false;
        value = "";
      }}
    >
      <Close />
    </IconButton>
    <input
      bind:value
      on:blur={() => (error = null)}
      on:focus={() => (error = null)}
      placeholder="Enter folder name"
      class={`placeholder-gray-500 h-14 border-orange-100 text-gray-900 dark:text-white bg-orange-50 dark:bg-slate-800 dark:placeholder-white appearance-none outline-none border-2 ${
        error ? "border-orange-500" : "border-transparent"
      } border-spacing-1 focus:border-orange-600 rounded p-2`}
    />
    {#if error}
      <p class="text-orange-500 bg-orange-50 rounded p-2 absolute mt-14 z-50">
        {error}
      </p>
    {/if}
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
