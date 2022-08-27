<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useFocus } from "svelte-navigator";
  import { open, confirm } from "@tauri-apps/api/dialog";
  import Loader from "../../components/loader/loader.svelte";
  import NameDivider from "src/components/nameDivider/nameDivider.svelte";
  import GridVideo from "src/components/gridVideo/gridVideo.svelte";
  import Tools from "../../components/tools/tools.svelte";

  import type { ImageBucket, CheckedFile } from "src/types";

  import {
    getTailwindClass,
    handleGrid,
    GridCol,
    chunkify,
  } from "src/lib/grid";
  import VirtualList from "src/components/virtualList/virtualList.svelte";

  const registerFocus = useFocus();
  let response: ImageBucket[];

  let loading = false;

  let gridCol: GridCol = 3;
  const height = "calc(100vh - 160px)";

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
    const res: ImageBucket[] = await invoke("get_all_movies");
    response = res;
    loading = false;
  }

  const handleCheckbox = (key: string, bucketName: string): void => {
    const checked = {
      key,
      bucket_name: bucketName,
    };
    console.log(checkedFiles);
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
        const res: ImageBucket[] = await invoke("get_all_movies");
        response = res;
        loading = false;
      }
    }
  }

  onMount(async () => {
    const res: ImageBucket[] = await invoke("get_all_movies");
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
    <div class="h-10" />
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
        <VirtualList
          items={chunkify(
            bucket.files,
            Number((bucket.files.length / gridCol).toFixed()),
            false
          )}
          {height}
          let:item
        >
          <div class={`grid ${getTailwindClass(gridCol)} mr-4`}>
            {#each item as i}
              <GridVideo
                {handleCheckbox}
                {checkedFiles}
                name={i.name}
                key={i.key}
                url={i.url}
                size={i.size}
                {bucket}
              />
            {/each}
          </div>
        </VirtualList>
      {/each}
    {/if}
  {/if}
</div>
