// Define a function to get a file name from a full path
export function getFileName(fullPath: string): string {

    if (!fullPath) {
      return "";
    }
  
    // Split the full path by slash (/) or backslash (\) depending on OS
    let splitPath = fullPath.split("/");
    if (splitPath.length === 1) {
      // Assume Windows OS and use backslash (\)
      splitPath = fullPath.split("\\");
    }
  
    return splitPath[splitPath.length - 1];
  }