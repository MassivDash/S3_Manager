<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useFocus } from "svelte-navigator";
  import { open, confirm } from "@tauri-apps/api/dialog";
  import Loader from "src/components/loader/loader.svelte";

  import Tools from "src/components/tools/tools.svelte";
  import Scroller from "src/components/scroller/scroller.svelte";
  import BucketGrid from "./bucket.svelte";

  import type {
    ImageBucket,
    CheckedFile,
    GridCol,
    TauriError,
  } from "src/types";
  import { handleGrid, search } from "src/lib";
  // For error display
  import { showModal } from "src/store/modal";
  import { images, image_grid_option } from "src/store/images";

  const registerFocus = useFocus();

  let response: ImageBucket[];

  const _unsubscribeList = images.subscribe((value: ImageBucket[]) => {
    response = value;
  });

  let savedGridOption: GridCol;
  const _unsubscribeGridOption = image_grid_option.subscribe(
    (value: GridCol) => {
      savedGridOption = value;
    }
  );

  // Define loading for rust files call
  let loading = false;
  // Define resync operations (resync, delete)
  let resync = false;

  // Grid and js responsiveness
  let innerWidth;
  let gridCol: GridCol = savedGridOption ? savedGridOption : 3;

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
        // eslint-disable-next-line no-self-assign
        gridCol = gridCol;
    }
  }

  // On mount get the files from rust
  // Add grid responsiveness via resize listener
  onMount(() => {
    if (!response) {
      handleSync("load").catch((err: TauriError) => {
        showModal({ title: err.name, message: err.message, type: "error" });
      });
    }

    window.addEventListener("resize", onResize);
    //clean up on unmount
    return () => window.removeEventListener("resize", onResize);
  });

  onDestroy(() => {
    // Save user gird option
    image_grid_option.set(gridCol);
  });

  // Searchbar value and filters
  let value = "";

  let filteredList: ImageBucket[] = [];

  $: filteredList = search(response, value);

  // Checkboxes for download and delete
  let checkedFiles: CheckedFile[] = [];

  function resetCheckedFiles(): void {
    checkedFiles = [];
  }

  // User manual sync op button
  // Either we have initial load scenario "load"
  // Or manual sync up
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
      const { name, message } = err as TauriError;
      showModal({
        title: name,
        message: message,
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
      const { name, message } = err as TauriError;
      showModal({
        title: name,
        message: message,
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
          await handleSync("sync");
        } catch (err) {
          const { name, message } = err as TauriError;
          showModal({
            title: name,
            message: message,
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
    <div class="mr-10">
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
    {#if loading}
      <div class="flex justify-center items-center w-full h-screen">
        <Loader />
      </div>
    {:else}
      <Scroller>
        {#each filteredList as bucket (bucket.name)}
          <BucketGrid {bucket} {checkedFiles} {gridCol} {handleCheckbox} />
        {/each}
      </Scroller>
    {/if}
  {/if}
</div>
