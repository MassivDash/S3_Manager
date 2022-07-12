<script lang="ts">
  import { Link } from "svelte-navigator";
  export let path;
  export let name;
  export let icon; 

  import bucket from '../icons/bucket.svg';
  import home from '../icons/home.svg'
  import photo from '../icons/photo.svg'

  const icons = {
    bucket,
    home,
    photo
  }
</script>

<Link to={path} class="flex flex-col justify-center items-center">
    {@html (icons[icon])}
    <div class="sideItem">{name}</div>
</Link>

<style>
    .sideItem {
        color: red;
    }
</style>
