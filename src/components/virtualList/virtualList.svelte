<script lang="ts">
  import { onMount, tick } from "svelte";
  import type { VirtualListArray, VirtualListType } from "src/types";
  // props
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let items: VirtualListArray<any[]>;
  export let height = "100%";
  export let itemHeight: number | undefined = undefined;
  // read-only, but visible to consumers via bind:start
  export let start = 0;
  export let end = 0;
  export let realMount = false;
  let mounted = false;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export const item: VirtualListType<any> | undefined = undefined;

  //Connect to store

  // local state
  let height_map = [];
  let rows:
    | (HTMLCollectionOf<Element> & { offsetHeigh: number })
    | undefined[] = [];
  let viewport: {
    scrollTop: number;
    scrollTo: (ScrollToIndexOptions) => Promise<void>;
  };
  let contents: HTMLElement;
  let viewport_height = 0;
  let visible;
  let top = 0;
  let bottom = 0;
  let average_height;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  $: visible = items.slice(start, end).map((data: any, i: number) => {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
    return { index: i + start, data };
  });
  // whenever `items` changes, invalidate the current heightmap
  $: if (mounted)
    refresh(items, viewport_height, itemHeight).catch(console.error);

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  async function refresh(
    items: any[],
    viewport_height: number,
    itemHeight: number
  ): Promise<void> {
    const isStartOverflow = items.length < start;

    if (isStartOverflow) {
      await scrollToIndex(items.length - 1, { behavior: "auto" });
    }

    const { scrollTop } = viewport;
    await tick(); // wait until the DOM is up to date
    let content_height = top - scrollTop;
    let i = start;
    while (content_height < viewport_height && i < items.length) {
      let row = rows[i - start];
      if (!row) {
        end = i + 1;
        await tick(); // render the newly visible row
        row = rows[i - start];
      }
      const row_height: number = (height_map[i] =
        itemHeight || (row as Element & { offsetHeight: number }).offsetHeight);
      content_height += row_height;
      i += 1;
    }
    end = i;
    const remaining = items.length - end;
    average_height = (top + content_height) / end;
    bottom = remaining * average_height;
    height_map.length = items.length;
    if (!realMount) {
      realMount = true;
    }
  }
  function handle_scroll(): void {
    const { scrollTop } = viewport;
    for (let v = 0; v < rows.length; v += 1) {
      height_map[start + v] =
        itemHeight || (rows as { offsetHeight: number }[])[v].offsetHeight;
    }
    let i = 0;
    let y = 0;
    while (i < items.length) {
      const row_height =
        (height_map as number[])[i] || (average_height as number);
      if (y + row_height > scrollTop) {
        start = i;
        top = y;
        break;
      }
      y += row_height;
      i += 1;
    }
    while (i < items.length) {
      y += height_map[i] || average_height;
      i += 1;
      if (y > scrollTop + viewport_height) break;
    }
    end = i;
    const remaining = items.length - end;
    average_height = y / end;
    while (i < items.length) height_map[i++] = average_height as number;
    bottom = remaining * average_height; // Save last item scroll position

    // TODO if we overestimated the space these
    // rows would occupy we may need to add some
    // more. maybe we can just call handle_scroll again?
  }

  interface ScrollToIndexOptions {
    left?: number;
    top?: number;
    behavior?: "auto" | "smooth";
  }

  export async function scrollToIndex(
    index: number,
    opts: ScrollToIndexOptions
  ): Promise<void> {
    const { scrollTop } = viewport;
    const itemsDelta: number = index - start;
    const _itemHeight: number = itemHeight || (average_height as number);
    const distance = itemsDelta * _itemHeight;
    const scrollOpts: ScrollToIndexOptions = {
      left: 0,
      top: scrollTop + distance,
      behavior: "smooth",
      ...opts,
    };
    await viewport.scrollTo(scrollOpts);
  }

  // trigger initial refresh
  onMount(() => {
    rows = contents.getElementsByTagName(
      "svelte-virtual-list-row"
    ) as HTMLCollectionOf<Element> & { offsetHeigh: number };
    mounted = true;
  });
</script>

<svelte-virtual-list-viewport
  bind:this={viewport}
  bind:offsetHeight={viewport_height}
  on:scroll={handle_scroll}
  style="height: {height};"
  id="scroller-inner"
>
  <svelte-virtual-list-contents
    bind:this={contents}
    style="padding-top: {top}px; padding-bottom: {bottom}px;"
  >
    {#each visible as row (row.index)}
      <svelte-virtual-list-row>
        <slot item={row.data}>Missing template</slot>
      </svelte-virtual-list-row>
    {/each}
  </svelte-virtual-list-contents>
</svelte-virtual-list-viewport>

<style>
  svelte-virtual-list-viewport {
    position: relative;
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
    display: block;
  }
  svelte-virtual-list-contents,
  svelte-virtual-list-row {
    display: block;
  }
  svelte-virtual-list-row {
    overflow: hidden;
  }

  @media (prefers-color-scheme: dark) {
    #scroller-inner::-webkit-scrollbar {
      background-color: #10172a;
      width: 10px;
    }
    #scroller-inner::-webkit-scrollbar-thumb {
      background: #fff;
      border-radius: 10px;
      height: 15px;
      width: 5px;
    }
  }
</style>
