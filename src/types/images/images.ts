export interface Image {
    key: string;
    name: string;
    url: string;
    size: number;
    last_modified: number;
  }

export interface ImageBucket {
    name: string;
    files: Image[];
    total_files: number;
}