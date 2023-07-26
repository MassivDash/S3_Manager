<script lang="ts">
  import { formatBytes, formatDate, isImage } from "../../lib";
  import Checkbox from "../checkbox/checkbox.svelte";
  import type { File, CheckedFile, VirtualListArray } from "src/types";
  import VirtualList from "../virtualList/virtualList.svelte";
  import { Link } from "svelte-navigator";
  import { invoke } from "@tauri-apps/api";
  import Preview from "../icons/preview.svelte";

  interface ImageObject {
    key: string;
    url: string;
  }

  export let files: VirtualListArray<File[]>;
  export let bucketName: string;
  export let handleCheckbox: (key: string, bucketName: string) => void;
  export let checkedFiles: CheckedFile[];
  let start;
  let end;

  let mouseOverImage: ImageObject | null = null;
  let handleMouseOver = async (
    key: string,
    bucketName: string
  ): Promise<void> => {
    const res: ImageObject = await invoke("get_image", {
      bucket: bucketName,
      key: key,
    });
    mouseOverImage = res;
  };

  export let height = "500px";
</script>

<!-- eslint-disable @typescript-eslint/no-unsafe-member-access, eslint-disable @typescript-eslint/no-unsafe-argument -->
{#if files.length > 0}
  <table class="table-auto w-full flex flex-col border-spacing-3 relative">
    <thead
      class="pl-4 bg-orange-50 dark:bg-slate-800 dark:text-white px-2 sticky top-[63px] z-10"
    >
      <tr class="text-left text-xs">
        <th />
        <th class="px-4" />
        <th class="p-2 w-8/12">Name</th>
        <th class="px-2 py-2 w-2/12">Size</th>
        <th class="px-2 py-2 w-2/12"><p class="ml-2">Date Modified</p></th>
      </tr>
    </thead>
    <tbody class="block relative w-full">
      <VirtualList items={files} {height} let:item bind:start bind:end>
        <tr
          class="flex items-center text-gray-700 dark:text-white my-2 py-2 border-orange-100 dark:border-slate-500 border-b-2"
          on:mouseover={() => (mouseOverImage = null)}
          on:focus={() => (mouseOverImage = null)}
        >
          <td id="checkbox" class="ml-4 flex items-bottom justify-center h-10">
            <Checkbox
              handleCheckbox={() => handleCheckbox(item.key, bucketName)}
              key={item.key}
              {checkedFiles}
            />
          </td>
          <td class="px-2 py-2 w-8/12 flex gap-4 align-middle items-center"
            ><Link
              on:mouseover={() => (mouseOverImage = null)}
              to="images/{bucketName}/{item.key}">{item.name}</Link
            >
            {#if isImage(item.key)}
              <div
                on:mouseover={() => handleMouseOver(item.key, bucketName)}
                on:focus={() => handleMouseOver(item.key, bucketName)}
                on:blur={() => (mouseOverImage = null)}
                on:mouseleave={() => (mouseOverImage = null)}
                on:mouseout={() => (mouseOverImage = null)}
                role="button"
                tabindex="0"
                class="hover:opacity-50"
              >
                <Preview width={24} height={24} />
              </div>
            {/if}
          </td>

          <td class="px-2 py-2 ml-2 w-2/12">{formatBytes(item.size)}</td>
          <td class="px-2 py-2 ml-2 w-2/12">{formatDate(item.last_modified)}</td
          >
          {#if mouseOverImage && mouseOverImage.key === item.key}
            <div
              class={"absolute border-2 border-white backdrop:blur-md bg-orange-50 block w-52 -mt-14 ml-[25vw] p-2"}
            >
              <img src={mouseOverImage.url} alt={mouseOverImage.key} />
            </div>
          {/if}
        </tr>
      </VirtualList>
      <p class="pl-6 text-sm p-2 border-t border-orange-500">
        showing items {start}-{end}
      </p>
    </tbody>
  </table>
{/if}

<style>
  #checkbox {
    width: 30px;
  }
</style>
