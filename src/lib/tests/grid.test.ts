import { chunkify, handleGrid, getTailwindClass } from "../grid";

describe("chunkify", () => {
  it("should split an array into chunks of equal size when possible", () => {
    const input = [1, 2, 3, 4, 5, 6];
    const expectedOutput = [[1, 2], [3, 4], [5, 6]];
    const actualOutput = chunkify(input, 3, true);
    expect(actualOutput).toEqual(expectedOutput);
  });

  it("should split an array into chunks of unequal size when necessary", () => {
    const input = [1, 2, 3, 4, 5, 6];
    const expectedOutput = [[1, 2], [3, 4], [5, 6]];
    const actualOutput = chunkify(input, 3, false);
    expect(actualOutput).toEqual(expectedOutput);
  });

  it("should return the original array when n is less than 2", () => {
    const input = [1, 2, 3, 4, 5, 6];
    const expectedOutput = [input];
    const actualOutput = chunkify(input, 1, true);
    expect(actualOutput).toEqual(expectedOutput);
  });
});

describe("handleGrid", () => {
  it("should return the next grid column", () => {
    expect(handleGrid(1)).toEqual(2);
    expect(handleGrid(2)).toEqual(3);
    expect(handleGrid(3)).toEqual(4);
    expect(handleGrid(4)).toEqual(1);
  });
});

describe("getTailwindClass", () => {
  it("should return the tailwind class for the given grid column", () => {
    expect(getTailwindClass(1)).toEqual("grid-cols-1");
    expect(getTailwindClass(2)).toEqual("grid-cols-2");
    expect(getTailwindClass(3)).toEqual("grid-cols-3");
    expect(getTailwindClass(4)).toEqual("grid-cols-4");
  });
});