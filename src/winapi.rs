//! 手动声明本程序需要的 Win32 API，避免任何第三方 crate 依赖。
#![allow(non_camel_case_types, dead_code)]

use std::ffi::c_void;

pub type HANDLE = *mut c_void;
pub type HWND = *mut c_void;
pub type HMENU = *mut c_void;
pub type HINSTANCE = *mut c_void;
pub type HICON = *mut c_void;
pub type HCURSOR = *mut c_void;
pub type HBRUSH = *mut c_void;
pub type HFONT = *mut c_void;
pub type HKEY = *mut c_void;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;

pub type WndProc = unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT;

pub const INVALID_HANDLE_VALUE: HANDLE = usize::MAX as *mut c_void;
pub const HKEY_CURRENT_USER: HKEY = 0x8000_0001usize as *mut c_void;

pub const ERROR_SUCCESS: u32 = 0;
pub const ERROR_ALREADY_EXISTS: u32 = 183;
pub const ERROR_ACCESS_DENIED: u32 = 5;
pub const ERROR_INVALID_PARAMETER: u32 = 87;

pub const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
pub const PROCESS_QUERY_INFORMATION: u32 = 0x0400;
pub const PROCESS_SET_INFORMATION: u32 = 0x0200;
pub const TH32CS_SNAPPROCESS: u32 = 0x0000_0002;

pub const IDLE_PRIORITY_CLASS: u32 = 0x0000_0040;
pub const BELOW_NORMAL_PRIORITY_CLASS: u32 = 0x0000_4000;
pub const NORMAL_PRIORITY_CLASS: u32 = 0x0000_0020;
pub const ABOVE_NORMAL_PRIORITY_CLASS: u32 = 0x0000_8000;
pub const HIGH_PRIORITY_CLASS: u32 = 0x0000_0080;

pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: u32 = 0x1;
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: u32 = 0x4;
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: u32 = 1;
pub const PROCESS_POWER_THROTTLING_CLASS: u32 = 4; // PROCESS_INFORMATION_CLASS

// 窗口消息
pub const WM_CREATE: u32 = 0x0001;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_SIZE: u32 = 0x0005;
pub const WM_SETFONT: u32 = 0x0030;
pub const WM_COMMAND: u32 = 0x0111;
pub const WM_TIMER: u32 = 0x0113;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_NOTIFY: u32 = 0x004E;
pub const WM_GETTEXT: u32 = 0x000D;
pub const WM_SETTEXT: u32 = 0x000C;
pub const WM_APP_TRAY: u32 = 0x8001;

// 窗口样式
pub const WS_OVERLAPPEDWINDOW: u32 = 0x00CF_0000;
pub const WS_VISIBLE: u32 = 0x1000_0000;
pub const WS_CHILD: u32 = 0x4000_0000;
pub const WS_TABSTOP: u32 = 0x0001_0000;
pub const WS_BORDER: u32 = 0x0080_0000;
pub const WS_VSCROLL: u32 = 0x0020_0000;
pub const WS_CLIPCHILDREN: u32 = 0x0200_0000;
pub const WS_EX_CONTROLPARENT: u32 = 0x0001_0000;
pub const WS_EX_CLIENTEDGE: u32 = 0x0000_0200;

pub const SW_HIDE: i32 = 0;
pub const SW_SHOWNORMAL: i32 = 1;
pub const SW_SHOW: i32 = 5;

pub const GWLP_USERDATA: i32 = -21;

