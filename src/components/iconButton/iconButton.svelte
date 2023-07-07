<script lang="ts">
  export let dataTestId = "button";
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  export let onClick: () => void = () => {};
  export let color: string | undefined = "";
  export let type: "button" | "reset" | "submit" = "button";
  export let className =
    color +
    " rounded bg-orange-50 p-4 dark:bg-slate-800 dark:text-white  gap-2 flex border-0 appearance-none outline-orange-500 bg-none transition-all hover:bg-gray-50 hover:dark:bg-slate-700 hover:dark:text-orange-50 hover:text-gray-800 active:bg-gray-200";
</script>

<button data-testId={dataTestId} on:click={onClick} {type} class={className}>
  <slot />
</button>
