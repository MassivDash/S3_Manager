

<script lang="ts">
    export let files;
    import { formatBytes } from "../../lib/date";
    const tableHead = ["Name", "Size", "Date Modified"];
</script>
<table class="table-auto w-full border-spacing-3 p-2 relative">
    <thead class="bg-orange-50 px-2 sticky">
      <tr class="text-gray-600 text-left text-xs">
        {#each tableHead as head}
        <th class="px-2 py-2">{head}</th>
        {/each}
        <th></th>
      </tr>
    </thead>
    <tbody>
        {#each files as file}
        <tr class="text-gray-700 my-2 py-2 border-orange-100 border-b-2">
            <td class="px-2 py-2 w-4/6">{file.name}</td>
            <td class="px-2 py-2">{formatBytes(file.size)}</td>
            <td class="px-2 py-2">{new Date(file.last_modified * 1000).toLocaleString().split(",")[0]}</td>
            <td>
                <button>
                    <input type="checkbox" />
                </button>
            </td>
          </tr>
        {/each}
    </tbody>
  </table>