// 控件样式
pub const ES_AUTOHSCROLL: u32 = 0x0000_0080;
pub const ES_AUTOVSCROLL: u32 = 0x0000_0040;
pub const ES_MULTILINE: u32 = 0x0000_0004;
pub const ES_READONLY: u32 = 0x0000_0800;
pub const BS_DEFPUSHBUTTON: u32 = 0x0000_0001;
pub const LVS_REPORT: u32 = 0x0000_0001;
pub const LVS_SHOWSELALWAYS: u32 = 0x0000_0008;
pub const LVS_SINGLESEL: u32 = 0x0000_0004;
pub const LVM_FIRST: u32 = 0x1000;
pub const LVM_GETITEMCOUNT: u32 = LVM_FIRST + 4;
pub const LVM_DELETEALLITEMS: u32 = LVM_FIRST + 9;
pub const LVM_DELETEITEM: u32 = LVM_FIRST + 8;
pub const LVM_GETNEXTITEM: u32 = LVM_FIRST + 12;
pub const LVM_INSERTITEMW: u32 = LVM_FIRST + 77;
pub const LVM_SETITEMW: u32 = LVM_FIRST + 76;
pub const LVM_GETITEMW: u32 = LVM_FIRST + 75;
pub const LVM_INSERTCOLUMNW: u32 = LVM_FIRST + 97;
pub const LVM_GETITEMTEXTW: u32 = LVM_FIRST + 115;
pub const LVM_SETEXTENDEDLISTVIEWSTYLE: u32 = LVM_FIRST + 54;
pub const LVS_EX_FULLROWSELECT: u32 = 0x0000_0020;
pub const LVS_EX_GRIDLINES: u32 = 0x0000_0001;
pub const LVS_EX_DOUBLEBUFFER: u32 = 0x0001_0000;
pub const LVS_EX_CHECKBOXES: u32 = 0x0000_0004;
pub const LVNI_SELECTED: u32 = 0x0002;
pub const LVIS_STATEIMAGEMASK: u32 = 0x0000_F000;
pub const LVM_GETITEMSTATE: u32 = LVM_FIRST + 44;
pub const LVM_SETIMAGELIST: u32 = LVM_FIRST + 3;
pub const LVSIL_SMALL: usize = 1;
// SHGetFileInfo flags
pub const SHGFI_ICON: u32 = 0x0000_0100;
pub const SHGFI_SMALLICON: u32 = 0x0000_0001;
pub const SHGFI_USEFILEATTRIBUTES: u32 = 0x0000_0010;
pub const FILE_ATTRIBUTE_NORMAL: u32 = 0x0000_0080;
// ImageList_Create flags
pub const ILC_COLOR32: u32 = 0x0000_0020;
pub const ILC_MASK: u32 = 0x0000_0001;
pub const LVM_ENSUREVISIBLE: u32 = LVM_FIRST + 19;
pub const LVM_SORTITEMS: u32 = LVM_FIRST + 48;
pub const LVM_GETITEMRECT: u32 = LVM_FIRST + 14;
pub const LVN_FIRST: u32 = 0xFFFF_FF9C;
pub const LVN_COLUMNCLICK: u32 = LVN_FIRST.wrapping_sub(8); // 0xFFFFFF94
pub const LVIS_FOCUSED: u32 = 0x0000_0001;

// TreeView
pub const TV_FIRST: u32 = 0x1100;
pub const TVS_HASLINES: u32 = 0x0000_0002;
pub const TVS_LINESATROOT: u32 = 0x0000_0004;
pub const TVS_HASBUTTONS: u32 = 0x0000_0001;
pub const TVS_SHOWSELALWAYS: u32 = 0x0000_0020;
pub const TVM_INSERTITEMW: u32 = TV_FIRST + 50;
pub const TVM_DELETEITEM: u32 = TV_FIRST + 1;
pub const TVI_ROOT: isize = -0x10000;
pub const TVM_GETNEXTITEM: u32 = TV_FIRST + 10;
pub const TVM_GETITEMW: u32 = TV_FIRST + 62;
pub const TVM_EXPAND: u32 = TV_FIRST + 2;
pub const TVM_HITTEST: u32 = TV_FIRST + 17;
pub const TVHT_ONITEMLABEL: u32 = 0x0000_0004;
pub const TVHT_ONITEMBUTTON: u32 = 0x0000_0010;
pub const TVHT_ONITEM: u32 = 0x0000_005E;
pub const TVGN_CARET: u32 = 9;
pub const TVGN_CHILD: u32 = 4;
pub const TVGN_NEXT: u32 = 1;
pub const TVGN_PARENT: u32 = 3;
pub const TVGN_ROOT: u32 = 0;
pub const TVE_COLLAPSE: u32 = 0x0001;
pub const TVE_EXPAND: u32 = 0x0002;
pub const TVIF_TEXT: u32 = 0x0000_0001;
pub const TVIF_STATE: u32 = 0x0000_0008;
pub const TVIF_PARAM: u32 = 0x0000_0004;
pub const TVIS_EXPANDED: u32 = 0x0000_0020;
pub const NM_CLICK: u32 = 0xFFFF_FFFE;

// 按钮 / 图标 / 文件
pub const BS_AUTOCHECKBOX: u32 = 0x0000_0003;
pub const BM_GETCHECK: u32 = 0x00F0;
pub const BM_SETCHECK: u32 = 0x00F1;
pub const BST_CHECKED: usize = 1;
pub const BST_UNCHECKED: usize = 0;
pub const IMAGE_ICON: u32 = 1;
pub const LR_LOADFROMFILE: u32 = 0x0000_0010;
pub const LR_DEFAULTSIZE: u32 = 0x0000_0040;
pub const AF_INET: u32 = 2;
pub const TCP_TABLE_OWNER_PID_ALL: u32 = 5;
pub const UDP_TABLE_OWNER_PID: u32 = 1;

// 设置对话框控件 ID
pub const ID_CHK_AUTOSTART: u32 = 3101;
pub const ID_CHK_START_TRAY: u32 = 3102;
pub const ID_CHK_TRAY_ICON: u32 = 3103;
pub const ID_CHK_AUTO_APPLY: u32 = 3104;
pub const ID_RULES_CHECK: u32 = 3105;
pub const ID_BTN_SETTINGS_OK: u32 = 3106;
pub const ID_BTN_SETTINGS_CANCEL: u32 = 3107;
pub const ID_CHK_EXIT_AFTER_APPLY: u32 = 3108;
pub const ID_CHK_AUTO_REFRESH: u32 = 3109;

