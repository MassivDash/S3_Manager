<script lang="ts">
  export let key;
  export let checkedFiles = [];
  export let handleCheckbox;

  let checked = key
    ? checkedFiles.includes(key)
    : false;

</script>


<button class="flex items-center justify-center text-red-500">
  <input
    type="checkbox"
    bind:checked={checked}
    on:change={handleCheckbox}
    class="h-8 w-8 accent-orange-600 rounded focus:outline-1 outline-orange-600 cursor-pointer"
  />
</button>
