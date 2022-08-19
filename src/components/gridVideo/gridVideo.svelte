<script lang="ts">
  import Checkbox from "../checkbox/checkbox.svelte";
  import { formatBytes } from "../../lib/date";

  import type { ImageBucket, CheckedFile } from "src/types";

  export let key: string;
  export let url: string;
  export let size: number;
  export let bucket: ImageBucket;
  export let name: string;
  export let checkedFiles: CheckedFile[];
  export let handleCheckbox: (key: string, bucketName: string) => void;

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
  class="min-h-[calc(200px+3.5rem)] overflow-hidden bg-orange-50 dark:bg-slate-700 rounded-md flex flex-col m-2"
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
    <p class="justify-self-end text-xs mt-1 mr-2">{formatBytes(size)}</p>
  </div>
  <video
    width="100%"
    height="100%"
    src={url}
    controls={true}
    muted={true}
    loop={false}
    autoplay={false}
  />
</div>
