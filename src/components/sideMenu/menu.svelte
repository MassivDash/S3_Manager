<script lang="ts">
  import MenuItem from "./item.svelte";
  import Bucket from "../icons/bucket.svelte";
  import Photo from "../icons/photo.svelte";
  import Movie from "../icons/movie.svelte";
  import Files from "../icons/files.svelte";
  const width = 24;
  const height = 24;
</script>

<header>
  <nav
    class="flex gap-2 flex-col h-full text-gray-600 dark:text-white dark:bg-slate-900 sticky"
  >
    <div class="flex-1 flex justify-center items-center my-8">
      <p class="text-lg font-audiowide text-amber-600 dark:text-amber-400 bold">
        S3
      </p>
    </div>
    <MenuItem name="files" path="/">
      <Files {width} {height} />
    </MenuItem>
    <MenuItem name="images" path="/images">
      <Photo {width} {height} />
    </MenuItem>
    <MenuItem name="movies" path="/movies">
      <Movie {width} {height} />
    </MenuItem>
    <MenuItem name="buckets" path="/buckets">
      <Bucket {width} {height} />
    </MenuItem>
  </nav>
</header>
