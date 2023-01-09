<div id="scroller" class="overflow-y-scroll h-[90vh] pr-4"><slot /></div>

<style>
  @media (prefers-color-scheme: dark) {
    #scroller::-webkit-scrollbar {
      background-color: #10172a;
      width: 10px;
    }
    #scroller::-webkit-scrollbar-thumb {
      background: #fff;
      border-radius: 10px;
      height: 15px;
      width: 5px;
    }
  }
</style>
