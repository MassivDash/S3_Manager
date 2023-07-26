import { render, fireEvent } from "@testing-library/svelte";
import FoldersPicker from "./foldersPicker.svelte";
import { describe, it, vi } from 'vitest'

describe("FoldersPicker", () => {
  it("should render folder list", () => {
    const folderList = ["Folder 1", "Folder 2", "Folder 3"];
    const handleCheckFolders = vi.fn();
    const { getAllByTestId, getByText } = render(FoldersPicker, { folderList, handleCheckFolders });

    const folders = getAllByTestId("folder");
    expect(folders.length).toBe(folderList.length);
    folderList.forEach((folder) => {
      const test = getByText(folder);
      expect(test).toBeTruthy();
    });
  });

  it("should check and uncheck folders", async () => {
    const folderList = ["Folder 1", "Folder 2", "Folder 3"];
    const handleCheckFolders = vi.fn();
    const { getAllByTestId } = render(FoldersPicker, { folderList, handleCheckFolders });

    const folders = getAllByTestId("folder");
    const folderToCheck = folders[1];
    await fireEvent.click(folderToCheck);

    expect(handleCheckFolders).toHaveBeenCalledTimes(1);
    expect(handleCheckFolders).toHaveBeenCalledWith(folderList[1]);

    const folderToUncheck = folders[0];
    await fireEvent.click(folderToUncheck);

    expect(handleCheckFolders).toHaveBeenCalledTimes(2);
    expect(handleCheckFolders).toHaveBeenCalledWith(folderList[0]);
  });
});