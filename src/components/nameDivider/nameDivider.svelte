<script lang="ts">
  export let label = "";
</script>

<div
  class="flex pb-4 h-9 justify-start items-center my-4 bg-gray-100 sticky top-14 z-30"
>
  <div class="w-1/4 h-1 rounded-md bg-gray-500" />
  <div class="w-1/4 text-center text-gray-500">
    {label}
  </div>
  <div class="h-1 w-2/4 rounded-md bg-gray-500" />
</div>
