<script lang="ts">
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  export let onClick: () => void = () => {};
  export let color: string | undefined = "";
</script>

<button
  on:click={onClick}
  type="button"
  class={color +
    " rounded bg-orange-50 p-4 dark:bg-slate-800 dark:text-white  gap-2 mr-4 flex border-0 appearance-none outline-orange-500 bg-none transition-all hover:bg-gray-50 hover:dark:bg-slate-700 hover:dark:text-orange-50 hover:text-gray-800 active:bg-gray-200"}
>
  <slot />
</button>
