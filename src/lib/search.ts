import type { ImageBucket } from './../types/images/images';

export const search = (buckets: ImageBucket[], value: string): ImageBucket[] => {
    return buckets?.map((bucket) => ({
        ...bucket,
        files:
          value === ""
            ? bucket.files
            : bucket.files.filter((item) => item.name.toLocaleLowerCase().indexOf(value.toLocaleLowerCase()) !== -1),
      }));
} 