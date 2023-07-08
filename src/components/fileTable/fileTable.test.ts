import { fireEvent } from "@testing-library/svelte";
import { render } from "src/test";
import FileTable from "./fileTable.svelte";
import { describe, it, vi } from 'vitest'
import '@testing-library/jest-dom'
import { BucketMock } from "src/mocks";

const handleCheckbox = vi.fn();
const handleFolderDelete = vi.fn();
const handleFilesSelect = vi.fn();
const checkedFiles = [];
const folder = BucketMock.folders[0];
const bucket = BucketMock;

// Define a mock implementation of the IntersectionObserver class



describe("FileTable component", () => {
  it("should render the table", async () => {
    const { getByText, getByTestId } = render(FileTable, {
      folder,
      bucket,
      handleCheckbox,
      checkedFiles,
      handleFolderDelete,
      handleFilesSelect,
    }, undefined, undefined);

    expect(getByText(`${folder.name} (5)`)).toBeInTheDocument();

    const foldButton = getByTestId("unfold")
    await fireEvent.click(foldButton);
    const date = getByText("Date Modified");
    expect(date).toBeInTheDocument();
    await fireEvent.scroll(date);
    // expect(getByText(folder.files[0].name)).toBeInTheDocument();
  });
});
