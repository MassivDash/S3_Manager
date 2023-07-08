/* eslint-disable @typescript-eslint/no-unsafe-assignment */
import Tags from "./tags.svelte";
import { render, fireEvent, act } from "@testing-library/svelte";
import { describe, it, vi, afterEach } from "vitest";
import "@testing-library/jest-dom";
import * as tApi from "@tauri-apps/api";

vi.mock("@tauri-apps/api", () => {
  return {
    invoke: () => Promise.resolve([{ key: "test-tag" }]),
  };
});

afterEach(() => {
  vi.restoreAllMocks();
});

describe("Tags component", () => {
  it("should render with no tags", async () => {
    const { getByPlaceholderText } = render(Tags, {
      props: { key: "test-key", bucket: "test-bucket" },
    });

    await act(() => vi.fn());
    const input = getByPlaceholderText("Add tags");
    expect(input).toBeInTheDocument();
  });

  it("should render with tags", async () => {
    const { getByText } = render(Tags, {
      props: {
        key: "test-key",
        bucket: "test-bucket",
      },
    });

    await act(() => vi.fn());
    const tag = getByText("test-tag");
    expect(tag).toBeInTheDocument();
  });

  it("should remove a tag", async () => {
    const { getByText, queryByText, getByTestId } = render(Tags, {
      props: { key: "test-key", bucket: "test-bucket" },
    });

    await act(() => vi.fn());
    const tagBefore = getByText("test-tag");
    expect(tagBefore).toBeInTheDocument();
    const closeButton = getByTestId("remove-tag");
    await fireEvent.click(closeButton);
    await act(() => vi.fn());
    const tagAfter = queryByText("test-tag");
    expect(tagAfter).not.toBeInTheDocument();
    vi.spyOn(tApi, "invoke");

    const submitButton = getByTestId("button");
    await fireEvent.click(submitButton);
    await act(() => vi.fn());
    expect(tApi.invoke).toHaveBeenCalled();

    expect(tApi.invoke).toHaveBeenCalledWith("set_all_tags", {
      bucket: "test-bucket",
      key: "test-key",
      tagKeys: [],
    });
    expect(tApi.invoke).toHaveBeenCalledTimes(1);
  });

  it("should add a tag", async () => {
    const { getByPlaceholderText, getByText, getByTestId } = render(Tags, {
      props: { key: "test-key", bucket: "test-bucket" },
    });

    await act(() => vi.fn());

    const input = getByPlaceholderText("Add tags");
    const tag = getByText("test-tag");
    expect(tag).toBeInTheDocument();

    await fireEvent.input(input, { target: { value: "test-tag2" } });
    const form = getByTestId("tags-form");
    await fireEvent.submit(form);

    vi.spyOn(tApi, "invoke").mockResolvedValue([{ key: "test-tag" }]);

    const submitButton = getByTestId("button");
    await fireEvent.click(submitButton);
    await act(() => vi.fn());
    expect(tApi.invoke).toHaveBeenCalled();

    expect(tApi.invoke).toHaveBeenCalledWith("set_all_tags", {
      bucket: "test-bucket",
      key: "test-key",
      tagKeys: ["test-tag", "test-tag2"],
    });
    expect(tApi.invoke).toHaveBeenCalledTimes(1);
  });
});
