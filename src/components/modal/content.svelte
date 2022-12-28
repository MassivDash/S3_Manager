<script>
  import Error from "../icons/error.svelte";

  export let message = "Hi";
  export let title = "Modal";
  export let type = "info";
</script>

<div class="flex gap-4">
  <div class="flex items-center justify-center h-full w-1/4 text-red-700">
    {#if type === "error"}
      <Error width={56} height={56} />
    {/if}
  </div>
  <div>
    <h2 class="text-lg">{title}</h2>
    <p class="text-sm text-red-700">
      response error: <span class="text-base text-black">{message}</span>
    </p>
  </div>
</div>
<ul class="mt-8 text-xs">
  <li class="m-2">
    - Check if your aws config files "config" and "credentials" are present on
    your current machine, for more info:
    https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html
  </li>
  <li class="m-2">
    - Aws permissions are set correctly for your S3 access read and write
    operations in AWS IAM console. (Non root user)
  </li>
</ul>
