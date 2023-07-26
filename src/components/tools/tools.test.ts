import { render, fireEvent } from "@testing-library/svelte";
import Tools from "./tools.svelte";
import { describe, it, vi } from "vitest";
import "@testing-library/jest-dom";
import type { CheckedFile } from "src/types";

describe("Tools component", () => {
  it("should render with search input and no checked files", () => {
    const { getByPlaceholderText, queryByText } = render(Tools, {
      props: {
        checkedFiles: [],
        handleDelete: undefined,
        handleDownload: undefined,
        handleSync: undefined,
        handleGrid: undefined,
        value: "",
        resync: false,
      },
    });

    const searchInput = getByPlaceholderText("Search...");
    expect(searchInput).toBeInTheDocument();

    const downloadButton = queryByText("Download");
    expect(downloadButton).not.toBeInTheDocument();

    const removeButton = queryByText("Remove");
    expect(removeButton).not.toBeInTheDocument();
  });

  it("should render with checked files and action buttons", async () => {
    const checkedFiles: CheckedFile[] = [
      { key: "file1.txt", bucket_name: "test-bucket" },
      { key: "file2.txt", bucket_name: "test-bucket" },
    ];

    const handleDelete = vi.fn();
    const handleDownload = vi.fn();

    const { getByText } = render(Tools, {
      props: {
        checkedFiles,
        handleDelete,
        handleDownload,
        handleSync: undefined,
        handleGrid: undefined,
        value: "",
        resync: false,
      },
    });

    const downloadButton = getByText("Download");
    expect(downloadButton).toBeInTheDocument();
    await fireEvent.click(downloadButton);
    expect(handleDownload).toHaveBeenCalledWith(checkedFiles);

    const removeButton = getByText("Remove");
    expect(removeButton).toBeInTheDocument();
    await fireEvent.click(removeButton);
    expect(handleDelete).toHaveBeenCalledWith(checkedFiles);
  });

  it("should render with resync loader", () => {
    const { getByTestId } = render(Tools, {
      props: {
        checkedFiles: [],
        handleDelete: undefined,
        handleDownload: undefined,
        handleSync: vi.fn(),
        handleGrid: undefined,
        value: "",
        resync: true,
      },
    });

    const loader = getByTestId("loader");
    expect(loader).toBeInTheDocument();
  });
});
