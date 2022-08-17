<script lang="ts">
  import MenuItem from './Item.svelte';
  import Bucket from '../icons/bucket.svelte';
  import Home from '../icons/home.svelte';
  import Photo from '../icons/photo.svelte';
  import Movie from '../icons/movie.svelte';
  import Files from '../icons/files.svelte';
  import Settings from '../icons/settings.svelte';
  const width = 24;
  const height = 24;
</script>

<header>
  <nav class="flex gap-2 flex-col h-full text-gray-400 sticky">
    <div class="flex-1 flex justify-center items-center my-8">
      <p class="text-lg text-amber-600 bold">S3</p>
    </div>
    <MenuItem name="home" path="/">
      <Home {width} {height} />
    </MenuItem>
    <MenuItem name="files" path="/files">
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
    <MenuItem name="settings" path="/settings">
      <Settings {width} {height} />
    </MenuItem>
  </nav>
</header>
