<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useFocus } from "svelte-navigator";
  import { open, confirm } from "@tauri-apps/api/dialog";
  import Loader from "../../components/loader/loader.svelte";
  import NameDivider from "src/components/nameDivider/nameDivider.svelte";
  import GridImage from "src/components/gridImage/gridImage.svelte";
  import Tools from "../../components/tools/tools.svelte";

  import type { ImageBucket, CheckedFile } from "src/types";
  import VirtualList from "src/components/virtualList/virtualList.svelte";

  const registerFocus = useFocus();
  let response: ImageBucket[];

  type GridCol = 2 | 3 | 4;

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

  const height = "calc(100vh - 160px)";

  function chunkify<T>(a: T[], n: number, balanced: boolean): T[][] {
    console.log(a);
    if (n < 2) return [a];

    var len = a.length,
      out = [],
      i = 0,
      size;

    if (len % n === 0) {
      size = Math.floor(len / n);
      while (i < len) {
        out.push(a.slice(i, (i += size)));
      }
    } else if (balanced) {
      while (i < len) {
        size = Math.ceil((len - i) / n--);
        out.push(a.slice(i, (i += size)));
      }
    } else {
      n--;
      size = Math.floor(len / n);
      if (len % size === 0) size--;
      while (i < size * n) {
        out.push(a.slice(i, (i += size)));
      }
      out.push(a.slice(size * n));
    }

    return out;
  }

  const handleGrid = (): void => {
    switch (gridCol) {
      case 2:
        gridCol = 3;
        break;
      case 3:
        gridCol = 4;
        break;
      case 4:
        gridCol = 2;
        break;
    }
  };

  const getTailwindClass = (col): string => {
    switch (col) {
      case 2:
        return "grid-cols-2";
      case 3:
        return "grid-cols-3";
      case 4:
        return "grid-cols-4";
    }
  };

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
    console.log(res);
    response = res;
    console.log(
      chunkify(
        res[0].files,
        Number((res[0].files.length / gridCol).toFixed()),
        false
      )
    );
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
        {handleGrid}
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
              <GridImage
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
