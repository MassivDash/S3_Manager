<script lang="ts">
  import { Link } from "svelte-navigator";
  import { useLazyImage as lazyImage } from "svelte-lazy-image";
  import Checkbox from "../checkbox/checkbox.svelte";
  import { formatBytes } from "../../lib/date";
  import Error from "../icons/error.svelte";

  import Tags from "../tags/tags.svelte";
  import type { ImageBucket, CheckedFile } from "src/types";

  export let key: string;
  export let url: string;
  export let size: number;
  export let last_modified: number;
  export let bucket: ImageBucket;
  export let name: string;
  export let checkedFiles: CheckedFile[];
  export let handleCheckbox: (key: string, bucketName: string) => void;
  export let imgError = false;

  // const tags = aws_tags.map(tag => tag.value ? `${tag.key}-${tag.value}` :  tag.key)

  function shortenName(string: string): string {
    if (string.length > 20) {
      return (
        string.substring(0, 9) +
        "..." +
        string.split(".")[string.split(".").length - 1]
      );
    }
    return string;
  }
</script>

<div
  class="h-[calc(500px+3.5rem)] overflow-hidden bg-orange-50 dark:bg-slate-700 rounded-sm flex flex-col m-2  first:ml-0 last:mr-0 relative"
>
  <div class="flex items-center justify-between">
    <div class="bg-orange-50 dark:bg-slate-700 p-2 h-14 flex items-center">
      <Checkbox
        handleCheckbox={() => handleCheckbox(key, bucket.name)}
        {key}
        {checkedFiles}
      />
      <p class="ml-0">{shortenName(name)}</p>
    </div>
    <div class="justify-self-end text-xs mt-1 mr-2">
      <p>{formatBytes(size)}</p>
      <p>{new Date(last_modified * 1000).toLocaleString().split(",")[0]}</p>
    </div>
  </div>
  <Link to="{bucket.name}/{key}" class="flex flex-col justify-center">
    <div>
      {#if !imgError}
        <img
          data-src={url}
          alt={key}
          class="rounded-sm w-full object-cover min-h-[500px]"
          use:lazyImage
          on:error={() => {
            imgError = true;
          }}
        />
      {/if}
      {#if imgError}
        <div class="p-4 mt-4 flex gap-4 h-full w-full  text-red-700">
          <div><Error width={36} height={36} /></div>
          <div class="text-sm">
            <h3>Image not loaded</h3>
            <p>Most probably the presigned url has expired.</p>
            <p>Try to resync</p>
          </div>
        </div>
      {/if}
    </div>
  </Link>
  <div class="absolute bottom-0 opacity-70 w-full">
    <Tags {key} bucket={bucket.name} />
  </div>
</div>