pub const LVIF_TEXT: u32 = 0x0000_0001;
pub const LVIF_IMAGE: u32 = 0x0000_0002;
pub const LVIF_PARAM: u32 = 0x0000_0004;
pub const LVIF_STATE: u32 = 0x0000_0008;
pub const LVCF_FMT: u32 = 0x0000_0001;
pub const LVCF_WIDTH: u32 = 0x0000_0002;
pub const LVCF_TEXT: u32 = 0x0000_0004;
pub const LVCF_SUBITEM: u32 = 0x0000_0008;
pub const LVCFMT_LEFT: i32 = 0;

pub const NM_DBLCLK: u32 = 0xFFFF_FFFD;
pub const NM_RCLICK: u32 = 0xFFFF_FFFB;
pub const EN_CHANGE: u32 = 0x0300;

pub const MF_BYCOMMAND: u32 = 0x0000_0000;
pub const MF_CHECKED: u32 = 0x0000_0008;
pub const MF_UNCHECKED: u32 = 0x0000_0000;
pub const MF_GRAYED: u32 = 0x0000_0001;
pub const MF_ENABLED: u32 = 0x0000_0000;
pub const TPM_LEFTALIGN: u32 = 0x0000_0000;
pub const TPM_RETURNCMD: u32 = 0x0000_0100;
pub const TPM_RIGHTBUTTON: u32 = 0x0000_0002;

pub const MB_OK: u32 = 0x0000_0000;
pub const MB_YESNO: u32 = 0x0000_0004;
pub const MB_ICONERROR: u32 = 0x0000_0010;
pub const MB_ICONQUESTION: u32 = 0x0000_0020;
pub const MB_ICONINFORMATION: u32 = 0x0000_0040;
pub const IDYES: usize = 6;
pub const IDNO: usize = 7;

pub const SM_CXSCREEN: i32 = 0;
pub const SM_CYSCREEN: i32 = 1;
pub const DEFAULT_GUI_FONT: i32 = 17;

pub const NIM_ADD: u32 = 0;
pub const NIM_DELETE: u32 = 2;
pub const NIF_MESSAGE: u32 = 0x0000_0001;
pub const NIF_ICON: u32 = 0x0000_0002;
pub const NIF_TIP: u32 = 0x0000_0004;

pub const OFN_OVERWRITEPROMPT: u32 = 0x0000_0002;
pub const OFN_HIDEREADONLY: u32 = 0x0000_0004;
pub const OFN_PATHMUSTEXIST: u32 = 0x0000_0800;
pub const OFN_FILEMUSTEXIST: u32 = 0x0000_1000;
pub const OFN_EXPLORER: u32 = 0x0008_0000;

pub const REG_SZ: u32 = 1;
pub const ICC_LISTVIEW_CLASSES: u32 = 0x0000_0001;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}

#[repr(C)]
pub struct WNDCLASSEXW {
    pub cbSize: u32,
    pub style: u32,
    pub lpfnWndProc: WndProc,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: *const u16,
    pub lpszClassName: *const u16,
    pub hIconSm: HICON,
}

#[repr(C)]
pub struct PROCESSENTRY32W {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ProcessID: u32,
    pub th32DefaultHeapID: usize,
    pub th32ModuleID: u32,
    pub cntThreads: u32,
    pub th32ParentProcessID: u32,
    pub pcPriClassBase: i32,
    pub dwFlags: u32,
    pub szExeFile: [u16; 260],
}

#[repr(C)]
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}

pub type HIMAGELIST = *mut c_void;

#[repr(C)]
pub struct SHFILEINFOW {
    pub hIcon: HICON,
    pub iIcon: i32,
    pub dwAttributes: u32,
    pub szDisplayName: [u16; 260],
    pub szTypeName: [u16; 80],
}

#[repr(C)]
pub struct LVITEMW {
    pub mask: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: *mut u16,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: LPARAM,
    pub iIndent: i32,
    pub iGroupId: i32,
    pub cColumns: u32,
    pub puColumns: *mut u32,
    pub piColFmt: *mut i32,
    pub iGroup: i32,
}

impl LVITEMW {
    pub fn zeroed() -> Self {
        Self {
            mask: 0, iItem: 0, iSubItem: 0, state: 0, stateMask: 0,
            pszText: std::ptr::null_mut(), cchTextMax: 0, iImage: 0,
            lParam: 0, iIndent: 0, iGroupId: 0, cColumns: 0,
            puColumns: std::ptr::null_mut(), piColFmt: std::ptr::null_mut(), iGroup: 0,
        }
    }
}

#[repr(C)]
pub struct LVCOLUMNW {
    pub mask: u32,
    pub fmt: i32,
    pub cx: i32,
    pub pszText: *mut u16,
    pub cchTextMax: i32,
    pub iSubItem: i32,
    pub iImage: i32,
    pub iOrder: i32,
    pub cxMin: i32,
    pub cxDefault: i32,
    pub cxIdeal: i32,
}

