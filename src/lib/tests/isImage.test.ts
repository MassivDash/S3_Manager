import { isImage } from '../isImage';

describe('isImage', () => {
  it('should return true for image files', () => {
    expect(isImage('image.jpg')).toBe(true);
    expect(isImage('image.jpeg')).toBe(true);
    expect(isImage('image.png')).toBe(true);
    expect(isImage('image.gif')).toBe(true);
    expect(isImage('image.webp')).toBe(true);
  });

  it('should return false for non-image files', () => {
    expect(isImage('document.pdf')).toBe(false);
    expect(isImage('video.mp4')).toBe(false);
    expect(isImage('audio.mp3')).toBe(false);
    expect(isImage('text.txt')).toBe(false);
    expect(isImage('code.js')).toBe(false);
  });
});