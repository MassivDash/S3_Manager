<script lang="ts">
  import { formatBytes } from "../../lib/date";
  import Checkbox from "./checkbox.svelte";
  export let files;
  export let bucketName;
  export let handleCheckbox;
  export let checkedFiles;
  const tableHead = ["Name", "Size", "Date Modified"];
</script>

<table class="table-auto w-full border-spacing-3 p-2 relative">
  <thead class="bg-orange-50 px-2 sticky top-32 z-10">
    <tr class="text-gray-600 text-left text-xs">
      <th />
      {#each tableHead as head}
        <th class="px-2 py-2">{head}</th>
      {/each}
    </tr>
  </thead>
  <tbody>
    {#each files as file}
      <tr class="text-gray-700 my-2 py-2 border-orange-100 border-b-2">
        <td id="checkbox" class="ml-4 flex items-bottom justify-center h-10">
          <Checkbox
            handleCheckbox={() => handleCheckbox(file.key, bucketName)}
            key={file.key}
            {checkedFiles}
          />
        </td>
        <td class="px-2 py-2 w-10/12">{file.name}</td>
        <td class="px-2 py-2 w-2/12">{formatBytes(file.size)}</td>
        <td class="px-2 py-2"
          >{new Date(file.last_modified * 1000)
            .toLocaleString()
            .split(",")[0]}</td
        >
      </tr>
    {/each}
  </tbody>
</table>

<style>
  #checkbox {
    width: 30px;
  }
</style>
