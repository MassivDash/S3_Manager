import type { ImageBucket, Image } from "../types";

export const createImages = (num: number): Image[] => {
  const files: Image[] = [];

  for (let i = 0; i < num; i++) {
    files.push({
      key: `example-key-${i}`,
      name: `example-file-${i}.jpg`,
      url: `https://example.com/image-${i}.jpg`,
      last_modified: 1628772000,
      folder: "example-folder",
      size: 1024,
      tags: [],
    });
  }

  return files;
};

export const image: Image = {
  key: "example-key",
  name: "example-file.jpg",
  url: "https://example.com/image.jpg",
  last_modified: 1628772000,
  folder: "example-folder",
  size: 1024,
  tags: [],
};

export const ImageBucketMock: ImageBucket = {
  name: "example-bucket",
  total_files: 2,
  files: [image],
};
