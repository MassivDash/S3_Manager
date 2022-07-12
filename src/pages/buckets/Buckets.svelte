<script lang="ts">
  import { useFocus } from "svelte-navigator";
  const registerFocus = useFocus();
</script>

<div use:registerFocus id="Buckets">Buckets</div>

<style>
  #Buckets {
    color: red;
  }
</style>
