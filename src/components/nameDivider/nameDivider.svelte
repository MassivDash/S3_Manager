<script lang="ts">
  export let label = "";
</script>

<div
  class="flex rounded-sm bg-orange-500 rounded-top-sm h-[100%] justify-start items-center text-white sticky z-20 p-2"
>
  <div class="w-1/6 h-1 rounded-sm" />
  <div class="w-4/6 text-center">
    {label}
  </div>
  <div class="h-1 w-1/6 rounded-sm" />
</div>
