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

  // For error display
  import { showModal } from "src/store/modal";
  import { images } from "src/store/images";

  const registerFocus = useFocus();
  let response: ImageBucket[];

  const _unsubscribe = images.subscribe((value) => {
    response = value;
  });

  // Define loading for rust files call
  let loading = false;
  // Define resync operations (resync, delete)
  let resync = false;

  // Grid and js responsiveness
  let innerWidth;
  let gridCol: GridCol = 3;

  // On user window resize, mimic css rwd pattern
  function onResize(): void {
    innerWidth = window.innerWidth;
    switch (true) {
      case innerWidth < 600 && innerWidth !== 0:
        gridCol = 1;
        break;
      case innerWidth > 600 && innerWidth < 900 && innerWidth !== 0:
        gridCol = 2;
        break;
      case innerWidth > 900 && innerWidth !== 0:
        gridCol = 3;
        break;
      default:
        gridCol = gridCol;
    }
  }

  // On mount get the files from rust
  // Add grid responsiveness via resize listener
  onMount(async () => {
    if (!response) {
      await handleSync("load");
    }
    window.addEventListener("resize", onResize);
    //clean up on unmount
    return () => window.removeEventListener("resize", onResize);
  });

  // Searchbar value and filters
  let value = "";

  let filteredList: ImageBucket[] = [];

  $: filteredList = response?.map((bucket) => ({
    ...bucket,
    files:
      value === ""
        ? bucket.files
        : bucket.files.filter((item) => item.name.indexOf(value) !== -1),
  }));

  // Checkboxes for download and delete
  let checkedFiles: CheckedFile[] = [];

  function resetCheckedFiles(): void {
    checkedFiles = [];
  }

  //User manual sync op
  async function handleSync(type: "load" | "sync"): Promise<void> {
    const load = type === "load";
    try {
      if (load) {
        loading = true;
      }
      if (!load) {
        resync = true;
      }
      const res: ImageBucket[] = await invoke("get_all_images");
      images.set(res);
      if (load) {
        loading = false;
      }
      if (!load) {
        resync = false;
      }
    } catch (err) {
      if (load) {
        loading = false;
      }
      if (!load) {
        resync = false;
      }
      showModal({
        title: err.name,
        message: err.message,
        type: "error",
      })();
    }
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

  // Download files directly to local machine
  async function handleDownload(checkedFiles: CheckedFile[]): Promise<void> {
    try {
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
    } catch (err) {
      showModal({
        title: err.name,
        message: err.message,
        type: "error",
      })();
    }
  }

  // Alert and remove files
  async function handleDelete(checkedFiles: CheckedFile[]): Promise<void> {
    const confirmed = await confirm(
      "This action cannot be reverted. Are you sure you want to delete?",
      { title: "Delete files ?", type: "warning" }
    );
    if (confirmed) {
      const success = await invoke("delete_files", { keys: checkedFiles });
      if (success) {
        resetCheckedFiles();
        try {
          handleSync("sync");
        } catch (err) {
          showModal({
            title: err.name,
            message: err.message,
            type: "error",
          })();
        }
      }
    }
  }
</script>

<svelte:window bind:innerWidth />

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
        {resync}
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
      {#each filteredList as bucket (bucket.name)}
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
          {#each gridCell as i (i.key)}
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
