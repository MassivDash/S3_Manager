import { formatBytes } from '../date';

describe('formatBytes', () => {
  it('should format bytes correctly', () => {
    expect(formatBytes(0)).toEqual('0 Bytes');
    expect(formatBytes(1024)).toEqual('1 KB');
    expect(formatBytes(1048576)).toEqual('1 MB');
    expect(formatBytes(1099511627776)).toEqual('1 TB');
    expect(formatBytes(1125899906842624)).toEqual('1 PB');
  });

});