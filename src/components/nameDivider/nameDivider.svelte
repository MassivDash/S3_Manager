<script lang="ts">
  export let label = "";
</script>

<div
  class="flex bg-orange-500 rounded-sm h-14 justify-start items-center my-4 text-white sticky z-20"
>
  <div class="w-1/6 h-1 rounded-sm" />
  <div class="w-4/6 text-center">
    {label}
  </div>
  <div class="h-1 w-1/6 rounded-sm" />
</div>
