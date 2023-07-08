import type { ImageBucket, Image } from '../types';

export const image: Image = {
    key: 'example-key',
    name: 'example-file.jpg',
    url: 'https://example.com/image.jpg',
    last_modified: 1628772000,
    folder: 'example-folder',
    size: 1024,
    tags: []
}


export const ImageBucketMock: ImageBucket = {
    name: 'example-bucket',
    total_files: 2,
    files: [image]
}
