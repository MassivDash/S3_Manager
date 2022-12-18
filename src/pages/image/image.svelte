<script lang="ts">
  interface ImageObject {
    key: string;
    url: string;
  }
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useParams, navigate } from "svelte-navigator";

  let bucket: string;
  let key: string;
  const params = useParams();
  $: {
    (bucket = $params.bucket), (key = $params.key);
  }
  let response: ImageObject;
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
  <main>
    <header>
      <nav>
        <button on:click={() => navigate(-1)}>Back</button>
      </nav>
    </header>
    <img src={response.url} alt={response.key} />
  </main>
{/if}
