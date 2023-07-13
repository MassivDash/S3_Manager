<script lang="ts">
  import { Router } from "svelte-navigator";
  import Layout from "./components/template/template.svelte";
  import Images from "./pages/images/images.svelte";
  import Image from "./pages/image/image.svelte";
  import Files from "./pages/files/files.svelte";
  import Movies from "./pages/movies/movies.svelte";
  import Buckets from "./pages/buckets/buckets.svelte";

  import "./tailwind.css";
  import FadeInRoute from "./components/fadeIn/fadeInRoute.svelte";
  import Modal from "./components/modal/modal.svelte";

  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";


  // When app is ready, close splashscreen
  onMount(async () => {
    await invoke("close_splashscreen");
  });
</script>

<Router>
  <Layout>
    <FadeInRoute path="/">
      <Files />
    </FadeInRoute>
    <FadeInRoute path="images/*">
      <FadeInRoute path="/">
        <Images />
      </FadeInRoute>
      <FadeInRoute path=":bucket/*key">
        <Image />
      </FadeInRoute>
    </FadeInRoute>
    <FadeInRoute path="movies">
      <Movies />
    </FadeInRoute>
    <FadeInRoute path="buckets"><Buckets /></FadeInRoute>
    <FadeInRoute path="settings">settings</FadeInRoute>
  </Layout>
  <Modal />
</Router>
