import { describe, it, vi } from "vitest";
import Search from "./search.svelte";
import "@testing-library/jest-dom";
import { render, fireEvent, screen } from "@testing-library/svelte";
import * as svelte from "svelte";

describe("Search", () => {
  it("renders with default props", () => {
    render(Search);
    expect(screen.getByPlaceholderText("Search...")).toBeInTheDocument();
  });

  it("renders with custom label", () => {
    render(Search, { props: { label: "Custom Label", showLabel: true } });
    expect(screen.getByLabelText("Custom Label")).toBeInTheDocument();
  });

  it("it sets up user input as the final value", async () => {
    let value = "";

    const mockFn = () => {
      return (_t: string, v: string) => {
        value = v;
        true;
      };
    };

    vi.spyOn(svelte, "createEventDispatcher").mockImplementation(
        // eslint-disable-next-line @typescript-eslint/no-unsafe-return, @typescript-eslint/no-explicit-any, @typescript-eslint/no-unsafe-argument
      mockFn as unknown as any
    );

    const ref = { current: null };
    const { getByPlaceholderText } = render(Search, {
      props: { ref, value: "wtf" },
    });
    const input = getByPlaceholderText("Search...");
    input.focus();

    await fireEvent.input(input, { target: { value: "hello ki" } });
    await fireEvent.keyDown(input, { key: "h", code: "h" });
    await fireEvent.input(input, { target: { value: "hello kity" } });
    await fireEvent.input(input, { target: { value: "hello kitt" } });
    await fireEvent.input(input, { target: { value: "hello kitty" } });

    expect(svelte.createEventDispatcher).toHaveBeenCalled();
    expect(svelte.createEventDispatcher).toHaveBeenCalledWith();

    expect(value).toBe("hello kitty");
  });
});
