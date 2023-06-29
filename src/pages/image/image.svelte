<script lang="ts">
  interface ImageObject {
    key: string;
    url: string;
    size: number;
    last_modified: number;
  }
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useParams, navigate } from "svelte-navigator";
  import Tags from "src/components/tags/tags.svelte";
  import Loader from "src/components/loader/loader.svelte";
  import Back from "src/components/icons/back.svelte";
  import IconButton from "src/components/iconButton/iconButton.svelte";
  import { formatBytes, formatDate } from "src/lib/date";
  import { showModal } from "src/store/modal";
  import Scroller from "src/components/scroller/scroller.svelte";
  import type { TauriError } from "src/types";

  let bucket: string;
  let key: string;
  const params = useParams();
  $: {
    (bucket = $params.bucket), (key = $params.key);
  }
  let response: ImageObject;
  onMount(async () => {
    try {
      const res: ImageObject = await invoke("get_image", {
        bucket: bucket,
        key: key,
      });
      response = res;
    } catch (err) {
      const { name, message } = err as TauriError;
      showModal({
        title: name,
        message: message,
        type: "error",
      })();
    }
  });
</script>

{#if !response?.url}
  <Loader />
{:else}
  <main>
    <header class="flex h-16 gap-2 my-4 items-center justify-start">
      <IconButton onClick={() => navigate(-1)}><Back /> Go back</IconButton>
      <h1
        class="bg-orange-50 dark:bg-slate-800 h-14 px-4 flex items-center text-lg"
      >
        {key}
      </h1>
      <h2
        class="bg-orange-50 dark:bg-slate-800 h-14 px-4 flex items-center text-lg"
      >
        {formatBytes(response.size)}
      </h2>
      <h2
        class="bg-orange-50 dark:bg-slate-800 h-14 px-4 flex items-center text-lg"
      >
        {formatDate(response.last_modified)}
      </h2>
    </header>
    <Tags {key} {bucket} />
    <Scroller>
      <img src={response.url} alt={response.key} />
    </Scroller>
  </main>
{/if}
