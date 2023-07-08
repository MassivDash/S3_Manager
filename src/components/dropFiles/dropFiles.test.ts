/* eslint-disable @typescript-eslint/require-await */
/* eslint-disable @typescript-eslint/restrict-template-expressions */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/ban-ts-comment */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import { render, fireEvent, screen } from "@testing-library/svelte";
import DropFiles from "./dropFiles.svelte";
import { describe, it, vi } from "vitest";
import { BucketMock } from "src/mocks";
import "@testing-library/jest-dom";



const file = new File(["test"], "test.txt", { type: "text/plain" });

vi.mock("@tauri-apps/api", () => {
  const listeners: { [key: string]: any } = {};
  return {
    invoke: () => Promise.resolve([BucketMock]),
    event: {
      listen: (event: string, callback: any) => {
        callback({ payload: ["test.txt"] });

        return () => vi.fn();
      },
      trigger: (event: string, ...args: any[]) => {
        if (listeners[event]) {
          listeners[event](...args);
        }
      },
    },
  };
});



vi.mock("@tauri-apps/api/fs", () => {
  return {
    readDir: () => Promise.resolve([file]),
  };
});

describe("DropFiles component", () => {
  it("should handle file drop", async () => {
    const { getByTestId } = render(DropFiles);

    const dropzone = getByTestId("dropzone");
    await fireEvent.drop(dropzone, {
      dataTransfer: { files: [file] },
    });

    expect(screen.getByText("Uploading files")).toBeInTheDocument();
    expect(screen.getByText("1 file")).toBeInTheDocument();
  });

  it("should handle file upload", async () => {
    const { getByTestId  } = render(DropFiles);
    const dropzone = getByTestId("dropzone");

    await fireEvent.drop(dropzone, {
      dataTransfer: { files: [file] },
    });

    expect(screen.getByText("Uploading files")).toBeInTheDocument();
    expect(screen.getByText("1 file")).toBeInTheDocument();

    const uploadButton = getByTestId("upload-submit");
    await fireEvent.click(uploadButton);


    expect(screen.getByText("starting up ...")).toBeInTheDocument();
    
  });
});
