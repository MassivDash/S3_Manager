<script lang="ts">
  import type { CheckedFile } from "src/types";

  export let key: string;
  export let checkedFiles: CheckedFile[] = [];
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let handleCheckbox: any;

  $: checked = key ? checkedFiles.some((item) => item.key === key) : false;
</script>

<button class="flex items-center justify-center text-red-500">
  <input
    type="checkbox"
    bind:checked
    on:change={handleCheckbox}
    class="m-2 h-4 w-4 accent-orange-600 rounded focus:outline-1 outline-orange-600 cursor-pointer"
  />
</button>
