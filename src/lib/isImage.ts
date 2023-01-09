export function isImage(path: string): boolean {
    const imageExtensions = ['jpg', 'jpeg', 'png', 'gif', 'webp'];
    const fileExtension = path.split('.').pop()!.toLowerCase();
    return imageExtensions.includes(fileExtension);
  }