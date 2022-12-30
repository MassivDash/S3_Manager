<div
  class="w-[calc(100%-144px)] h-1 overflow-hidden bg-orange-50 dark:bg-slate-800 rounded-sm"
>
  <div class="progress-bar-value bg-orange-500" />
</div>

<style>
  .progress-bar-value {
    width: 100%;
    height: 100%;
    animation: indeterminateAnimation 1s infinite linear;
    transform-origin: 0% 50%;
  }

  @keyframes indeterminateAnimation {
    0% {
      transform: translateX(0) scaleX(0);
    }
    40% {
      transform: translateX(0) scaleX(0.4);
    }
    100% {
      transform: translateX(100%) scaleX(0.5);
    }
  }
</style>
