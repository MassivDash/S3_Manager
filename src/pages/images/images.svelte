<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api';
  import { useFocus } from 'svelte-navigator';
  import Loader from '../../components/loader/loader.svelte';
  import Search from 'src/components/search/search.svelte';
  import GridImage from 'src/components/gridImage/gridImage.svelte';
  import { tick } from 'svelte';

  const registerFocus = useFocus();
  let response;
  let gridCol;

  $: gridCol = 3;
  let value = '';

  let filteredList;
  $: response;
  $: filteredList = response?.map(bucket => ({
    ...bucket,
    files:
      value === ''
        ? bucket.files
        : bucket.files.filter(item => item.name.indexOf(value) !== -1),
  }));

  const getTailwindClass = col => {
    switch (col) {
      case 1:
        return 'columns-1';
      case 2:
        return 'columns-2';
      case 3:
        return 'columns-3';
      case 4:
        return 'columns-4';
      case 5:
        return 'columns-5';
    }
  };

  interface Image {
    key: string;
    name: string;
    url: string;
    size: number;
    last_modified: number;
  }

  interface Bucket {
    name: string;
    files: Image[];
    total_files: number;
  }

  onMount(async () => {
    const res: Bucket[] = await invoke('get_all_images');
    response = res;
  });
</script>

<div use:registerFocus class="outline-none">
  {#if !filteredList}
    <div class="flex justify-center items-center w-full h-screen">
      <Loader />
    </div>
  {/if}
  {#if filteredList && filteredList[0].name}
    <div class="flex">
      <Search bind:value />
      <button
        on:click={async () => {
          gridCol = 1;
          await tick();
        }}>1</button
      >
      <button
        on:click={async () => {
          gridCol = 2;
          await tick();
        }}>2</button
      >
      <button
        on:click={async () => {
          gridCol = 3;
          await tick();
        }}>3</button
      >
      <button
        on:click={async () => {
          gridCol = 4;
          await tick();
        }}>4</button
      >
      <button on:click={() => (gridCol = 5)}>5</button>
    </div>
    {#each filteredList as bucket}
      <div class="flex h-9 justify-start items-center my-4">
        <div class="w-1/4 h-1 rounded-md bg-gray-500" />
        <div class="w-1/4 text-center text-gray-500">
          bucket: {bucket.name}
          {bucket.files.length > 0 ? `(${bucket.files.length})` : ''}
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