impl LVCOLUMNW {
    pub fn zeroed() -> Self {
        Self {
            mask: 0, fmt: 0, cx: 0, pszText: std::ptr::null_mut(),
            cchTextMax: 0, iSubItem: 0, iImage: 0, iOrder: 0,
            cxMin: 0, cxDefault: 0, cxIdeal: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NMHDR {
    pub hwndFrom: HWND,
    pub idFrom: usize,
    pub code: u32,
}

#[repr(C)]
pub struct NMLISTVIEW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: POINT,
    pub lParam: LPARAM,
}

#[repr(C)]
pub struct NMITEMACTIVATE {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: POINT,
    pub lParam: LPARAM,
    pub uKeyFlags: u32,
}

#[repr(C)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: HICON,
    pub szTip: [u16; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u16; 256],
    pub uTimeoutOrVersion: u32,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: u32,
    pub guidItem: [u8; 16],
    pub hBalloonIcon: HICON,
}

#[repr(C)]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: HWND,
    pub hInstance: HINSTANCE,
    pub lpstrFilter: *const u16,
    pub lpstrCustomFilter: *mut u16,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: *mut u16,
    pub nMaxFile: u32,
    pub lpstrFileTitle: *mut u16,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: *const u16,
    pub lpstrTitle: *const u16,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: *const u16,
    pub lCustData: LPARAM,
    pub lpfnHook: *mut c_void,
    pub lpTemplateName: *const u16,
    pub pvReserved: *mut c_void,
    pub dwReserved: u32,
    pub FlagsEx: u32,
}

#[repr(C)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut c_void,
    pub bInheritHandle: i32,
}

#[repr(C)]
pub struct INITCOMMONCONTROLSEX {
    pub dwSize: u32,
    pub dwICC: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}

impl FILETIME {
    pub fn ticks(&self) -> u64 {
        ((self.dwHighDateTime as u64) << 32) | self.dwLowDateTime as u64
    }
}

#[repr(C)]
pub struct PROCESS_MEMORY_COUNTERS {
    pub cb: u32,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
}

#[repr(C)]
pub struct IO_COUNTERS {
    pub ReadOperationCount: u64,
    pub WriteOperationCount: u64,
    pub OtherOperationCount: u64,
    pub ReadTransferCount: u64,
    pub WriteTransferCount: u64,
    pub OtherTransferCount: u64,
}

#[repr(C)]
pub struct SYSTEM_INFO {
    pub wProcessorArchitecture: u16,
    pub wReserved: u16,
    pub dwPageSize: u32,
    pub lpMinimumApplicationAddress: *mut c_void,
    pub lpMaximumApplicationAddress: *mut c_void,
    pub dwActiveProcessorMask: usize,
    pub dwNumberOfProcessors: u32,
    pub dwProcessorType: u32,
    pub dwAllocationGranularity: u32,
    pub wProcessorLevel: u16,
    pub wProcessorRevision: u16,
}

#[repr(C)]
pub struct TVITEMW {
    pub mask: u32,
    pub hItem: isize,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: *mut u16,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: i32,
    pub lParam: isize,
}

impl TVITEMW {
    pub fn zeroed() -> Self {
        Self {
            mask: 0, hItem: 0, state: 0, stateMask: 0, pszText: std::ptr::null_mut(),
            cchTextMax: 0, iImage: 0, iSelectedImage: 0, cChildren: 0, lParam: 0,
        }
    }
}

#[repr(C)]
pub struct TVINSERTSTRUCTW {
    pub hParent: isize,
    pub hInsertAfter: isize,
    pub item: TVITEMW,
}

#[repr(C)]
pub struct TVHITTESTINFO {
    pub pt: POINT,
    pub flags: u32,
    pub hItem: isize,
}

#[repr(C)]
pub struct NMTREEVIEWW {
    pub hdr: NMHDR,
    pub action: u32,
    pub itemOld: TVITEMW,
    pub itemNew: TVITEMW,
    pub ptDrag: POINT,
}

#[repr(C)]
pub struct MIB_TCPROW_OWNER_PID {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
    pub dwOwningPid: u32,
}

#[repr(C)]
pub struct MIB_TCPTABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW_OWNER_PID; 1],
}

#[repr(C)]
pub struct MIB_UDPROW_OWNER_PID {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
}

#[repr(C)]
pub struct MIB_UDPTABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW_OWNER_PID; 1],
}

