<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useFocus } from "svelte-navigator";
  import Loader from "../../components/loader/loader.svelte";
  import Search from "src/components/search/search.svelte";
  import FolderTitle from "src/components/table/folderTitle.svelte";
  import Table from "src/components/table/table.svelte";
  import IconButton from "src/components/iconButton/iconButton.svelte";
  import { open } from "@tauri-apps/api/dialog";
  import { appDir } from "@tauri-apps/api/path";
  // Open a selection dialog for directories
  import Download from "../../components/icons/download.svelte";
  import Delete from "../../components/icons/delete.svelte";
  import { fade } from "svelte/transition";
  import Tools from "../../components/tools/tools.svelte";

  interface File {
    key: string;
    name: string;
    folder: string;
    size: number;
    last_modified: number;
  }

  interface Folder {
    name: string;
    files: File[];
    total_files: number;
    total_size: number;
  }

  interface Bucket {
    name: string;
    folders: Folder[];
    total_files: number;
  }

  const registerFocus = useFocus();
  let response;
  let filteredList;
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

  let files: String[] = [];

  async function handleFilesSelect(bucketName, folderName) {
    const selected = await open({
      multiple: true,
      defaultPath: await appDir(),
    });

    files = [...selected];
    const upload = await invoke("put_files", {
      bucketName,
      folderName,
      files,
    });

    if (upload) {
      const res: Bucket[] = await invoke("get_files");
      response = res;
    }
  }

  onMount(async () => {
    const res: Bucket[] = await invoke("get_files");
    response = res;
  });

  interface CheckedFile{
    key: string;
    bucket_name: string;
  }

  let checkedFiles: CheckedFile[] = [];

  const handleCheckbox = (key: string, bucketName: string) => {
    const checked = {
      key,
      bucket_name: bucketName,
    };
    if (checkedFiles.includes(checked)) {
      checkedFiles = [...checkedFiles.filter((item) => item.key !== key)];
    } else {
      checkedFiles = [...checkedFiles, checked];
    }
  };
</script>

<div use:registerFocus class="outline-none relative">
  {#if !filteredList}
    <div class="flex justify-center items-center w-full h-screen">
      <Loader />
    </div>
  {/if}
  {#if filteredList && filteredList[0].name}
    <div
      class="fixed w-11/12 justify-between flex items-center h-20 top-0 bg-gray-100 z-30"
    >
      <Search
        bind:value
        hidelabel="true"
        class="placeholder-gray-700 bg-orange-50 appearance-none outline-none border-2 border-transparent border-spacing-1  focus:border-orange-600 rounded text-gray-900 p-2"
      />

      {#if checkedFiles && checkedFiles.length > 0}
      <Tools checkedFiles={checkedFiles} value={value} />
      {/if}
    </div>
    <div class="h-10" />
    {#each filteredList as bucket}
      <div
        class="flex pb-4 h-9 justify-start items-center my-4 bg-gray-100 sticky top-12 z-30"
      >
        <div class="w-1/4 h-1 rounded-md bg-gray-500" />
        <div class="w-1/4 text-center text-gray-500">
          bucket: {bucket.name}
        </div>
        <div class="h-1 w-2/4 rounded-md bg-gray-500" />
      </div>
      {#each bucket.folders as folder}
        <div class="bg-white">
          <FolderTitle
            folderName={folder.name}
            handleFilesSelect={() =>
              handleFilesSelect(bucket.name, folder.name)}
          />
          <div>
            <Table bucketName={bucket.name} files={folder.files} {handleCheckbox} {checkedFiles} />
          </div>
        </div>
      {/each}
    {/each}
  {/if}
</div>
