
  import type { Bucket, Folder } from "src/types";

  export const searchWithFolders = (buckets: Bucket[], value: string): Bucket[] => {
   return buckets?.map((bucket: Bucket) => ({
        ...bucket,
        folders:
          value === ""
            ? [...bucket.folders]
            : bucket.folders.map((folder: Folder) => ({
                ...folder,
                files: folder.files.filter(
                  (item) =>
                    item.name
                      .toLocaleLowerCase()
                      .indexOf(value.toLocaleLowerCase()) !== -1
                ),
              })),
      }));
  }