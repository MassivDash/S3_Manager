<script lang="ts">
  interface ImageObject {
    key: string;
    url: string;
  }
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useParams, navigate } from "svelte-navigator";
  import Tags from "../../components/tags/tags.svelte";
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
    <h1>Image key: {key}</h1>
    <img src={response.url} alt={response.key} />
    <Tags {key} {bucket} />
  </main>
{/if}
