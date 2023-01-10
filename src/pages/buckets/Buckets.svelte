<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke, event } from "@tauri-apps/api";
  import { useFocus } from "svelte-navigator";
  import Loader from "src/components/loader/loader.svelte";
  import { formatDate } from "src/lib/date";
  // Open a selection dialog for directories
  import Tools from "src/components/tools/tools.svelte";
  import Scroller from "src/components/scroller/scroller.svelte";

  import { showModal } from "src/store/modal";
  import { buckets } from "src/store/buckets";

  interface BucketInfo {
    name: string;
    creation_date: number;
  }

  const registerFocus = useFocus();
  let response: BucketInfo[];
  let filteredList: BucketInfo[];
  let value = "";

  const _unsubscribe = buckets.subscribe((value) => {
    response = value;
  });

  $: filteredList = response?.filter(
    (bucket: BucketInfo) =>
      bucket.name.toLocaleLowerCase().indexOf(value.toLocaleLowerCase()) !== -1
  );

  onMount(async () => {
    if (!response) {
      await handleSync("load");
    }
  });

  const listenToFileUpload = event.listen("event-resync", () => {
    handleSync("sync");
  });

  onDestroy(async () => {
    const unlisten = await listenToFileUpload;
    unlisten();
  });

  let loading = false;
  let resync = false;

  //User manual sync op
  async function handleSync(type: "load" | "sync"): Promise<void> {
    const load = type === "load";
    try {
      if (load) {
        loading = true;
      }
      if (!load) {
        resync = true;
      }
      const res: BucketInfo[] = await invoke("get_buckets");
      buckets.set(res);
      console.log(res);
      if (load) {
        loading = false;
      }
      if (!load) {
        resync = false;
      }
    } catch (err) {
      if (load) {
        loading = false;
      }
      if (!load) {
        resync = false;
      }
      showModal({
        title: err.name,
        message: err.message,
        type: "error",
      })();
    }
  }
</script>

<div use:registerFocus class="outline-none">
  {#if loading}
    <div class="flex justify-center items-center w-full h-screen">
      <Loader />
    </div>
  {/if}
  {#if filteredList && filteredList[0].name}
    <div class="mr-7">
      <Tools {resync} {handleSync} bind:value />
    </div>
    <Scroller>
      <div
        class="grid gap-8 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 w-full"
      >
        {#each filteredList as bucket (bucket.name)}
          <div
            class="p-8 h-96  border-2 relative rounded-sm flex flex-col justify-between items-center"
          >
            <div class="w-full flex justify-start">
              <h2
                class="bg-white text-slate-900 font-extrabold
         text-[115px] font-roboto border-2 rounded-sm h-28 w-28 flex justify-center items-center"
              >
                {bucket.name.slice(0, 1).toLocaleUpperCase()}
              </h2>
            </div>
            <div>
              <h3 class="mt-8 text-lg">
                <span class="text-sm">name:</span>
                {bucket.name}
              </h3>
              <p>
                <span class="text-sm"> creation date: </span>
                <date>{formatDate(bucket.creation_date)}</date>
              </p>
            </div>
          </div>
        {/each}
      </div>
    </Scroller>
  {/if}
</div>
