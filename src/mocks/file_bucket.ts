import type { Bucket, Folder, File } from '../types/files/files';

// function to create array of files 

export const createFiles = (num: number): File[] => {

    const files: File[] = [];

    for (let i = 0; i < num; i++) {
        files.push({
            key: `example-key-${i}`,
            name: `example-file-${i}.jpg`,
            folder: `example-folder-${i}`,
            size: 1024,
            last_modified: 1628772000
        })
    }

    return files;

}

export const file: File = {
    key: 'example-key',
    name: 'example-file.jpg',
    folder: 'example-folder',
    size: 1024,
    last_modified: 1628772000
}

export const folder: Folder = {
    name: 'example-folder',
    files: createFiles(5),
    total_files: 1,
    total_size: 1024
}

export const BucketMock: Bucket = {
    name: 'example-bucket',
    total_files: 1,
    total_size: 1024,
    folders: [folder]
}

