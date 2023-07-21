import { search } from '../search';
import type { ImageBucket } from "../../types";
import { createImages } from "../../mocks";

describe('search', () => {
  const buckets: ImageBucket[] = [
    {
      name: 'Bucket 1',
      files: createImages(2),
      total_files: 2,
    },
    {
      name: 'Bucket 2',
      files: createImages(2),
      total_files: 2,
    },
  ];

  it('should return all buckets and files when value is empty', () => {
    const result = search(buckets, '');
    expect(result).toEqual(buckets);
  });


  it('should ignore case when searching', () => {
    const result = search(buckets, 'Example-File-0');
    expect(result).toEqual([{
      name: 'Bucket 1',
      files: createImages(1),
      total_files: 2,
    },
    {
      name: 'Bucket 2',
      files: createImages(1),
      total_files: 2,
    },]);
  });


  it('it should show all the matching files', () => {
    const result = search(buckets, 'example-file');
    expect(result).toEqual(buckets);
  });
});