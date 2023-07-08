import { searchWithFolders } from "../foldersSearch";
import type { Bucket } from "../../types";
import { createFiles } from "../../mocks/file_bucket";
describe("searchWithFolders", () => {
  const buckets: Bucket[] = [
    {
      name: "Bucket 1",
      total_files: 4,
      total_size: 1024 * 4,
      folders: [
        {
          name: "Folder 1",
          files: createFiles(2),
          total_files: 2,
          total_size: 1024 * 2,
        },
        {
          name: "Folder 2",
          files: createFiles(2),
          total_files: 2,
          total_size: 1024 * 2,
        },
      ],
    },
    {
      name: "Bucket 2",
      total_files: 4,
      total_size: 1024 * 4,
      folders: [
        {
          name: "Folder 3",
          files: createFiles(3),
          total_files: 2,
          total_size: 1024 * 2,
        },
      ],
    },
  ];

  it("should return all buckets and folders when value is empty", () => {
    const result = searchWithFolders(buckets, "");
    expect(result).toEqual(buckets);
  });

  it("should return buckets and folders with matching files when value is not empty", () => {
    const result = searchWithFolders(buckets, "example-file-2");
    expect(result).toEqual([
      {
        name: "Bucket 1",
        total_files: 4,
        total_size: 4096,
        folders: [
          {
            files: [],
            name: "Folder 1",
            total_files: 2,
            total_size: 2048,
          },
          {
            files: [],
            name: "Folder 2",
            total_files: 2,
            total_size: 2048,
          },
        ],
      },
      {
        name: "Bucket 2",
        total_files: 4,
        total_size: 4096,
        folders: [
          {
            files: [
              {
                folder: "example-folder-2",
                key: "example-key-2",
                last_modified: 1628772000,
                name: "example-file-2.jpg",
                size: 1024,
              },
            ],
            name: "Folder 3",
            total_files: 2,
            total_size: 2048,
          },
         ],
      },
    ]);
  });
});
