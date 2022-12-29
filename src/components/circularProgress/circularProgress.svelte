<script lang="ts">
  export let progress = 0;
  $: angle = 360 * progress;
  $: procentage = (progress * 100).toFixed(0) + "%";
  // Adapt the logic according to the approach
  $: background = `radial-gradient(white 50%, transparent 51%),
    conic-gradient(transparent 0deg ${angle}deg, gainsboro ${angle}deg 360deg),
    conic-gradient(orange 0deg, orange 90deg, orange 180deg, orange);`;

  $: cssVarStyles = `--background:${background}`;
</script>

<div id="progress-circle" style={cssVarStyles}>
  <div
    class="flex items-center justify-center rounded-full w-20 h-20 text-lg dark:bg-slate-800 dark:text-white  gap-2 border-0  outline-orange-500 bg-none transition-all hover:bg-gray-50 hover:dark:bg-slate-700 hover:dark:text-orange-50 hover:text-gray-800 active:bg-gray-200"
  >
    {procentage}
  </div>
</div>

<style>
  #progress-circle {
    background: var(--background);
    border-radius: 50%;
    width: 120px;
    height: 120px;
    transition: all 500ms ease-in;
    will-change: transform;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