#[link(name = "kernel32")]
extern "system" {
    pub fn GetLastError() -> u32;
    pub fn GetCurrentProcessId() -> u32;
    pub fn GetCurrentProcess() -> HANDLE;
    pub fn GetModuleHandleW(lpModuleName: *const u16) -> HINSTANCE;
    pub fn GetProcAddress(hModule: HINSTANCE, lpProcName: *const u8) -> *mut c_void;
    pub fn GetModuleFileNameW(hModule: HINSTANCE, lpFilename: *mut u16, nSize: u32) -> u32;
    pub fn GetShortPathNameW(lpszLongPath: *const u16, lpszShortPath: *mut u16, cchBuffer: u32) -> u32;
    pub fn WideCharToMultiByte(CodePage: u32, dwFlags: u32, lpWideCharStr: *const u16,
        cchWideChar: i32, lpMultiByteStr: *mut i8, cbMultiByte: i32,
        lpDefaultChar: *const i8, lpUsedDefaultChar: *mut i32) -> i32;
    pub fn FreeConsole() -> i32;
    pub fn CloseHandle(hObject: HANDLE) -> i32;
    pub fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: i32, dwProcessId: u32) -> HANDLE;
    pub fn CreateToolhelp32Snapshot(dwFlags: u32, th32ProcessID: u32) -> HANDLE;
    pub fn Process32FirstW(hSnapshot: HANDLE, lppe: *mut PROCESSENTRY32W) -> i32;
    pub fn Process32NextW(hSnapshot: HANDLE, lppe: *mut PROCESSENTRY32W) -> i32;
    pub fn QueryFullProcessImageNameW(hProcess: HANDLE, dwFlags: u32, lpExeName: *mut u16, lpdwSize: *mut u32) -> i32;
    pub fn GetPriorityClass(hProcess: HANDLE) -> u32;
    pub fn SetPriorityClass(hProcess: HANDLE, dwPriorityClass: u32) -> i32;
    pub fn GetProcessTimes(
        hProcess: HANDLE, lpCreationTime: *mut FILETIME, lpExitTime: *mut FILETIME,
        lpKernelTime: *mut FILETIME, lpUserTime: *mut FILETIME,
    ) -> i32;
    pub fn GetProcessIoCounters(hProcess: HANDLE, lpIoCounters: *mut IO_COUNTERS) -> i32;
    pub fn GetTickCount64() -> u64;
    pub fn GetSystemInfo(lpSystemInfo: *mut SYSTEM_INFO);
    pub fn DeleteFileW(lpFileName: *const u16) -> i32;
    pub fn CreateMutexW(lpMutexAttributes: *const SECURITY_ATTRIBUTES, bInitialOwner: i32, lpName: *const u16) -> HANDLE;
    pub fn GetCommandLineW() -> *const u16;
    pub fn MoveFileExW(lpExistingFileName: *const u16, lpNewFileName: *const u16, dwFlags: u32) -> i32;
    pub fn CreateFileW(
        lpFileName: *const u16, dwDesiredAccess: u32, dwShareMode: u32,
        lpSecurityAttributes: *const SECURITY_ATTRIBUTES, dwCreationDisposition: u32,
        dwFlagsAndAttributes: u32, hTemplateFile: HANDLE,
    ) -> HANDLE;
}

