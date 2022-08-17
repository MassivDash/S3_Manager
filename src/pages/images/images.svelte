<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useFocus } from "svelte-navigator";
  import Loader from "../../components/loader/loader.svelte";
  import Search from "src/components/search/search.svelte";
  import GridImage from "src/components/gridImage/gridImage.svelte";

  import type { ImageBucket } from "src/types";

  const registerFocus = useFocus();
  let response: ImageBucket[];

  type GridCol = 1 | 2 | 3 | 4 | 5;

  let gridCol: GridCol = 1;

  $: gridCol = 3;
  let value = "";

  let filteredList: ImageBucket[] = [];
  $: response;
  $: filteredList = response?.map((bucket) => ({
    ...bucket,
    files:
      value === ""
        ? bucket.files
        : bucket.files.filter((item) => item.name.indexOf(value) !== -1),
  }));

  const handleColumnChange = (col: GridCol): void => {
    gridCol = col;
  };

  const getTailwindClass = (col: GridCol): string => {
    switch (col) {
      case 1:
        return "columns-1";
      case 2:
        return "columns-2";
      case 3:
        return "columns-3";
      case 4:
        return "columns-4";
      case 5:
        return "columns-5";
    }
  };

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
    <div class="flex">
      <Search bind:value />
      <button on:click={() => handleColumnChange(1)}>1</button>
      <button
        on:click={() => {
          gridCol = 2;
        }}>2</button
      >
      <button
        on:click={() => {
          gridCol = 3;
        }}>3</button
      >
      <button
        on:click={() => {
          gridCol = 4;
        }}>4</button
      >
      <button on:click={() => (gridCol = 5)}>5</button>
    </div>
    {#each filteredList as bucket}
      <div class="flex h-9 justify-start items-center my-4">
        <div class="w-1/4 h-1 rounded-md bg-gray-500" />
        <div class="w-1/4 text-center text-gray-500">
          bucket: {bucket.name}
          {bucket.files.length > 0 ? `(${bucket.files.length})` : ""}
        </div>
        <div class="h-1 w-2/4 rounded-md bg-gray-500" />
      </div>
      <div class={getTailwindClass(gridCol)}>
        {#each bucket.files as item}
          <GridImage
            name={item.name}
            key={item.key}
            url={item.url}
            size={item.size}
            last_modified={item.last_modified}
            {bucket}
          />{/each}
      </div>
    {/each}
  {/if}
</div>
