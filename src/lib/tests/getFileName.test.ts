// BEGIN: 8f7e6d5c7b3a
import { getFileName } from "../getFileName";

describe("getFileName", () => {
  it("should return empty string if input is empty", () => {
    expect(getFileName("")).toEqual("");
  });

  it("should return file name from full path", () => {
    expect(getFileName("/path/to/file.txt")).toEqual("file.txt");
    expect(getFileName("C:\\path\\to\\file.txt")).toEqual("file.txt");
  });
});
// END: 8f7e6d5c7b3a