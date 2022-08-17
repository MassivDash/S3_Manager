<script lang="ts">
  import {onMount} from 'svelte';
  import {invoke} from '@tauri-apps/api';
  import {useFocus} from 'svelte-navigator';
  import Loader from '../../components/loader/loader.svelte';
  import {open, confirm} from '@tauri-apps/api/dialog';
  import {appDir} from '@tauri-apps/api/path';
  // Open a selection dialog for directories
  import Tools from '../../components/tools/tools.svelte';
  import FileTable from 'src/components/fileTable/fileTable.svelte';
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
  let value = '';

  $: response;
  $: filteredList = response?.map((bucket: Bucket) => ({
    ...bucket,
    folders:
      value === ''
        ? [...bucket.folders]
        : bucket.folders.map((folder: Folder) => ({
            ...folder,
            files: folder.files.filter(item => item.name.indexOf(value) !== -1)
          }))
  }));

  let files: String[] = [];

  async function handleFilesSelect(bucketName, folderName) {
    const selected = await open({
      multiple: true,
      defaultPath: await appDir()
    });

    files = [...selected];
    const upload = await invoke('put_files', {
      bucketName,
      folderName,
      files
    });

    if (upload) {
      const res: Bucket[] = await invoke('get_files');
      response = res;
    }
  }

  onMount(async () => {
    const res: Bucket[] = await invoke('get_files');
    response = res;
  });

  interface CheckedFile {
    key: string;
    bucket_name: string;
  }

  let checkedFiles: CheckedFile[] = [];

  function resetCheckedFiles() {
    checkedFiles = [];
  }

  const handleCheckbox = (key: string, bucketName: string) => {
    const checked = {
      key,
      bucket_name: bucketName
    };
    if (checkedFiles.some(item => item.key === checked.key)) {
      checkedFiles = [...checkedFiles.filter(item => item.key !== key)];
    } else {
      checkedFiles = [...checkedFiles, checked];
    }
  };

  async function handleDownload(checkedFiles) {
    const dirPath = await open({
      directory: true,
      title: 'Select a directory'
    });
    if (dirPath) {
      const success = await invoke('save_files', {
        keys: checkedFiles,
        dir: dirPath
      });
      if (success) {
        resetCheckedFiles();
      }
    }
  }

  async function handleDelete(checkedFiles) {
    const confirmed = await confirm(
      'This action cannot be reverted. Are you sure you want to delete?',
      {title: 'Delete files ?', type: 'warning'}
    );
    if (confirmed) {
      const success = await invoke('delete_files', {keys: checkedFiles});
      if (success) {
        resetCheckedFiles();
        const res: Bucket[] = await invoke('get_files');
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
      class="fixed w-11/12 justify-between flex items-center h-20 top-0 bg-gray-100 z-30"
    >
      <Tools {handleDownload} {handleDelete} {checkedFiles} {value} />
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
