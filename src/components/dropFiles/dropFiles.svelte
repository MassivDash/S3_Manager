<script lang="ts">
  import FileDrop from "svelte-tauri-filedrop";
  import { onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { open } from "@tauri-apps/api/dialog";
  import { appDir } from "@tauri-apps/api/path";
  import { listen } from "@tauri-apps/api/event";
  import { readDir } from "@tauri-apps/api/fs";
  import { fly } from "svelte/transition";
  import Select from "../select/select.svelte";
  import type { Bucket, TauriError } from "src/types";
  import type { FileEntry } from "@tauri-apps/api/fs";
  import AddFile from "../icons/addFile.svelte";
  import Close from "../icons/close.svelte";
  import Check from "../icons/check.svelte";
  import CircularProgress from "../circularProgress/circularProgress.svelte";
  import { showModal } from "../../store/modal";
  import { getFileName } from "../../lib/getFileName";

  import {
    dropFileFiles,
    dropFileVisible,
    dropFileLoading,
    dropFileProgressList,
  } from "../../store/dropFiles";

  let visible;
  let loading;
  let files: string[] = [];
  let currentMultiUploadFileName: string;
  let buckets: Bucket[] = [];
  let bucketName: string;
  let folderName: string;
  let uploadingFilesList: string[] = [];

  const _unsubscribeVisible = dropFileVisible.subscribe((value: boolean) => {
    visible = value;
  });

  const _unsubscribeFiles = dropFileFiles.subscribe((value: string[]) => {
    files = value;
  });

  const _unsubscribeLoading = dropFileLoading.subscribe((value: boolean) => {
    loading = value;
  });

  const _unsubscribeProgressList = dropFileProgressList.subscribe(
    (value: string[]) => {
      uploadingFilesList = value;
    }
  );

  interface Folder {
    name: string;
    files: string[];
  }

  $: folders =
    buckets &&
    buckets[findIndex(bucketName)]?.folders.map((folder) => folder.name);

  function getAllFiles(arr: FileEntry[]): string[] {
    const result = [];
    arr.forEach((file) => {
      if (!Object.prototype.hasOwnProperty.call(file, "children")) {
        result.push(file.name);
      } else {
        result.push(...getAllFiles(file.children));
      }
    });
    const res = result.filter((file: string) => !file.startsWith("."));
    return res as string[];
  }

  const filesUploaded = listen("event-upload-file", (event) => {
    dropFileProgressList.set([...uploadingFilesList, event.payload as string]);
  });

  const menuListener = listen("event-upload-menu-files", (_event): void => {
    appDir()
      .then((path) => {
        open({
          multiple: true,
          defaultPath: path,
        })
          .then((selected): void => {
            if (selected) {
              handleDrop([selected].flat()).catch((err: TauriError) => {
                showModal({
                  title: err.name,
                  message: err.message,
                  type: "error",
                })();
              });
            }
          })
          .catch((err: TauriError) => {
            showModal({
              title: err.name,
              message: err.message,
              type: "error",
            })();
          });
      })
      .catch((err: TauriError) => {
        showModal({
          title: err.name,
          message: err.message,
          type: "error",
        })();
      });
  });

  const menuFoldersListener = listen(
    "event-upload-menu-folders",
    (_event): void => {
      appDir()
        .then((path) => {
          open({
            directory: true,
            multiple: true,
            defaultPath: path,
          })
            .then((selected): void => {
              if (selected) {
                handleDrop([selected].flat()).catch((err: TauriError) => {
                  showModal({
                    title: err.name,
                    message: err.message,
                    type: "error",
                  })();
                });
              }
            })
            .catch((err: TauriError) => {
              showModal({
                title: err.name,
                message: err.message,
                type: "error",
              })();
            });
        })
        .catch((err: TauriError) => {
          showModal({
            title: err.name,
            message: err.message,
            type: "error",
          })();
        });
    }
  );

  const multipartUploadListen = listen(
    "event-multipart-upload-file",
    (event) => {
      if (event.payload !== "") {
        currentMultiUploadFileName = event.payload as string;
      } else {
        currentMultiUploadFileName = null;
      }
    }
  );

  let dirs: Folder[] = [];

  $: watchUpload =
    uploadingFilesList.length >= 1 ? uploadingFilesList.length : 1;
  $: dirsLength = dirs.length;
  $: totalFiles =
    dirsLength > 0
      ? dirs
          .map((folder) => folder.files.length)
          .reduce(
            (previousValue, currentValue) => previousValue + currentValue,
            0
          ) + (files.length - dirsLength > 0 ? files.length - dirsLength : 0)
      : files.length;
  // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-call
  $: progress = (
    ((Number(watchUpload) - 1) / Number(totalFiles)) *
    100
  ).toFixed(0);

  async function handleDrop(paths: string[]): Promise<void> {
    // This loads possible folder options
    const res: Bucket[] = await invoke("get_files");
    buckets = res;

    // User can drag as many folders and items before they submit
    dropFileFiles.set([...new Set([...files, ...paths])]);
    // eslint-disable-next-line @typescript-eslint/no-misused-promises
    [...new Set([...files, ...paths])].forEach(async (file: string) => {
      const getName = getFileName(file);
      if (!getName.includes(".")) {
        const files: FileEntry[] = await readDir(file, { recursive: true });
        const filesArray = getAllFiles(files);
        const preDir = [
          ...dirs,
          {
            name: file,
            files: filesArray,
          },
        ];
        dirs = [
          ...new Map(preDir.map((item) => [item["name"], item])).values(),
        ];
      }
    });
    dropFileVisible.set(true);
  }

  async function handleUpload(
    paths: string[],
    bucketName: string,
    folderName: string
  ): Promise<void> {
    try {
      dropFileLoading.set(true);
      const upload = await invoke("put_files", {
        bucketName,
        folderName,
        files: paths,
      });
      if (upload) {
        dirs = [];
        dropFileFiles.set([]);
        buckets = [];
        uploadingFilesList = [];
        dropFileLoading.set(false);
        dropFileVisible.set(false);
      }
    } catch (err) {
      const { name, message } = err as TauriError;
      dirs = [];
      dropFileFiles.set([]);
      buckets = [];
      dropFileProgressList.set([]);
      dropFileLoading.set(false);
      dropFileVisible.set(false);

      showModal({
        title: name,
        message: message,
        type: "error",
      })();
    }
  }

  function findIndex(bucketName: string): number {
    const index = buckets?.findIndex((bucket) => bucket.name === bucketName);
    return index;
  }

  onDestroy(async () => {
    const unlisten = await filesUploaded;
    const unlistenMenu = await menuListener;
    const unlistenMenuFolders = await menuFoldersListener;
    const unlistenMultipartUpload = await multipartUploadListen;
    unlisten();
    unlistenMenu();
    unlistenMenuFolders();
    unlistenMultipartUpload();
  });
</script>

<!-- eslint-disable-next-line @typescript-eslint/no-unused-vars, eslint-disable-next-line @typescript-eslint/no-unsafe-argument  -->
<FileDrop handleFiles={(paths) => handleDrop(paths)} let:files>
  <div data-testId="dropzone">
    <slot />
  </div></FileDrop
>
{#if visible}
  <div
    in:fly={{ x: 200, duration: 1000 }}
    out:fly={{ x: 200, duration: 1000 }}
    class="fixed overflow-y-auto bottom-0 pl-4 pt-2 pb-4 right-0 w-72 h-full z-50 bg-orange-50 dark:bg-slate-800 flex flex-col rounded-t shadow-sm justify-start items-stretch"
  >
    {#if loading}
      <div
        class="text-gray-800 dark:text-white flex justify-between items-center mb-4"
      >
        <h2>Uploading files</h2>
        <div
          class="text-gray-800 m-2 p-2 cursor-pointer dark:text-white bg-orange-50 dark:bg-slate-700 hover:bg-amber-100 hover:text-gray-50 hover:dark:bg-slate-900 hover:dark:text-orange-50"
        >
          <Close />
        </div>
      </div>
      <div class="w-full h-full flex flex-col justify-center items-center p-4">
        <CircularProgress progress={Number(progress)} />
        {#if uploadingFilesList.length === 0}
          <p class="text-xs m-8">starting up ...</p>
        {/if}
        {#if uploadingFilesList.length > 0}
          <p class="text-xs m-8">uploading:</p>
          <p class="text-xs m-8">
            {uploadingFilesList[uploadingFilesList.length - 1]}
          </p>
        {/if}
        {#if currentMultiUploadFileName}
          <p class="text-xs m-8">
            file above 100MB, multipart upload started for {currentMultiUploadFileName}
          </p>
        {/if}
      </div>
    {:else}
      <div
        class="text-gray-800 dark:text-white flex justify-between items-center mb-4"
      >
        <h2>Uploading files</h2>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div
          class="text-gray-800 m-2 p-2 cursor-pointer dark:text-white bg-orange-50 dark:bg-slate-700 hover:bg-amber-100 hover:text-gray-50 hover:dark:bg-slate-900 hover:dark:text-orange-50"
          on:click={() => {
            dropFileLoading.set(false);
            dropFileFiles.set([]);
            dropFileVisible.set(false);
            dirs = [];
            uploadingFilesList = [];
          }}
          role="button"
          tabindex="0"
        >
          <Close />
        </div>
      </div>
      <form
        on:submit|preventDefault={() =>
          handleUpload(files, bucketName, folderName)}
      >
        <Select
          bind:value={bucketName}
          label={"Choose bucket"}
          options={buckets.map((bucket) => bucket.name)}
          handleChange={() => (folderName = "")}
        />
        {#if bucketName}
          <Select
            bind:value={folderName}
            label={"Choose folder"}
            options={folders}
          />
        {/if}
        <div
          class="flex flex-col mt-8 pt-4 justify-center items-center text-gray-800 dark:text-white"
        >
          <AddFile width={48} height={48} />
          {#if dirsLength > 0}
            <p class="my-2">Total:</p>
            <p>
              {`${
                dirsLength > 1 ? `${dirsLength} directories` : "1 directory"
              }  with ${
                totalFiles -
                (files.length - dirsLength > 0 ? files.length - dirsLength : 0)
              }`} files
            </p>
          {/if}
          {#if files.length - dirsLength > 0}
            <p>
              {`${files.length - dirsLength} file${
                files.length - dirsLength > 1 ? "s" : ""
              }`}
            </p>
          {/if}
        </div>
        <div class="flex justify-center items-center w-full h-24">
          <button
            data-testId="upload-submit"
            disabled={folderName === ""}
            class="m-auto mt-8 flex justify-between p-5 w-32 text-gray-700 dark:text-white bg-orange-100 dark:bg-slate-600"
            type="submit"
          >
            <Check /> Upload
          </button>
        </div>
      </form>
    {/if}
  </div>
{/if}
