<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useFocus } from "svelte-navigator";
  import { open, confirm } from "@tauri-apps/api/dialog";
  import Loader from "src/components/loader/loader.svelte";
  import NameDivider from "src/components/nameDivider/nameDivider.svelte";
  import GridImage from "src/components/gridImage/gridImage.svelte";
  import Tools from "src/components/tools/tools.svelte";

  import type { ImageBucket, CheckedFile, GridCol } from "src/types";
  import { handleGrid } from "src/lib/grid";
  import VirtualGrid from "src/components/virtualGrid/virtualGrid.svelte";

  const registerFocus = useFocus();
  let response: ImageBucket[];

  let loading = false;

  let gridCol: GridCol = 3;

  let value = "";

  let filteredList: ImageBucket[] = [];

  $: filteredList = response?.map((bucket) => ({
    ...bucket,
    files:
      value === ""
        ? bucket.files
        : bucket.files.filter((item) => item.name.indexOf(value) !== -1),
  }));

  let checkedFiles: CheckedFile[] = [];

  function resetCheckedFiles(): void {
    checkedFiles = [];
  }

  async function handleSync(): Promise<void> {
    loading = true;
    const res: ImageBucket[] = await invoke("get_all_images");
    response = res;
    loading = false;
  }

  const handleCheckbox = (key: string, bucketName: string): void => {
    const checked = {
      key,
      bucket_name: bucketName,
    };
    if (checkedFiles.some((item) => item.key === checked.key)) {
      checkedFiles = [...checkedFiles.filter((item) => item.key !== key)];
    } else {
      checkedFiles = [...checkedFiles, checked];
    }
  };

  async function handleDownload(checkedFiles: CheckedFile[]): Promise<void> {
    const dirPath = await open({
      directory: true,
      title: "Select a directory",
    });
    if (dirPath) {
      const success = await invoke("save_files", {
        keys: checkedFiles,
        dir: dirPath,
      });
      if (success) {
        resetCheckedFiles();
      }
    }
  }

  async function handleDelete(checkedFiles: CheckedFile[]): Promise<void> {
    const confirmed = await confirm(
      "This action cannot be reverted. Are you sure you want to delete?",
      { title: "Delete files ?", type: "warning" }
    );
    if (confirmed) {
      const success = await invoke("delete_files", { keys: checkedFiles });
      if (success) {
        resetCheckedFiles();
        loading = true;
        const res: ImageBucket[] = await invoke("get_all_images");
        response = res;
        loading = false;
      }
    }
  }

  onMount(async () => {
    const res: ImageBucket[] = await invoke("get_all_images");
    response = res;
  });
</script>

<!-- svelte-ignore non-top-level-reactive-declaration -->
<div use:registerFocus class="outline-none">
  {#if !filteredList}
    <div class="flex justify-center items-center w-full h-screen">
      <Loader />
    </div>
  {/if}
  {#if filteredList && filteredList[0].name}
    <div
      class="fixed w-11/12 justify-between flex items-center h-20 top-0 z-30"
    >
      <Tools
        handleGrid={() => (gridCol = handleGrid(gridCol))}
        {handleSync}
        {handleDownload}
        {handleDelete}
        {checkedFiles}
        bind:value
      />
    </div>
    <div class="h-14" />
    {#if loading}
      <div class="flex justify-center items-center w-full h-screen">
        <Loader />
      </div>
    {:else}
      {#each filteredList as bucket}
        <NameDivider
          label={`bucket: ${bucket.name}
      ${bucket.files.length > 0 ? `(${bucket.files.length})` : ""}`}
        />
        <VirtualGrid
          items={bucket.files}
          length={bucket.files.length}
          {gridCol}
          let:gridCell
        >
          {#each gridCell as i}
            <GridImage
              {handleCheckbox}
              {checkedFiles}
              name={i.name}
              key={i.key}
              url={i.url}
              size={i.size}
              last_modified={i.last_modified}
              {bucket}
            />
          {/each}
        </VirtualGrid>
      {/each}
    {/if}
  {/if}
</div>
