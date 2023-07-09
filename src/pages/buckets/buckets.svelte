<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke, event } from "@tauri-apps/api";
  import { useFocus } from "svelte-navigator";
  import Loader from "../../components/loader/loader.svelte";
  import { formatDate, formatBytes } from "../../lib";
  // Open a selection dialog for directories
  import Tools from "../../components/tools/tools.svelte";
  import Scroller from "../../components/scroller/scroller.svelte";
  import type { Bucket, TauriError } from "../../types";

  import { showModal } from "../../store/modal";
  import { buckets } from "../../store/buckets";
  import { files } from "../../store/files";

  interface BucketInfo {
    name: string;
    creation_date: number;
  }

  const registerFocus = useFocus();
  let response: BucketInfo[];
  let filteredList: BucketInfo[];
  let value = "";
  let loading = false;
  let resync = false;

  const _unsubscribe = buckets.subscribe((value: BucketInfo[]) => {
    response = value;
  });

  $: filteredList = response?.filter(
    (bucket: BucketInfo) =>
      bucket.name.toLocaleLowerCase().indexOf(value.toLocaleLowerCase()) !== -1
  );

  let filesInfoCall: Bucket[] | null = null;
  const _unsubscribeFiles = files.subscribe((value: Bucket[]) => {
    filesInfoCall = value;
  });

  onMount(async () => {
    if (!response) {
      await handleSync("load");
    }
  });

  const listenToFileUpload = event.listen("event-resync", () => {
    handleSync("sync")
      .then(() => {
        console.log("resynced");
      })
      .catch((err: TauriError) => {
        showModal({
          title: err.name,
          message: err.message,
          type: "error",
        })();
      });
  });

  onDestroy(async () => {
    const unlisten = await listenToFileUpload;
    unlisten();
  });

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
      const { name, message } = err as TauriError;

      showModal({
        title: name,
        message: message,
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
    <div class="mr-10 mt-2">
      <Tools {resync} {handleSync} bind:value />
    </div>
    <Scroller>
      <div
        class="grid gap-8 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 w-full"
      >
        {#each filteredList as bucket (bucket.name)}
          <div
            class="p-8 h-96 border-2 relative rounded-sm flex flex-col justify-between items-center"
          >
            <div class="w-full flex justify-start">
              <h2
                class="bg-white text-slate-900 font-extrabold
         text-[115px] font-audiowide border-2 rounded-sm h-28 w-28 flex justify-center items-center"
              >
                {bucket?.name.slice(0, 1).toLocaleUpperCase()}
              </h2>
            </div>
            {#if filesInfoCall}
              <div class="w-full flex flex-col">
                <p class="w-full">
                  <span class="text-sm"> total size: </span>
                  {formatBytes(
                    filesInfoCall.find((v) => v.name === bucket.name).total_size
                  )}
                </p>
                <p class="w-full m-0">
                  <span class="text-sm"> total files: </span>
                  {filesInfoCall.find((v) => v.name === bucket.name)
                    .total_files}
                </p>
              </div>
            {/if}
            <div class="w-full flex flex-col">
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
