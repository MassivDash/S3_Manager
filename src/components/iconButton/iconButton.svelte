<script lang="ts">
  export let onClick;
</script>

<button
  on:click={onClick}
  type="button"
  class="rounded bg-orange-50 p-4  gap-2 mr-4 flex border-0 appearance-none outline-orange-500 text-gray-600 bg-none transition-all hover:bg-gray-50 hover:text-gray-800 active:bg-gray-200"
>
  <slot />
</button>
