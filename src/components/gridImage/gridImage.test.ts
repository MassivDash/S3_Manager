import { render } from "src/test";
import GridImage from "./gridImage.svelte";
import { describe, it, vi } from 'vitest'
import { ImageBucketMock} from "src/mocks";

describe("GridImage component", () => {
  it("should render the component with correct props", () => {
    const key = "example-key";
    const url = "https://example.com/image.jpg";
    const size = 1024;
    const last_modified = 1628772000;
    const bucket = ImageBucketMock
    const name = "example-image.jpg";
    const checkedFiles = [];
    const handleCheckbox = vi.fn();

    const { getByText, getByAltText } = render(GridImage, {
        key,
        url,
        size,
        last_modified,
        bucket,
        name,
        checkedFiles,
        handleCheckbox,
    }, undefined, undefined);

    expect(getByText("example-image.jpg")).toBeTruthy();
    expect(getByText("1 KB")).toBeTruthy();
    expect(getByText("8/12/2021")).toBeTruthy();
    expect(getByAltText("example-key")).toBeTruthy();
  });
});