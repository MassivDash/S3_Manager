<script lang="ts">
  import { Router, Route } from "svelte-navigator";
  import Layout from "./components/template/template.svelte";
  import Images from "./pages/images/images.svelte";
  import Image from "./pages/image/image.svelte";
  import  './tailwind.css';
</script>

<Router>
  <Layout>
    <Route path="/">
			Home for 4 sure
		</Route>
    <Route path="files">
			files
		</Route>
    <Route path="images/*">
			<Route path="/">
       <Images />
			</Route>
			<Route path=":bucket/*key">
        <Image />
      </Route>
		</Route>
    <Route path="movies">
			Movies
		</Route>
    <Route path="buckets">
			buckets
		</Route>
    <Route path="settings">
			settings
		</Route>
  </Layout>
</Router>


