import { render, fireEvent } from '@testing-library/svelte';
import IconButton from './iconButton.svelte';
import { describe, it, vi } from 'vitest'
import '@testing-library/jest-dom'

describe('IconButton', () => {
  it('should render with default props', () => {
    const { getByRole } = render(IconButton);

    const button = getByRole('button');

    expect(button).toBeInTheDocument();
    expect(button).toHaveClass('bg-orange-50');
    expect(button).toHaveClass('rounded');
    expect(button).toHaveClass('p-4');
    expect(button).toHaveClass('gap-2');
    expect(button).toHaveClass('flex');
    expect(button).toHaveClass('border-0');
    expect(button).toHaveClass('appearance-none');
    expect(button).toHaveClass('outline-orange-500');
    expect(button).toHaveClass('bg-none');
    expect(button).toHaveClass('transition-all');
    expect(button).toHaveClass('dark:bg-slate-800');
    expect(button).toHaveClass('dark:text-white');
    expect(button).toHaveClass('hover:bg-gray-50');
    expect(button).toHaveClass('hover:dark:bg-slate-700');
    expect(button).toHaveClass('hover:dark:text-orange-50');
    expect(button).toHaveClass('hover:text-gray-800');
    expect(button).toHaveClass('active:bg-gray-200');
  });

  it('should render with custom props', async () => {
    const onClick = vi.fn();
    const color = 'bg-blue-500';
    const type = 'submit';

    const { getByRole } = render(IconButton, {
      props: {
        onClick,
        color,
        type,
      },
    });

    const button = getByRole('button');

    expect(button).toBeTruthy();
    expect(button).toHaveClass(color);
    expect(button).toHaveAttribute('type', type);

    await fireEvent.click(button);

    expect(onClick).toHaveBeenCalledTimes(1);
  });
});