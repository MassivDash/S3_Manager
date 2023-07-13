<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import Sync from "../icons/sync.svelte";
  import Close from "../icons/close.svelte";
  import IconButton from "../iconButton/iconButton.svelte";
  import Check from "../icons/check.svelte";
  import { getOS } from "../../lib/";

  export let key;
  export let bucket;
  let loading = true;

  interface Tag {
    key: string;
    value: string;
  }

  const os = getOS();

  let value = "";
  let tags: Tag[];
  $: tags;
  $: tagNames = tags?.map((tag) =>
    tag.value ? `${tag.key}-${tag.value}` : tag.key
  );

  let syncLoading = false;

  const handleSubmitTags = async (): Promise<void> => {
    syncLoading = true;
    try {
      await invoke("set_all_tags", {
        bucket: bucket,
        key: key,
        tagKeys: tagNames,
      });
      syncLoading = false;
    } catch (err) {
      tags = [];
      console.log(err);
      syncLoading = false;
      //TO DO: show errors to user
    }
  };

  const handleAddTag = (): void => {
    tags = [...tags, { key: value, value: undefined }];
    console.log;
    value = "";
  };

  const handleRemoveTag = (tag: string) => (): void => {
    const rmTag = tags.filter((t) => t.key !== tag);
    tags = [...rmTag];
  };

  onMount(async () => {
    try {
      const res: Tag[] = await invoke("get_all_tags", {
        bucket: bucket,
        key: key,
      });
      tags = res;

      loading = false;
    } catch (err) {
      console.log(err);
      loading = false;
      //TO DO: show errors to user
    }
  });
</script>

<div
  class="relative m-2 flex align-middle items-center justify-start flex-wrap"
>
  {#if os !== "Linux"}
    {#if !loading}
      <form data-testId="tags-form" on:submit|preventDefault={handleAddTag}>
        <label class="flex flex-wrap">
          {#if tagNames}
            {#each tagNames as tagName}
              <li
                class="rounded first:ml-0 first:mr-1 mx-2 my-1 h-6 text-xs flex items-center bg-orange-50 p-2 dark:bg-slate-800 dark:text-white border-0 appearance-none outline-orange-500 bg-none transition-all hover:bg-gray-50 hover:dark:bg-slate-700 hover:dark:text-orange-50 hover:text-gray-800 active:bg-gray-200"
              >
                <p class="mr-2">{tagName}</p>
                <span
                  id={tagName}
                  data-testId="remove-tag"
                  class="relative w-3 flex items-center justify-end cursor-pointer"
                  role="button"
                  tabindex="0"
                  on:keypress={handleRemoveTag(tagName)}
                  on:click={handleRemoveTag(tagName)}><Close width={16} /></span
                >
              </li>
            {/each}
          {/if}
          <input
            class="text-xs my-1 placeholder:text-xs h-6 w-20 bg-orange-50 placeholder:bg-orange-50 placeholder:dark:bg-slate-800 dark:bg-slate-800 dark:text-white p-2 gap-2 flex border-0 outline-orange-500 bg-none transition-all hover:bg-gray-50 hover:dark:bg-slate-700 hover:dark:text-orange-50 hover:text-gray-800 active:bg-gray-200 placeholder:italic mr-2 placeholder:text-slate-400 placeholder:dark:text-slate-50 rounded-sm py-2 pl-2 pr-1 focus:outline-none focus:border-slate-800 focus:ring-slate-700 focus:ring-1 sm:text-sm"
            type="text"
            bind:value
            name={"tag"}
            placeholder="Add tags"
          />
          <IconButton
            className="rounded mr-2 my-1 h-6 text-xs flex items-center gap-2  bg-orange-50 p-2 dark:bg-slate-800 dark:text-white border-0 appearance-none outline-orange-500 bg-none transition-all hover:bg-gray-50 hover:dark:bg-slate-700 hover:dark:text-orange-50 hover:text-gray-800 active:bg-gray-200"
            onClick={handleSubmitTags}
          >
            {#if syncLoading}
              <Sync width={18} /> Syncing
            {/if}
            {#if !syncLoading}
              <Check width={16} /> Submit
            {/if}
          </IconButton>
        </label>
      </form>
    {/if}
  {/if}
</div>
