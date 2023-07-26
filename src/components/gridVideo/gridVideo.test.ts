/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import { render, fireEvent } from "@testing-library/svelte";
import GridVideo from "./gridVideo.svelte";
import { describe, it, vi } from 'vitest'
import '@testing-library/jest-dom'


const handleCheckbox = vi.fn();

describe("GridVideo component", () => {
  it("should render the video with the correct source URL", () => {
    const key = "abc123";
    const checkedFiles = [{ key: "def456", bucket_name: "my-bucket" }];
    const url = "https://example.com/video.mp4";
    const name = "This is a very long video name that should be shortened";
    const size = 1024 * 1024 * 10; // 10 MB
    const { getByTestId } = render(GridVideo, {
        props: { key, checkedFiles, url, name, size, bucket: { name: "my-bucket", files:[], total_files: 1 }, handleCheckbox },
    });
    const videoElement = getByTestId("video");
    expect(videoElement.getAttribute("src")).toBe(`${url}#t=0.1`);
  });

it("should render the checkbox with the correct key and checked state", async () => {
    const key = "def456";
    const checkedFiles = [{ key: "def456", bucket_name: "my-bucket" }];
    const url = "https://example.com/video.mp4";
    const name = "This is a very long video name that should be shortened";
    const size = 1024 * 1024 * 10; // 10 MB
    const { getByTestId } = render(GridVideo, {
        props: { key, checkedFiles, url, name, size, bucket: { name: "my-bucket", files:[], total_files: 1 }, handleCheckbox },
    });
    const checkboxElement = getByTestId("checkbox");
    await fireEvent.click(checkboxElement);
    expect(handleCheckbox).toHaveBeenCalled();
    expect(handleCheckbox).toBeCalledWith(key, "my-bucket")
    const videoElement = getByTestId("video");
    expect(videoElement.getAttribute("src")).toBe(`${url}#t=0.1`);
});

});