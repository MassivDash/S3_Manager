<script>
  import Error from "../icons/error.svelte";

  export let message = "Critical";
  export let title = "Unknown error";
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
      error details: <span class="text-base text-black">{message}</span>
    </p>
  </div>
</div>
{#if message === "failed to construct request"}
  <div class="mt-8">
    <p>
      Failed to construct request usually refers to missing or incorrect
      credentials
    </p>
    <ul class="mt-2 text-xs">
      <li class="m-2">
        - Check if your aws config files "config" and "credentials" are present
        on your current machine, for more info:
        https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html
      </li>
      <li class="m-2">
        - Aws permissions are set correctly for your S3 access read and write
        operations in AWS IAM console. (Non root user)
      </li>
    </ul>
  </div>
{/if}

{#if message === "service error"}
  <div class="mt-8">
    <p>
      Service error usually refers to incorrect credentials, either access key
      or access id is incorrect
    </p>
    <ul class="mt-2 text-xs">
      <li class="m-2">
        - Check if your aws config file "credentials" contains valid connection
        keys
      </li>
    </ul>
  </div>
{/if}

{#if message === "dispatch failure"}
  <div class="mt-8">
    <p>Unable to connect</p>
    <ul class="mt-2 text-xs">
      <li class="m-2">- Check your internet connection</li>
    </ul>
  </div>
{/if}

{#if !["service error", "failed to construct request", "dispatch failure"].includes(message)}
  <div class="mt-8">
    <p>S3 unknown error</p>
    <ul class="mt-2 text-xs">
      <li class="m-2">
        - Please report the issue on <a
          href="https://github.com/MassivDash/S3_Manager/issues"
          target="_blank">https://github.com/MassivDash/S3_Manager/issues</a
        >
      </li>
    </ul>
  </div>
{/if}
