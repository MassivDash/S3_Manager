import { render } from '@testing-library/svelte';
import CircularProgress from './circularProgress.svelte';
import { describe, it } from 'vitest'
import '@testing-library/jest-dom'


describe('CircularProgress', () => {
  it('should render with 0% progress', () => {
    const { getByText } = render(CircularProgress, { progress: 0 });
    expect(getByText('0%')).toBeInTheDocument();
  });

  it('should render with 50% progress', () => {
    const { getByText } = render(CircularProgress, { progress: 50 });
    expect(getByText('50%')).toBeInTheDocument();
  });

  it('should render with 100% progress', () => {
    const { getByText } = render(CircularProgress, { progress: 100 });
    expect(getByText('100%')).toBeInTheDocument();
  });
});