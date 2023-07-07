import type { Bucket, Folder, File } from '../types/files/files';

export const file: File = {
    key: 'example-key',
    name: 'example-file.jpg',
    folder: 'example-folder',
    size: 1024,
    last_modified: 1628772000
}

export const folder: Folder = {
    name: 'example-folder',
    files: [file],
    total_files: 1,
    total_size: 1024
}

export const BucketMock: Bucket = {
    name: 'example-bucket',
    total_files: 2,
    total_size: 1024,
    folders: [folder]
}
