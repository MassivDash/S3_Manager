<script lang="ts">
  /**
   * Specify the input value
   */
  export let value = "";

  /**
   * Set to `true` to auto focus the input on mount
   */
  export let autofocus = false;

  /**
   * Specify the debounce value in milliseconds (ms)
   */
  export let debounce = 0;

  /**
   * Specify the input label text
   */
  export let label = "Label";

  /**
   * Set to `true` to visually hide the label
   */
  export let showLabel = false;

  /**
   * Specify an `id` for the `input`
   */
  export let id = "search" + Math.random().toString(36);

  /**
   * Obtain a reference to the `input` element
   * @type {HTMLInputElement}
   */
  export let ref: HTMLInputElement | null | undefined = null;

  /**
   * Set to `true` to omit the form `role="search"` attribute
   */
  export let removeFormAriaAttributes = false;

  import { createEventDispatcher, onMount, afterUpdate } from "svelte";

  const dispatch = createEventDispatcher();

  let prevValue = value;
  let timeout: ReturnType<typeof setTimeout> | undefined = undefined;
  let calling = false;

  function debounceFn(fn: () => void): void {
    if (calling) return;
    calling = true;
    timeout = setTimeout(() => {
      fn();
      calling = false;
    }, debounce);
  }

  onMount(() => {
    if (autofocus) window.requestAnimationFrame(() => ref?.focus());
    return (): void => clearTimeout(timeout);
  });

  afterUpdate(() => {
    if (value.length > 0 && value !== prevValue) {
      if (debounce > 0) {
        debounceFn(() => dispatch("type", value));
      } else {
        dispatch("type", value);
      }
    }

    if (value.length === 0 && prevValue.length > 0) dispatch("clear");

    prevValue = value;
  });
</script>

<form
  data-svelte-search
  role={removeFormAriaAttributes ? "" : "search"}
  aria-labelledby={removeFormAriaAttributes ? null : id}
  on:submit|preventDefault
>
  {#if showLabel}
    <label
      id="{id}-label"
      for={id}
      style={showLabel &&
        "position: absolute;height: 1px;width: 1px;overflow: hidden;clip: rect(1px 1px 1px 1px);clip: rect(1px, 1px, 1px, 1px);white-space: nowrap;"}
    >
      <slot name="label">{label}</slot>
    </label>
  {/if}
  <input
    bind:this={ref}
    name="search"
    type="search"
    placeholder="Search..."
    autocomplete="off"
    spellcheck="false"
    {...$$restProps}
    {id}
    bind:value
    on:input
    on:change
    on:focus
    on:blur
    on:keydown
  />
</form>