#[link(name = "user32")]
extern "system" {
    pub fn DestroyIcon(hIcon: HICON) -> i32;
    pub fn RegisterClassExW(lpwcx: *const WNDCLASSEXW) -> u16;
    pub fn CreateWindowExW(
        dwExStyle: u32, lpClassName: *const u16, lpWindowName: *const u16, dwStyle: u32,
        x: i32, y: i32, nWidth: i32, nHeight: i32, hWndParent: HWND, hMenu: HMENU,
        hInstance: HINSTANCE, lpParam: *mut c_void,
    ) -> HWND;
    pub fn DestroyWindow(hWnd: HWND) -> i32;
    pub fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> i32;
    pub fn UpdateWindow(hWnd: HWND) -> i32;
    pub fn GetMessageW(lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: u32, wMsgFilterMax: u32) -> i32;
    pub fn TranslateMessage(lpMsg: *const MSG) -> i32;
    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
    pub fn PostQuitMessage(nExitCode: i32);
    pub fn DefWindowProcW(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn LoadIconW(hInstance: HINSTANCE, lpIconName: *const u16) -> HICON;
    pub fn LoadImageW(hInstance: HINSTANCE, lpszName: *const u16, uType: u32, cx: i32, cy: i32, fuLoad: u32) -> *mut c_void;
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: *const u16) -> HCURSOR;
    pub fn EnableWindow(hWnd: HWND, bEnable: i32) -> i32;
    pub fn GetWindow(hWnd: HWND, uCmd: u32) -> HWND;
    pub fn IsWindow(hWnd: HWND) -> i32;
    pub fn CheckDlgButton(hDlg: HWND, nIDButton: i32, uCheck: u32) -> i32;
    pub fn IsDlgButtonChecked(hDlg: HWND, nIDButton: i32) -> u32;
    pub fn SendMessageW(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn PostMessageW(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> i32;
    pub fn SetWindowTextW(hWnd: HWND, lpString: *const u16) -> i32;
    pub fn GetWindowTextLengthW(hWnd: HWND) -> i32;
    pub fn GetWindowTextW(hWnd: HWND, lpString: *mut u16, nMaxCount: i32) -> i32;
    pub fn MoveWindow(hWnd: HWND, x: i32, y: i32, nWidth: i32, nHeight: i32, bRepaint: i32) -> i32;
    pub fn SetWindowPos(hWnd: HWND, hWndInsertAfter: HWND, x: i32, y: i32, cx: i32, cy: i32, uFlags: u32) -> i32;
    pub fn GetClientRect(hWnd: HWND, lpRect: *mut RECT) -> i32;
    pub fn GetWindowRect(hWnd: HWND, lpRect: *mut RECT) -> i32;
    pub fn SetFocus(hWnd: HWND) -> HWND;
    pub fn GetFocus() -> HWND;
    pub fn GetDlgItem(hDlg: HWND, nIDDlgItem: i32) -> HWND;
    pub fn GetCursorPos(lpPoint: *mut POINT) -> i32;
    pub fn GetMessagePos() -> u32;
    pub fn ScreenToClient(hWnd: HWND, lpPoint: *mut POINT) -> i32;
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: i32, dwNewLong: isize) -> isize;
    pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: i32) -> isize;
    pub fn SetTimer(hWnd: HWND, nIDEvent: usize, uElapse: u32, lpTimerFunc: *mut c_void) -> usize;
    pub fn KillTimer(hWnd: HWND, nIDEvent: usize) -> i32;
    pub fn CreateMenu() -> HMENU;
    pub fn CreatePopupMenu() -> HMENU;
    pub fn AppendMenuW(hMenu: HMENU, uFlags: u32, uIDNewItem: usize, lpNewItem: *const u16) -> i32;
    pub fn SetMenu(hWnd: HWND, hMenu: HMENU) -> i32;
    pub fn TrackPopupMenu(
        hMenu: HMENU, uFlags: u32, x: i32, y: i32, nReserved: i32,
        hWnd: HWND, prcRect: *const RECT,
    ) -> i32;
    pub fn DestroyMenu(hMenu: HMENU) -> i32;
    pub fn CheckMenuItem(hMenu: HMENU, uIDCheckItem: u32, uCheck: u32) -> u32;
    pub fn EnableMenuItem(hMenu: HMENU, uIDEnableItem: u32, uEnable: u32) -> u32;
    pub fn MessageBoxW(hWnd: HWND, lpText: *const u16, lpCaption: *const u16, uType: u32) -> i32;
    pub fn GetSystemMetrics(nIndex: i32) -> i32;
    pub fn IsWindowVisible(hWnd: HWND) -> i32;
    pub fn SetForegroundWindow(hWnd: HWND) -> i32;
    pub fn ShowCursor(bShow: i32) -> i32;
}

#[link(name = "psapi")]
extern "system" {
    pub fn GetProcessMemoryInfo(hProcess: HANDLE, ppsmemCounters: *mut PROCESS_MEMORY_COUNTERS, cb: u32) -> i32;
}

#[link(name = "iphlpapi")]
extern "system" {
    pub fn GetExtendedTcpTable(
        pTcpTable: *mut c_void, pdwSize: *mut u32, bOrder: i32, ulAf: u32,
        TableClass: u32, Reserved: u32,
    ) -> u32;
    pub fn GetExtendedUdpTable(
        pUdpTable: *mut c_void, pdwSize: *mut u32, bOrder: i32, ulAf: u32,
        TableClass: u32, Reserved: u32,
    ) -> u32;
}

#[link(name = "shell32")]
extern "system" {
    /// 打开文件夹/文件/URL；explorer 是 shell 应用，必须用 ShellExecute 启动，
    /// 用 CreateProcess（std::process::Command）会失败并返回 os error 50。
    pub fn ShellExecuteW(
        hwnd: HWND, lpOperation: *const u16, lpFile: *const u16,
        lpParameters: *const u16, lpDirectory: *const u16, nShowCmd: i32,
    ) -> *mut c_void;
}

#[link(name = "gdi32")]
extern "system" {
    pub fn CreateFontW(
        cHeight: i32, cWidth: i32, cEscapement: i32, cOrientation: i32, cWeight: i32,
        bItalic: u32, bUnderline: u32, bStrikeOut: u32, iCharSet: u32,
        iOutPrecision: u32, iClipPrecision: u32, iQuality: u32, iPitchAndFamily: u32,
        pszFaceName: *const u16,
    ) -> HFONT;
    pub fn GetStockObject(i: i32) -> *mut c_void;
    pub fn DeleteObject(hObject: *mut c_void) -> i32;
}

#[link(name = "comctl32")]
extern "system" {
    pub fn InitCommonControlsEx(lpICC: *const INITCOMMONCONTROLSEX) -> i32;
    pub fn ImageList_Create(cx: i32, cy: i32, flags: u32, cInitial: i32, cGrow: i32) -> HIMAGELIST;
    pub fn ImageList_AddIcon(himl: HIMAGELIST, hicon: HICON) -> i32;
    pub fn ImageList_Destroy(himl: HIMAGELIST) -> i32;
}

