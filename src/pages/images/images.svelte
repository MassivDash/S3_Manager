<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";
    import { Link } from "svelte-navigator";

      let response = "";
      onMount(async () => {
		const res = await invoke("get_all_images")
        const json = JSON.parse(res as string);
        response = json;
        console.log(json);
	});

    console.log(response)
      

</script>
  
<div> photos
    <Link to="/photos/lc-photobackup/LukePhone/067283D3-0CE6-4F03-B746-FD682560F3C0.jpeg">Photos</Link>
</div>
  

  