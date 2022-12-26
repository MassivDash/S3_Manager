<script lang="ts">
  import { Link } from "svelte-navigator";
  import { useLazyImage as lazyImage } from "svelte-lazy-image";
  import Checkbox from "../checkbox/checkbox.svelte";
  import { formatBytes } from "../../lib/date";

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
  class="h-[calc(500px+3.5rem)] overflow-hidden bg-orange-50 dark:bg-slate-700 rounded-md flex flex-col m-2  first:ml-0 last:mr-0 relative"
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
      <img
        data-src={url}
        alt={key}
        class="rounded-sm w-full object-cover min-h-[500px]"
        use:lazyImage
      />
    </div>
  </Link>
  <div class="absolute bottom-0 opacity-70 w-full">
    <Tags {key} bucket={bucket.name} />
  </div>
</div>
