<script lang="ts">
  export let checkedFolders = [];
  export let handleCheckFolders: (folder: string) => void;
  export let folder = "";

  let checked = false;
  $: checked = checkedFolders.includes(folder);
</script>

<button
  data-testId="folder"
  class="flex items-center justify-center text-red-500"
  on:click={() => handleCheckFolders(folder)}
>
  <input
    type="checkbox"
    bind:checked
    class="m-2 h-4 w-4 accent-orange-600 rounded focus:outline-1 outline-orange-600 cursor-pointer"
  />
  <span>{folder}</span>
</button>
