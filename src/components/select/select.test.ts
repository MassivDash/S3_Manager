import { render, fireEvent } from "@testing-library/svelte";
import Select from "./select.svelte";
import '@testing-library/jest-dom'

describe("Select component", () => {
  const options = ["Option 1", "Option 2", "Option 3"];
  const label = "Select an option";
  let value = "";

  const handleChange = (event: Event): void => {
    const target = event.target as HTMLSelectElement;
    value = target.value;
  };

  it("should render the label and options", () => {
    const { getByLabelText, getByText } = render(Select, {
      options,
      value,
      label,
      handleChange,
    });

    expect(getByLabelText(label)).toBeInTheDocument();

    options.forEach((option) => {
      expect(getByText(option)).toBeInTheDocument();
    });
  });

  it("should update the value when an option is selected", async () => {
    const { getByTestId } = render(Select, {
      options,
      value,
      label,
      handleChange,
    });

    const select = getByTestId("select");
    await fireEvent.change(select, { target: { value: options[1] } });

    expect(value).toBe(options[1]);
  });
});