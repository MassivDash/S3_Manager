<script lang="ts">
  import {Link} from 'svelte-navigator';
  import {useLazyImage as lazyImage} from 'svelte-lazy-image';
  import {formatBytes} from '../../lib/date';
  import More from '../../components/icons/more.svelte';
  import Heart from '../../components/icons/heart.svelte';
  export let key;
  export let url;
  export let size;
  export let last_modified;
  export let bucket;
  export let name;
  const width = 32;
  const height = 32;
</script>

<div class="px-6 py-2 bg-orange-50 rounded-md flex flex-col drop-shadow-xl m-2">
  <div class="flex w-full justify-between my-2">
    <button class="mb-4 text-orange-600">
      <Heart {width} {height} />
    </button>
    <button class="-mr-2 mb-4 text-gray-700">
      <More {width} {height} />
    </button>
  </div>

  <Link to="{bucket.name}/{key}" class="flex flex-col justify-center">
    <img
      data-src={url}
      alt={key}
      use:lazyImage
      class="rounded-sm object-cover"
    />

    <div class="text-center font-bold text-gray-600">{name}</div>
    <div class="flex justify-between items-center mt-2">
      <div>
        <p class="text-gray-800 font-bold">file size:</p>
        <p class="text-gray-700">{formatBytes(size)}</p>
      </div>
      <div class="text-right">
        <p class="text-gray-800 font-bold">last modified:</p>
        <p class="text-gray-700">
          {new Date(last_modified * 1000).toLocaleString().split(',')[0]}
        </p>
      </div>
    </div>
  </Link>
</div>
