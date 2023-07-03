<script lang="ts">
  export let label = "";
</script>

<div
  class="flex bg-orange-500 rounded-top-sm h-10 justify-start items-center text-white sticky z-20 mt-2"
>
  <div class="w-1/6 h-1 rounded-sm" />
  <div class="w-4/6 text-center">
    {label}
  </div>
  <div class="h-1 w-1/6 rounded-sm" />
</div>
