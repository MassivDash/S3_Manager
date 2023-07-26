<script lang="ts">
  import type { Bucket } from "src/types";

  let visible = false;
  import AddFolder from "src/components/addFolder/addFolder.svelte";
  import NameDivider from "src/components/nameDivider/nameDivider.svelte";
  import { formatBytes } from "src/lib";
  export let handleSync: (string: string) => Promise<void>;
  export let bucket: Bucket;
  export let filesCounter: { [key: string]: number } = {};
</script>

<div class="flex justify-stretch items-stretch mr-4">
  <div class={!visible ? "w-[93%]" : "w-[60%]"}>
    <NameDivider
      label={`${bucket.name} (${filesCounter[bucket.name]}) ${formatBytes(
        bucket.total_size
      )}`}
    />
  </div>
  <div
    class={[
      !visible ? "w-[7%] duration-800 transition-all" : "w-[40%]",
      "flex justify-end items-center ",
    ].join(" ")}
  >
    <AddFolder
      bind:visible
      bucketName={bucket.name}
      handleSync={() => handleSync("sync")}
    />
  </div>
</div>
