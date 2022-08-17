export interface File {
    key: string;
    name: string;
    folder: string;
    size: number;
    last_modified: number;
  }

export interface Folder {
    name: string;
    files: File[];
    total_files: number;
    total_size: number;
  }

export interface Bucket {
    name: string;
    folders: Folder[];
    total_files: number;
  }


export interface CheckedFile {
  key: string;
  bucket_name: string;
}