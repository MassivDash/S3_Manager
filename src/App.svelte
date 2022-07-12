<script lang="ts">
  import { Router, Route, Link } from "svelte-navigator";
  import Counter from "./lib/Counter.svelte";
  import Layout from "./components/layout/layout.svelte";
  import Images from "./pages/images/images.svelte";
  import Image from "./pages/image/image.svelte";
</script>

<Router>
  <Layout>
    <Route path="/">
			<Counter />
		</Route>
    <Route path="photos/*">
			<Route path="/">
       <Images />
			</Route>
			<Route path=":bucket/*key">
        <Image />
      </Route>
		</Route>
    <Route path="buckets">
			buckets
		</Route>
  </Layout>
</Router>


  <style>
    :root {
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
        Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
    }
  </style>
