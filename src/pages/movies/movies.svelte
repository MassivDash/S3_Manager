<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useFocus } from "svelte-navigator";
  import { open, confirm } from "@tauri-apps/api/dialog";
  import Loader from "src/components/loader/loader.svelte";
  import NameDivider from "src/components/nameDivider/nameDivider.svelte";
  import GridVideo from "src/components/gridVideo/gridVideo.svelte";
  import Tools from "src/components/tools/tools.svelte";

  import type { ImageBucket, CheckedFile, GridCol } from "src/types";

  import { handleGrid } from "src/lib/grid";
  import { showModal } from "src/store/modal";
  import {
    movies,
    movies_grid_option,
    movies_scroll_index,
  } from "src/store/movies";

  import VirtualGrid from "src/components/virtualGrid/virtualGrid.svelte";

  const registerFocus = useFocus();
  let response: ImageBucket[];

  const _unsubscribe = movies.subscribe((value) => {
    response = value;
  });

  // Define loading for rust files call
  let loading = false;
  // Define resync operations (resync, delete)
  let resync = false;

  let savedGridOption: GridCol;
  const _unsubscribeGridOption = movies_grid_option.subscribe((value) => {
    savedGridOption = value;
  });

  // Grid and js responsiveness
  let innerWidth;
  let gridCol: GridCol = savedGridOption ? savedGridOption : 2;

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

  async function handleSync(type: "load" | "sync"): Promise<void> {
    const load = type === "load";

    try {
      if (load) {
        loading = true;
      }
      if (!load) {
        resync = true;
      }
      const res: ImageBucket[] = await invoke("get_all_movies");
      movies.set(res);
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
        await handleSync("sync");
      }
    }
  }

  onMount(async () => {
    !response && (await handleSync("load"));
    window.addEventListener("resize", onResize);
    //clean up on unmount
    return () => window.removeEventListener("resize", onResize);
  });

  let savedScroll: number;
  const _unsubscribeScroll = movies_scroll_index.subscribe((value) => {
    savedScroll = value;
  });

  let scrollToIndex;
  function scrollToItem(number: number): void {
    console.log(number);
    scrollToIndex(number);
  }

  // End item index needed for scroll restore
  let start; // first in view

  // virtual list mounted
  let realMount;

  $: if (realMount) {
    savedScroll && scrollToItem(savedScroll);
    movies_scroll_index.set(undefined);
  }

  onDestroy(() => {
    // Save scroll position
    movies_scroll_index.set(start);
    movies_grid_option.set(gridCol);
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
        {resync}
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
      {#each filteredList as bucket (bucket.name)}
        <NameDivider
          label={`bucket: ${bucket.name}
  ${bucket.files.length > 0 ? `(${bucket.files.length})` : ""}`}
        />
        <VirtualGrid
          items={bucket.files}
          length={bucket.files.length}
          {gridCol}
          bind:start
          bind:scrollToIndex
          bind:realMount
          let:gridCell
        >
          {#each gridCell as i (i.key)}
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
        </VirtualGrid>
      {/each}
    {/if}
  {/if}
</div>
