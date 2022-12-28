<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import Sync from "../icons/sync.svelte";
  import Close from "../icons/close.svelte";
  import IconButton from "../iconButton/iconButton.svelte";

  export let key;
  export let bucket;
  let loading = true;

  interface Tag {
    key: string;
    value: string;
  }

  let tag = "";
  let tags: Tag[];
  $: tags;
  $: tagNames = tags?.map((tag) =>
    tag.value ? `${tag.key}-${tag.value}` : tag.key
  );

  let syncLoading = false;

  async function handleSubmitTags(): Promise<void> {
    syncLoading = true;
    try {
      await invoke("set_all_tags", {
        bucket: bucket,
        key: key,
        tagKeys: tagNames,
      });
      syncLoading = false;
    } catch (err) {
      console.log(err);
      syncLoading = false;
      //TO DO: show errors to user
    }
  }

  function handleAddTag(): void {
    tags = [...tags, { key: tag, value: undefined }];
    tag = "";
  }

  const handleRemoveTag = (tag: string) => () => {
    console.log(tag);
    const rmTag = tags.filter((t) => t.key !== tag);

    tags = [...rmTag];
  };

  onMount(async () => {
    const res: Tag[] = await invoke("get_all_tags", {
      bucket: bucket,
      key: key,
    });
    tags = res;
    loading = false;
  });
</script>

{#if !loading}
  <div
    class="relative m-2 flex align-middle items-center justify-start flex-wrap"
  >
    <form on:submit|preventDefault={() => handleAddTag()}>
      <label class="flex flex-wrap">
        <ul class="flex flex-wrap">
          {#each tagNames as tagName}
            <li
              class="rounded first:ml-0 first:mr-0 mx-2 h-6 text-xs flex items-center  bg-orange-50 p-2 dark:bg-slate-800 dark:text-white border-0 appearance-none outline-orange-500 bg-none transition-all hover:bg-gray-50 hover:dark:bg-slate-700 hover:dark:text-orange-50 hover:text-gray-800 active:bg-gray-200"
            >
              <p class="mr-2">{tagName}</p>
              <span
                id={tagName}
                class="relative w-3 flex items-center justify-end"
                on:click={handleRemoveTag(tagName)}><Close width={16} /></span
              >
            </li>
          {/each}
        </ul>
        <input
          class="text-xs placeholder:text-xs h-6 w-28 dark:bg-slate-800 dark:text-white p-2  gap-2 flex border-0 appearance-none outline-orange-500 bg-none transition-all hover:bg-gray-50 hover:dark:bg-slate-700 hover:dark:text-orange-50 hover:text-gray-800 active:bg-gray-200 placeholder:italic mr-2 placeholder:text-slate-400 placeholder:dark:text-slate-50 bg-transparent rounded-sm   py-2 pl-2 pr-1 focus:outline-none focus:border-slate-800 focus:ring-slate-700 focus:ring-1 sm:text-sm"
          type="text"
          bind:value={tag}
          name={"tag"}
          placeholder="Add tags"
        />
      </label>
    </form>
    <IconButton
      className="rounded mr-2 h-6 text-xs flex items-center gap-2  bg-orange-50 p-2 dark:bg-slate-800 dark:text-white border-0 appearance-none outline-orange-500 bg-none transition-all hover:bg-gray-50 hover:dark:bg-slate-700 hover:dark:text-orange-50 hover:text-gray-800 active:bg-gray-200"
      onClick={() => handleSubmitTags()}
    >
      {#if syncLoading}
        syncing ...
      {/if}
      {#if !syncLoading}
        <Sync />Sync
      {/if}
    </IconButton>
  </div>
{/if}
