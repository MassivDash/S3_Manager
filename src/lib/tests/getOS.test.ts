import type { OS } from "../getOS";
import { getOS } from "../getOS";
import { describe, it, vi } from "vitest";

describe("getOS", () => {
  it("should return the correct OS name for macOS", () => {
    // Arrange
    const userAgent =
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36";

    vi.spyOn(window.navigator, "userAgent", "get").mockImplementation(
      () => userAgent
    );
    // Act
    const result: OS = getOS();

    // Assert
    expect(result).toEqual("Mac/iOS");
  });

  it("should return the correct OS name for Linux", () => {
    // Arrange
    const userAgent =
      "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:93.0) Gecko/20100101 Firefox/93.0";
    vi.spyOn(window.navigator, "userAgent", "get").mockImplementation(
      () => userAgent
    );
    // Act
    const result: OS = getOS();

    // Assert
    expect(result).toEqual("Linux");
  });

  it("should return the correct OS name for an unknown user agent", () => {
    // Arrange
    const userAgent = "Unknown user agent";
    vi.spyOn(window.navigator, "userAgent", "get").mockImplementation(
      () => userAgent
    );
    // Act
    const result: OS = getOS();

    // Assert
    expect(result).toEqual("Unknown");
  });

  it("should return the correct OS name for Windows 10", () => {
    // Arrange
    const userAgent =
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:93.0) Gecko/20100101 Firefox/93.0";
    vi.spyOn(window.navigator, "userAgent", "get").mockImplementation(
      () => userAgent
    );
    // Act
    const result: OS = getOS();

    // Assert
    expect(result).toEqual("Windows 10");
  });

  //And now the rest of the test cases for the other OSs
});
