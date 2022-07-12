<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";
    import { Link } from "svelte-navigator";

      let response;
      onMount(async () => {
		const res = await invoke("get_all_images");
        response = res;
	});
</script>
  
<div> 
{#if response && response[0].name}
{#each response as bucket}
<div>{bucket.name}</div>
{#each bucket.files as image}
<Link to="{bucket.name}/{image.key}">{image.key}</Link>
{/each}

{/each}
{/if}
</div>
  

  