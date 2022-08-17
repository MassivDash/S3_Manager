<script lang="ts">
  import Unfold from "../icons/unfold.svelte";
  export let options: string[];
  export let value: string;
  export let label: string;

  import IconButton from "../iconButton/iconButton.svelte";

  // eslint-disable-next-line @typescript-eslint/no-explicit-any, @typescript-eslint/no-empty-function
  export let handleChange: any = () => {};
</script>

<div class="flex flex-col text-gray-900 w-full">
  <label
    for={label}
    aria-label={label}
    class="w-full block mb-2 text-sm font-medium text-gray-900 dark:text-gray-400 required"
    >{label}</label
  >
  <div class="flex justify-center item-center">
    <select
      class="required appearance-none outline-none w-full bg-gray-50 text-gray-900 text-sm rounded-lg hover:border hover:ring-orange-500 hover:border-orange-500 block p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-orange-500 dark:focus:border-orange-500"
      bind:value
      on:change={handleChange}
    >
      {#each options as option}
        <option value={option}>
          {option}
        </option>
      {/each}
    </select>
    <IconButton>
      <Unfold />
    </IconButton>
  </div>
</div>