#[link(name = "shell32")]
extern "system" {
    pub fn SHGetFileInfoW(pszPath: *const u16, dwFileAttributes: u32,
        psfi: *mut SHFILEINFOW, cbFileInfo: u32, uFlags: u32) -> usize;
    pub fn Shell_NotifyIconW(dwMessage: u32, lpData: *mut NOTIFYICONDATAW) -> i32;
    pub fn CommandLineToArgvW(lpCmdLine: *const u16, pNumArgs: *mut i32) -> *mut *mut u16;
    pub fn LocalFree(hMem: HANDLE) -> HANDLE;
}

#[link(name = "comdlg32")]
extern "system" {
    pub fn GetOpenFileNameW(lpofn: *mut OPENFILENAMEW) -> i32;
    pub fn GetSaveFileNameW(lpofn: *mut OPENFILENAMEW) -> i32;
}

#[link(name = "advapi32")]
extern "system" {
    pub fn RegOpenKeyExW(hKey: HKEY, lpSubKey: *const u16, ulOptions: u32, samDesired: u32, phkResult: *mut HKEY) -> i32;
    pub fn RegSetValueExW(hKey: HKEY, lpValueName: *const u16, lReserved: u32, dwType: u32, lpData: *const u8, cbData: u32) -> i32;
    pub fn RegQueryValueExW(hKey: HKEY, lpValueName: *const u16, lpReserved: *mut u32, lpType: *mut u32, lpData: *mut u8, lpcbData: *mut u32) -> i32;
    pub fn RegDeleteValueW(hKey: HKEY, lpValueName: *const u16) -> i32;
    pub fn RegCloseKey(hKey: HKEY) -> i32;
}

// ---------- COM：创建 Windows 快捷方式（.lnk）----------
#[repr(C)]
pub struct GUID { pub a: u32, pub b: u16, pub c: u16, pub d: [u8; 8] }
// CLSID_ShellLink = {00021401-0000-0000-C000-000000000046}
pub const CLSID_SHELL_LINK: GUID = GUID { a: 0x00021401, b: 0, c: 0, d: [0xC0,0,0,0,0,0,0,0x46] };
// IID_IShellLinkW = {000214F9-0000-0000-C000-000000000046}
pub const IID_ISHELL_LINK_W: GUID = GUID { a: 0x000214F9, b: 0, c: 0, d: [0xC0,0,0,0,0,0,0,0x46] };
// IID_IPersistFile = {0000010B-0000-0000-C000-000000000046}
pub const IID_IPERSIST_FILE: GUID = GUID { a: 0x0000010B, b: 0, c: 0, d: [0xC0,0,0,0,0,0,0,0x46] };
pub const CLSCTX_INPROC_SERVER: u32 = 0x1;
pub const COINIT_APARTMENTTHREADED: u32 = 0x2;

#[link(name = "ole32")]
extern "system" {
    pub fn CoInitializeEx(pvReserved: *mut c_void, dwCoInit: u32) -> i32;
    pub fn CoUninitialize();
    pub fn CoCreateInstance(rclsid: *const GUID, pUnkOuter: *mut c_void,
        dwClsContext: u32, riid: *const GUID, ppv: *mut *mut c_void) -> i32;
}

