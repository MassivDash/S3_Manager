/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-explicit-any */
import { render } from "../../test";
import { vi, describe, it } from "vitest";
import { BucketMock } from "src/mocks";
import Buckets from "./buckets.svelte";
import { act } from "@testing-library/svelte";
import "@testing-library/jest-dom";
import { buckets } from "../../store/buckets";
import { files } from "../../store/files";
import { modal } from "../../store/modal";
import * as tauri from "@tauri-apps/api";
import { tick } from "svelte";

vi.mock("@tauri-apps/api", () => {
  const listeners: { [key: string]: any } = {};
  return {
    invoke: () => Promise.resolve([BucketMock].map((b) => ({ ...b, creation_date: new Date() }) )),
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

describe("Buckets page", () => {

  buckets.set([BucketMock]);
  files.set([BucketMock]);
  it("should render the page", async () => {



    const modalSpy = vi.spyOn(modal, 'set')

    const { getByText, getByTestId } = render(Buckets, {}, { withRoute: true }, undefined);
    expect(getByTestId("loader")).toBeInTheDocument();
    await act(() => { vi.fn()})
    expect(getByText("example-bucket")).toBeInTheDocument();
    expect(getByText("E")).toBeInTheDocument();
    expect(getByText("total size:")).toBeInTheDocument();
    expect(getByText("1 KB")).toBeInTheDocument();
    expect(getByText("total files:")).toBeInTheDocument();
    expect(modalSpy).not.toHaveBeenCalled()
    buckets.set(null)
    files.set(null)
  });

  it('should show modal on error', async () => {
    const modalSpy = vi.spyOn(modal, 'set')
    const tauriSpy = vi.spyOn(tauri, 'invoke').mockImplementation(() => Promise.reject(new Error('test')))
    render(Buckets, {}, { withRoute: true }, undefined);
    expect(tauriSpy).toHaveBeenCalled()
    await tick();
    await tick();
    await act(() => { vi.fn()})
    expect(modalSpy).toHaveBeenCalled()
  })
});
