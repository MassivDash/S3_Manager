<script lang="ts">
  interface ImageObject {
    key: string;
    url: string;
  }
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useParams } from "svelte-navigator";

  let bucket;
  let key;
  const params = useParams();
  $: {
    bucket = $params.bucket,
    key = $params.key
  };
  let response;
  onMount(async () => {
    const res: ImageObject = await invoke("get_image", {
      bucket: bucket,
      key: key,
    });
    response = res;
  });

  
</script>

{#if !response?.url}
  <div>Loading...</div>
{:else}
  <img src={response.url} alt={response.key} />
{/if}

<style>
  #photo {
    color: red;
  }
</style>
