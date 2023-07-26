import { render, fireEvent } from "@testing-library/svelte";
import AddFolder from "./addFolder.svelte";
import { describe, it, vi } from 'vitest'
import '@testing-library/jest-dom'

vi.mock("@tauri-apps/api", () => {
  return {
    invoke: () => Promise.resolve({ data: "test"}),
  };
});

const handleSync = vi.fn();


describe("AddFolder component", () => {
  it("should render the add folder button", () => {
    const { getByTestId } = render(AddFolder, {
      props: { handleSync },
    });
    const addForm = getByTestId("add-folder-form")
    expect(addForm).toBeInTheDocument();
  });

  it("should show the input field when the add folder button is clicked", async () => {
    const { getByTestId, getByPlaceholderText } = render(AddFolder, {
      props: { handleSync },
    });
    const addForm = getByTestId("add-folder-form");
    await fireEvent.click(addForm);
    const inputField = getByPlaceholderText("Enter folder name");
    expect(inputField).toBeInTheDocument();
  });

  it("should submit the folder name when the submit button is clicked", async () => {

    const { getByTestId, getByText, getByPlaceholderText } = render(AddFolder, {
      props: { handleSync },
    });
    const addForm = getByTestId("add-folder-form");
    await fireEvent.click(addForm);
    const close = getByTestId("add-folder-close");
    expect(close).toBeInTheDocument();
    const inputField = getByPlaceholderText("Enter folder name");
    const submitButton = getByText("Submit");
    await fireEvent.input(inputField, { target: { value: "test-folder" } });
    await fireEvent.click(submitButton);
    expect(handleSync).toHaveBeenCalled();
    const loader = getByTestId('loader');
    expect(loader).toBeInTheDocument();


  });
});