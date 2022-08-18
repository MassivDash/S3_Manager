<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useFocus } from "svelte-navigator";
  import Loader from "../../components/loader/loader.svelte";
  import { open, confirm } from "@tauri-apps/api/dialog";
  import { appDir } from "@tauri-apps/api/path";
  // Open a selection dialog for directories
  import Tools from "../../components/tools/tools.svelte";
  import NameDivider from "src/components/nameDivider/nameDivider.svelte";
  import FileTable from "src/components/fileTable/fileTable.svelte";

  import type { Bucket, Folder, CheckedFile } from "src/types";

  const registerFocus = useFocus();
  let response: Bucket[];
  let filteredList: Bucket[];
  let value = "";

  $: response;
  $: filteredList = response?.map((bucket: Bucket) => ({
    ...bucket,
    folders:
      value === ""
        ? [...bucket.folders]
        : bucket.folders.map((folder: Folder) => ({
            ...folder,
            files: folder.files.filter(
              (item) => item.name.indexOf(value) !== -1
            ),
          })),
  }));

  $: bucketFiles = filteredList?.map((bucket) => ({
    [bucket.name]: bucket.folders.map((item) => item.files).flat().length,
  }))[0];

  onMount(async () => {
    const res: Bucket[] = await invoke("get_files");
    response = res;
  });

  let selectedFiles: string[] = [];

  async function handleFilesSelect(
    bucketName: string,
    folderName: string
  ): Promise<void> {
    const selected: string | string[] | null = await open({
      multiple: true,
      defaultPath: await appDir(),
    });

    if (selected) {
      selectedFiles = [...selected];
      const upload = await invoke("put_files", {
        bucketName,
        folderName,
        files: selectedFiles,
      });
      if (upload) {
        const res: Bucket[] = await invoke("get_files");
        console.log(res);
        response = res;
      }
    }
  }

  let checkedFiles: CheckedFile[] = [];

  function resetCheckedFiles(): void {
    checkedFiles = [];
  }

  async function handleSync(): Promise<void> {
    const res: Bucket[] = await invoke("get_files");
    response = res;
    console.log(res);
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
      const success = await invoke("save_files", {
        keys: checkedFiles,
        dir: dirPath,
      });
      if (success) {
        resetCheckedFiles();
      }
    }
  }

  async function handleDelete(checkedFiles: CheckedFile[]): Promise<void> {
    const confirmed = await confirm(
      "This action cannot be reverted. Are you sure you want to delete?",
      { title: "Delete files ?", type: "warning" }
    );
    if (confirmed) {
      const success = await invoke("delete_files", { keys: checkedFiles });
      if (success) {
        resetCheckedFiles();
        const res: Bucket[] = await invoke("get_files");
        console.log(res);
        response = res;
      }
    }
  }
</script>

<div use:registerFocus class="outline-none relative">
  {#if !filteredList}
    <div class="flex justify-center items-center w-full h-screen">
      <Loader />
    </div>
  {/if}
  {#if filteredList && filteredList[0].name}
    <div
      class="fixed w-11/12 justify-between flex items-center h-20 top-0 z-30 right-0"
    >
      <Tools
        {handleSync}
        {handleDownload}
        {handleDelete}
        {checkedFiles}
        bind:value
      />
    </div>
    <div class="h-10" />
    {#each filteredList as bucket}
      <NameDivider
        label={bucket.name + " " + "(" + bucketFiles[bucket.name] + ")"}
      />
      {#each bucket.folders as folder}
        <FileTable
          handleFilesSelect={() => handleFilesSelect(bucket.name, folder.name)}
          {folder}
          {bucket}
          {handleCheckbox}
          {checkedFiles}
        />
      {/each}
    {/each}
  {/if}
</div>
