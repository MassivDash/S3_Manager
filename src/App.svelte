<script lang="ts">
  import { Router, Route } from "svelte-navigator";
  import Layout from "./components/template/template.svelte";
  import Images from "./pages/images/images.svelte";
  import Image from "./pages/image/image.svelte";
  import  './tailwind.css';
  import FadeInRoute from "./components/fadeIn/fadeInRoute.svelte";
</script>

<Router>
  <Layout>
    <FadeInRoute path="/">
			Home for 4 sure
		</FadeInRoute>
    <FadeInRoute path="files">
			files
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
			Movies
		</FadeInRoute>
    <FadeInRoute path="buckets">
			buckets
		</FadeInRoute>
    <FadeInRoute path="settings">
			settings
		</FadeInRoute>
  </Layout>
</Router>


