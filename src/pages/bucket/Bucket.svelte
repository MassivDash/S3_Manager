<script lang="ts">
  import { useFocus } from "svelte-navigator";
  const registerFocus = useFocus();
</script>

<div use:registerFocus id="Bucket">Bucket</div>

<style>
  #Bucket {
    color: red;
  }
</style>
