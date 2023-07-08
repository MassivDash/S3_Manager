import { render, fireEvent } from '@testing-library/svelte';
import Checkbox from './checkbox.svelte';
import { describe, it, vi } from 'vitest'
import '@testing-library/jest-dom'

const handleCheckbox = vi.fn();
describe('Checkbox', () => {
  it('should render a checkbox', () => {
    const { getByRole } = render(Checkbox, { props: { handleCheckbox, key: "1234" } });
    const checkbox = getByRole('checkbox');
    expect(checkbox).toBeInTheDocument();
  });

  it('should call handleCheckbox when checkbox is clicked', async () => {

    const { getByRole } = render(Checkbox, { props: { handleCheckbox, key: "1234" } });
    const checkbox = getByRole('checkbox');
    await fireEvent.click(checkbox);
    expect(handleCheckbox).toHaveBeenCalled();
  });
});