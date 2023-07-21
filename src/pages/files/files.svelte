<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke, event } from "@tauri-apps/api";
  import { useFocus } from "svelte-navigator";
  import Loader from "src/components/loader/loader.svelte";
  import { open, confirm } from "@tauri-apps/api/dialog";
  import { appDir } from "@tauri-apps/api/path";
  import { searchWithFolders } from "src/lib";
  // Open a selection dialog for directories
  import Tools from "src/components/tools/tools.svelte";
  import FileTable from "src/components/fileTable/fileTable.svelte";
  import Scroller from "src/components/scroller/scroller.svelte";

  import Title from "./bucketTitle.svelte";

  import type { Bucket, CheckedFile, TauriError } from "src/types";

  import { showModal } from "src/store/modal";
  import { files } from "src/store/files";

  const registerFocus = useFocus();
  let response: Bucket[];
  let filteredList: Bucket[];
  let value = "";

  const _unsubscribe = files.subscribe((value: Bucket[]) => {
    response = value;
  });

  $: filteredList = searchWithFolders(response, value);

  $: filesCounter = filteredList?.reduce(
    (acc, bucket) => ({
      ...acc,
      [bucket.name]: bucket.folders.map((item) => item.files).flat().length,
    }),
    {}
  ) as { [key: string]: number };

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

  let selectedFiles: string[] = [];
  let loading = false;
  let resync = false;

  async function handleFilesSelect(
    bucketName: string,
    folderName: string
  ): Promise<void> {
    const selected: string | string[] | null = await open({
      multiple: true,
      defaultPath: await appDir(),
    });

    if (selected) {
      try {
        selectedFiles = [...selected];
        const upload = await invoke("put_files", {
          bucketName,
          folderName,
          files: selectedFiles,
        });
        if (upload) {
          await handleSync("sync");
        }
      } catch (err) {
        const { name, message } = err as TauriError;
        showModal({
          title: name,
          message: message,
          type: "error",
        })();
      }
    }
  }

  let checkedFiles: CheckedFile[] = [];

  function resetCheckedFiles(): void {
    checkedFiles = [];
  }

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
      const res: Bucket[] = await invoke("get_files");
      files.set(res);
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

  const handleCheckbox = (key: string, bucketName: string): void => {
    const checked = {
      key,
      bucket_name: bucketName,
    };
    if (checkedFiles.some((item) => item.key === checked.key)) {
      checkedFiles = [...checkedFiles.filter((item) => item.key !== key)];
    } else {
      checkedFiles = [...checkedFiles, checked];
    }
  };

  async function handleDownload(checkedFiles: CheckedFile[]): Promise<void> {
    const dirPath = await open({
      directory: true,
      title: "Select a directory",
    });
    if (dirPath) {
      resync = true;
      const success = await invoke("save_files", {
        keys: checkedFiles,
        dir: dirPath,
      });
      if (success) {
        resetCheckedFiles();
        resync = false;
      }
    }
  }

  async function handleDelete(checkedFiles: CheckedFile[]): Promise<void> {
    const confirmed = await confirm(
      "This action cannot be reverted. Are you sure you want to delete?",
      { title: "Delete files ?", type: "warning" }
    );
    if (confirmed) {
      resync = true;
      try {
        const success = await invoke("delete_files", { keys: checkedFiles });
        if (success) {
          resetCheckedFiles();
          await handleSync("sync");
        }
      } catch (err) {
        resync = false;
        const { name, message } = err as TauriError;
        showModal({
          title: name || "Delete files failed",
          message: message,
          type: "error",
        })();
      }
    }
  }

  async function handleFolderDelete(
    bucketName: string,
    value: string
  ): Promise<void> {
    const prepareFolder = {
      key: value + "/",
      bucket_name: bucketName,
    };
    const confirmed = await confirm(
      "This action cannot be reverted. Are you sure you want to delete?",
      { title: "Delete folder with all files inside ?", type: "warning" }
    );
    if (confirmed) {
      resync = true;
      try {
        const res = await invoke("delete_folders", {
          keys: [prepareFolder],
        });
        if (res) {
          await handleSync("sync");
        }
      } catch (err) {
        resync = false;
        const { name, message } = err as TauriError;
        showModal({
          title: name || "Delete files failed",
          message: message,
          type: "error",
        })();
      }
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
    <div class="mr-10">
      <Tools
        {resync}
        {handleSync}
        {handleDownload}
        {handleDelete}
        {checkedFiles}
        bind:value
      />
    </div>
    <Scroller>
      {#each filteredList as bucket (bucket.name)}
        <Title {handleSync} {bucket} {filesCounter} />
        <div class="mr-4 mt-4">
          {#each bucket.folders as folder (folder.name)}
            <FileTable
              handleFolderDelete={() =>
                handleFolderDelete(bucket.name, folder.name)}
              handleFilesSelect={() =>
                handleFilesSelect(bucket.name, folder.name)}
              {folder}
              {bucket}
              {handleCheckbox}
              {checkedFiles}
            />
          {/each}
        </div>
        <div class="h-5" />
      {/each}
    </Scroller>
  {/if}
</div>
