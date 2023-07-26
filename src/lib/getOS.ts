export type OS = "Mac/iOS" | "Linux" | "UNIX" | "Windows 10" | "Windows 8.1" | "Windows 8" | "Windows 7" | "Windows Vista" | "Windows XP" | "Windows 2000" | "Unknown";


export const getOS = (): OS => {
  let OSName: | "Mac/iOS"
  | "Linux"
  | "UNIX"
  | "Windows 10"
  | "Windows 8.1"
  | "Windows 8"
  | "Windows 7"
  | "Windows Vista"
  | "Windows XP"
  | "Windows 2000"
  | "Unknown" = "Unknown";
  if (window.navigator.userAgent.indexOf("Windows NT 10.0") != -1)
    OSName = "Windows 10";
  if (window.navigator.userAgent.indexOf("Windows NT 6.3") != -1)
    OSName = "Windows 8.1";
  if (window.navigator.userAgent.indexOf("Windows NT 6.2") != -1)
    OSName = "Windows 8";
  if (window.navigator.userAgent.indexOf("Windows NT 6.1") != -1)
    OSName = "Windows 7";
  if (window.navigator.userAgent.indexOf("Windows NT 6.0") != -1)
    OSName = "Windows Vista";
  if (window.navigator.userAgent.indexOf("Windows NT 5.1") != -1)
    OSName = "Windows XP";
  if (window.navigator.userAgent.indexOf("Windows NT 5.0") != -1)
    OSName = "Windows 2000";
  if (window.navigator.userAgent.indexOf("Mac") != -1) OSName = "Mac/iOS";
  if (window.navigator.userAgent.indexOf("X11") != -1) OSName = "UNIX";
  if (window.navigator.userAgent.indexOf("Linux") != -1) OSName = "Linux";
  return OSName;
};
