import type { GridCol } from "src/types";

export function chunkify<T>(a: T[], n: number, balanced: boolean): T[][] {
    if (n < 2) return [a];

    const len = a.length;
    const out: T[][] = [];
    let i = 0;
    let size = 0;

    if (len % n === 0) {
      size = Math.floor(len / n);
      while (i < len) {
        out.push(a.slice(i, (i += size)));
      }
    } else if (balanced) {
      while (i < len) {
        size = Math.ceil((len - i) / n--);
        out.push(a.slice(i, (i += size)));
      }
    } else {
      n--;
      size = Math.floor(len / n);
      if (len % size === 0) size--;
      while (i < size * n) {
        out.push(a.slice(i, (i += size)));
      }
      out.push(a.slice(size * n));
    }

    return out;
  }

  export const handleGrid = (gridCol: GridCol): GridCol => {
    switch (gridCol) {
      case 1: 
        return 2;
      case 2:
        return 3;
      case 3:
        return 4;
      case 4:
        return 1;
    }
  };

  export const getTailwindClass = (col: GridCol): string => {
    switch (col) {
      case 1:
        return "grid-cols-1";
      case 2:
        return "grid-cols-2";
      case 3:
        return "grid-cols-3";
      case 4:
        return "grid-cols-4";
    }
  };
