<script lang="ts">
  import { formatBytes } from "../../lib/date";
  import Checkbox from "../checkbox/checkbox.svelte";
  import type { File, CheckedFile } from "src/types";
  import VirtualList from "../virtualList/virtualList.svelte";

  export let files: File[];
  export let bucketName: string;
  export let handleCheckbox: (key: string, bucketName: string) => void;
  export let checkedFiles: CheckedFile[];
  let start;
  let end;

  export let height = "500px";
</script>

{#if files.length > 0}
  <table class="table-auto w-full flex flex-col border-spacing-3 relative">
    <thead
      class="pl-4 bg-orange-50 dark:bg-slate-800 dark:text-white px-2 sticky top-32 z-10"
    >
      <tr class="text-left text-xs">
        <th />
        <th class="px-4" />
        <th class="p-2 w-8/12">Name</th>
        <th class="px-2 py-2 w-2/12">Size</th>
        <th class="px-2 py-2 w-2/12"><p class="ml-2">Date Modified</p></th>
      </tr>
    </thead>
    <tbody class="block relative w-full">
      <VirtualList items={files} {height} let:item bind:start bind:end>
        <tr
          class="flex items-center text-gray-700 dark:text-white my-2 py-2 border-orange-100 dark:border-slate-500  border-b-2"
        >
          <td id="checkbox" class="ml-4 flex items-bottom justify-center h-10">
            <Checkbox
              handleCheckbox={() => handleCheckbox(item.key, bucketName)}
              key={item.key}
              {checkedFiles}
            />
          </td>
          <td class="px-2 py-2 w-8/12">{item.name}</td>
          <td class="px-2 py-2 ml-2 w-2/12">{formatBytes(item.size)}</td>
          <td class="px-2 py-2 ml-2 w-2/12"
            >{new Date(item.last_modified * 1000)
              .toLocaleString()
              .split(",")[0]}</td
          >
        </tr>
      </VirtualList>
      <p class="pl-6 text-sm p-2 border-t border-orange-500">
        showing items {start}-{end}
      </p>
    </tbody>
  </table>
{/if}

<style>
  #checkbox {
    width: 30px;
  }
</style>
