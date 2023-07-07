import { chunkify } from "../grid";

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