/// 用 IShellLinkW + IPersistFile 创建快捷方式。
/// target: 目标 exe 完整路径；args: 命令行参数；workdir: 工作目录；
/// desc: 备注；icon: 图标文件（空串则用目标 exe 图标）；lnk_path: 保存的 .lnk 路径。
pub unsafe fn create_shortcut(target: &str, args: &str, workdir: &str, desc: &str, icon: &str, lnk_path: &str) -> Result<(), String> {
    // vtable 布局（IShellLinkW 前 20 个方法）
    // 0..2 IUnknown; 3 GetPath 4 GetIDList 5 SetIDList 6 GetDescription 7 SetDescription
    // 8 GetWorkingDirectory 9 SetWorkingDirectory 10 GetArguments 11 SetArguments
    // 12 GetHotkey 13 SetHotkey 14 GetShowCmd 15 SetShowCmd
    // 16 GetIconLocation 17 SetIconLocation 18 SetRelativePath 19 Resolve 20 SetPath
    type SetStrFn = unsafe extern "system" fn(*mut c_void, *const u16) -> i32;
    type SetIconFn = unsafe extern "system" fn(*mut c_void, *const u16, i32) -> i32;
    type SetIntFn = unsafe extern "system" fn(*mut c_void, i32) -> i32;
    type QIFn = unsafe extern "system" fn(*mut c_void, *const GUID, *mut *mut c_void) -> i32;
    type RelFn = unsafe extern "system" fn(*mut c_void) -> u32;
    type SaveFn = unsafe extern "system" fn(*mut c_void, *const u16, i32) -> i32;

    let hr = CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED);
    // RPC_E_CHANGED_MODE (0x80010106) 表示已初始化为其它模式，仍可继续
    let need_uninit = hr == 0 || hr == 1;

    let mut link: *mut c_void = std::ptr::null_mut();
    let hr = CoCreateInstance(&CLSID_SHELL_LINK, std::ptr::null_mut(),
        CLSCTX_INPROC_SERVER, &IID_ISHELL_LINK_W, &mut link);
    if hr != 0 || link.is_null() {
        if need_uninit { CoUninitialize(); }
        return Err(format!("CoCreateInstance(IShellLinkW) 失败：0x{:08X}", hr as u32));
    }

    let vt = *(link as *const *const usize);
    let set_path: SetStrFn = std::mem::transmute(*vt.add(20));
    let set_args: SetStrFn = std::mem::transmute(*vt.add(11));
    let set_work: SetStrFn = std::mem::transmute(*vt.add(9));
    let set_desc: SetStrFn = std::mem::transmute(*vt.add(7));
    let set_icon: SetIconFn = std::mem::transmute(*vt.add(17));
    let set_show: SetIntFn = std::mem::transmute(*vt.add(15));
    let qi: QIFn = std::mem::transmute(*vt.add(0));
    let release: RelFn = std::mem::transmute(*vt.add(2));

    let t = to_wide(target); let a = to_wide(args); let w = to_wide(workdir);
    let d = to_wide(desc); let ic = to_wide(icon);
    set_path(link, t.as_ptr());
    set_args(link, a.as_ptr());
    set_work(link, w.as_ptr());
    set_desc(link, d.as_ptr());
    if !icon.is_empty() { set_icon(link, ic.as_ptr(), 0); }
    set_show(link, SW_SHOWNORMAL);

    let mut persist: *mut c_void = std::ptr::null_mut();
    let hr = qi(link, &IID_IPERSIST_FILE, &mut persist);
    let result = if hr != 0 || persist.is_null() {
        Err(format!("QueryInterface(IPersistFile) 失败：0x{:08X}", hr as u32))
    } else {
        let pvt = *(persist as *const *const usize);
        // IPersistFile: 0..2 IUnknown, 3 GetClassID, 4 IsDirty, 5 Load, 6 Save, 7 SaveCompleted, 8 GetCurFile
        let save: SaveFn = std::mem::transmute(*pvt.add(6));
        let prel: RelFn = std::mem::transmute(*pvt.add(2));
        let lp = to_wide(lnk_path);
        let hr = save(persist, lp.as_ptr(), 1);
        prel(persist);
        if hr == 0 { Ok(()) } else { Err(format!("保存快捷方式失败：0x{:08X}", hr as u32)) }
    };
    release(link);
    if need_uninit { CoUninitialize(); }
    result
}

/// &str -> 以 NUL 结尾的 UTF-16 缓冲区
pub fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// &str -> 系统 ANSI 代码页（中文系统=GBK）字节，供 cmd 批处理使用
pub fn to_ansi(s: &str) -> Vec<u8> {
    let wide = to_wide(s);
    unsafe {
        let n = WideCharToMultiByte(0, 0, wide.as_ptr(), -1,
            std::ptr::null_mut(), 0, std::ptr::null(), std::ptr::null_mut());
        if n <= 0 { return s.as_bytes().to_vec(); }
        let mut buf = vec![0i8; n as usize];
        WideCharToMultiByte(0, 0, wide.as_ptr(), -1,
            buf.as_mut_ptr(), n, std::ptr::null(), std::ptr::null_mut());
        buf.into_iter().take_while(|&b| b != 0).map(|b| b as u8).collect()
    }
}

/// UTF-16 指针 -> String
pub unsafe fn from_wide(ptr: *const u16) -> String {
    if ptr.is_null() {
        return String::new();
    }
    let mut len = 0usize;
    while *ptr.add(len) != 0 {
        len += 1;
        if len > 32768 {
            break;
        }
    }
    String::from_utf16_lossy(std::slice::from_raw_parts(ptr, len))
}

/// 按 ID 取子窗口句柄（i32 参数收窄，避免调用点到处转型）
pub unsafe fn GetDlgItemCompat(hwnd: HWND, id: u32) -> HWND {
    GetDlgItem(hwnd, id as i32)
}

/// 取鼠标屏幕坐标
pub unsafe fn GetCursorPosCompat(point: *mut POINT) -> i32 {
    GetCursorPos(point)
}

pub unsafe fn CheckDlgButtonCompat(hwnd: HWND, id: u32, checked: bool) {
    CheckDlgButton(hwnd, id as i32, if checked { 1 } else { 0 });
}

pub unsafe fn IsDlgButtonCheckedCompat(hwnd: HWND, id: u32) -> bool {
    IsDlgButtonChecked(hwnd, id as i32) == 1
}

/// 读取某个控件的文本
pub unsafe fn window_text(hwnd: HWND) -> String {
    let len = GetWindowTextLengthW(hwnd);
    if len <= 0 {
        return String::new();
    }
    let mut buf = vec![0u16; len as usize + 2];
    GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32);
    from_wide(buf.as_ptr())
}

pub fn last_error_text(op: &str) -> String {
    unsafe {
        let code = GetLastError();
        format!("{}失败（错误码 {}）", op, code)
    }
}
