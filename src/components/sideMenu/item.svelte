<script lang="ts">
  import { Link, useLocation } from "svelte-navigator";
  export let path: string;
  export let name: string;
  const location = useLocation();
</script>

<Link
  to={path}
  class={`font-roboto text-xs flex flex-col justify-between items-center p-2 h-16 ${
    path === $location.pathname
      ? "text-amber-700 border-r-2 border-amber-600"
      : ""
  }`}
>
  <slot />
  <div class="text-current mt-2">{name}</div>
</Link>
