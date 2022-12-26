<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import Tags from "svelte-tags-input";

  export let key;
  export let bucket;
  let loading = true;

  interface tag {
    key: string;
    value: string;
  }

  let tags: tag[];
  $: tags;
  $: tagNames = tags?.map((tag) =>
    tag.value ? `${tag.key}-${tag.value}` : tag.key
  );

  function handleTags(event): void {
    tags = event.detail.tags;
    console.log(tags);
  }

  onMount(async () => {
    const res: tag[] = await invoke("get_all_tags", {
      bucket: bucket,
      key: key,
    });
    tags = res;
    loading = false;

    console.log(res, tagNames);
  });
</script>

{#if !loading}
  <Tags
    on:tags={handleTags}
    tags={tagNames}
    placeholder={"Add tags here"}
    name={"custom-name"}
    id={"custom-id"}
    allowBlur={true}
  />
{/if}
