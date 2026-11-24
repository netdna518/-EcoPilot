//! EcoPilot v3 - Windows 11 效率模式（EcoQoS）批量管理器
//! 纯 Rust + Win32，无第三方依赖。纯绿色：配置与规则全部在程序目录。

#![allow(non_snake_case)]

mod eco;
mod mini_json;
mod perf;
mod winapi;

use eco::{ApplyReport, Config, ProcessInfo, Rule};
use std::ffi::c_void;
use std::path::PathBuf;
use winapi as w;

const CLASS_NAME: &str = "EcoPilotMainWindowV3";
const SETTINGS_CLASS: &str = "EcoPilotSettingsDialogV3";
const APP_TITLE: &str = "自由效能进程EcoPilot-netdna518";

const ID_SEARCH: u32 = 1001;
const ID_BTN_REFRESH: u32 = 1002;
const ID_BTN_ECO_ON: u32 = 1003;
const ID_BTN_ECO_OFF: u32 = 1004;
const ID_BTN_ADD_RULE: u32 = 1005;
const ID_BTN_APPLY: u32 = 1006;
const ID_PROC_LIST: u32 = 1007;
const ID_ECO_TREE: u32 = 1008;
const ID_RULE_TREE: u32 = 1009;
const ID_BTN_SETTINGS: u32 = 1010;
const ID_BTN_RESTORE_ALL: u32 = 1011;
const ID_BTN_AUTO_REFRESH: u32 = 1017;
const ID_BTN_EXIT: u32 = 1018;
const ID_BTN_NEW_RULE: u32 = 1019;
const ID_BTN_RENAME_RULE: u32 = 1020;
const ID_BTN_AUTOSTART: u32 = 1023;
const ID_BTN_OPEN_STARTUP_ALL: u32 = 1024;
const ID_BTN_OPEN_STARTUP_USER: u32 = 1025;
const ID_STATIC_STATUS: u32 = 1012;
const ID_LABEL_PROC: u32 = 1013;
const ID_LABEL_ECO: u32 = 1014;
const ID_LABEL_RULE: u32 = 1015;
const ID_LABEL_SEARCH: u32 = 1016;

const IDM_IMPORT: u32 = 2001;
const IDM_EXPORT: u32 = 2002;
const IDM_APPLY: u32 = 2003;
const IDM_RESTORE_ALL: u32 = 2004;
const IDM_EXIT: u32 = 2005;
const IDM_REFRESH: u32 = 2006;
const IDM_AUTO_REFRESH: u32 = 2007;
const IDM_SETTINGS: u32 = 2008;
const IDM_ABOUT: u32 = 2009;
const IDM_PROT_ADD: u32 = 2010;
const IDM_PROT_REMOVE: u32 = 2011;
const IDM_REMOVE_FROM_RULE: u32 = 2012;
const IDM_DELETE_RULE: u32 = 2013;
const IDM_RESTORE_PID: u32 = 2014;

// 加入规则对话框
const PICK_CLASS: &str = "EcoPilotPickRuleDialog";
const ID_PICK_LIST: u32 = 3201;
const ID_PICK_NEWNAME: u32 = 3202;
const ID_PICK_OK: u32 = 3203;
const ID_PICK_CANCEL: u32 = 3204;
const ID_PICK_LABEL: u32 = 3205;

// 重命名规则对话框
// 关于对话框（自定义，便于放大字体）
const ABOUT_CLASS: &str = "EcoPilotAboutDialog";
const ID_ABOUT_TEXT: u32 = 3501;
const ID_ABOUT_OK: u32 = 3502;
const ABOUT_TEXT: &str = concat!(
    "自由效能进程EcoPilot-netdna518\r\n",
    "版本 v1.58 · 纯绿色效能模式管家\r\n\r\n",
    "效率模式 = Idle 优先级 + EcoQoS。\r\n",
    "全部设置/规则只存程序目录，不写注册表、不写系统目录。\r\n\r\n",
    "操作：\r\n",
    "1. 搜索框输入关键字实时过滤并定位。\r\n",
    "2. 点列头排序（再点切换升降序）。\r\n",
    "3. 双击进程切换效能模式。\r\n",
    "4. 右上：当前效能进程；右下：规则树，点规则展开其效能进程。\r\n",
    "5. 右键进程/规则节点可移出本规则。\r\n",
    "6. 每条规则是程序目录下一个 JSON 文件，几个文件几条规则。\r\n\r\n",
    "命令行：--selftest / --list / --apply / --tray / --config PATH"
);

// 应用结果对话框（失败进程清单 + 一键移出规则）
const APPLY_CLASS: &str = "EcoPilotApplyResultDialog";
const ID_APPLY_LIST: u32 = 3401;
const ID_APPLY_REMOVE: u32 = 3402;
const ID_APPLY_CLOSE: u32 = 3403;
const ID_APPLY_SUMMARY: u32 = 3404;
const LVS_EX_CHECKBOXES: u32 = 0x0000_0004;

struct ApplyDialogState {
    summary: String,
    failed: Vec<(String, u32, String, String)>,
    to_remove: Vec<(String, String)>, // (进程名, 规则名)
}

const RENAME_CLASS: &str = "EcoPilotRenameDialog";
const ID_RENAME_EDIT: u32 = 3301;
const ID_RENAME_OK: u32 = 3302;
const ID_RENAME_CANCEL: u32 = 3303;

const IDM_TRAY_SHOW: u32 = 2101;
const IDM_TRAY_APPLY: u32 = 2102;
const IDM_TRAY_RESTORE: u32 = 2103;
const IDM_TRAY_EXIT: u32 = 2104;

const WM_APP_APPLY: u32 = 0x8002;
const WM_APP_EXIT: u32 = 0x8003;
const LVM_SETITEMSTATE: u32 = w::LVM_FIRST + 43;
const LVIS_SELECTED: u32 = 0x0000_0002;
const IDI_APPLICATION: usize = 32512;

// 进程列表列索引
const COL_NAME: usize = 0;
const COL_PID: usize = 1;
const COL_CPU: usize = 2;
const COL_MEM: usize = 3;
const COL_DISK: usize = 4;
const COL_NET: usize = 5;
const COL_MODE: usize = 6;
const COL_PATH: usize = 7;

#[repr(C)]
struct CREATESTRUCTW {
    lpCreateParams: *mut c_void,
    hInstance: w::HINSTANCE,
    hMenu: w::HMENU,
    hwndParent: w::HWND,
    cy: i32, cx: i32, y: i32, x: i32, style: i32,
    lpszName: *const u16, lpszClass: *const u16, dwExStyle: u32,
}

struct SettingsState {
    result: Option<SettingsResult>,
    config: Config,
}

#[derive(Clone)]
struct SettingsResult {
    start_to_tray: bool,
    show_tray_icon: bool,
    auto_apply: bool,
    auto_apply_rules: Vec<String>,
    exit_after_apply: bool,
    auto_refresh: bool,
}

/// 加入规则对话框状态：结果 = 选中的规则索引，或新建规则名
struct PickState {
    result: Option<PickResult>,
    rule_names: Vec<String>,
    default_new: String,
}

enum PickResult {
    Existing(usize),
    New(String),
}

struct RenameState {
    result: Option<String>,
    initial: String,
}

struct App {
    hwnd: w::HWND,
    hproc: w::HWND,
    heco_tree: w::HWND,
    hrule_tree: w::HWND,
    hsearch: w::HWND,
    hstatus: w::HWND,
    hlabels: Vec<w::HWND>,
    hfont: w::HFONT,
    hfont_btn: w::HFONT,
    hmenu: w::HMENU,
    hicon: w::HICON,
    config_path: PathBuf,
    config: Config,
    procs: Vec<ProcessInfo>,
    visible_rows: Vec<usize>,
    sort_column: usize,
    sort_ascending: bool,
    tray: w::NOTIFYICONDATAW,
    tray_added: bool,
    auto_refresh: bool,
    start_to_tray: bool,
    last_message: String,
    initial_report: Option<ApplyReport>,
    sampler: perf::Sampler,
    /// 规则树节点句柄 -> 规则索引
    rule_nodes: Vec<(isize, usize)>,
    /// 规则下效能进程节点句柄 -> PID
    rule_proc_nodes: Vec<(isize, u32, usize, String)>,
    /// 规则树右键菜单当前命中的规则索引
    rule_menu_index: Option<usize>,
    /// 规则树右键命中的进程 PID（点在成员节点上时）
    rule_menu_pid: Option<u32>,
    /// 规则树右键命中的成员进程名（用于未运行进程也能移出）
    rule_menu_name: Option<String>,
    /// 进程列表小图标 image list 与缓存（路径小写 → 图标索引）
    himages: w::HIMAGELIST,
    icon_cache: std::collections::HashMap<String, i32>,
    default_icon: i32,
    /// 效能树右键菜单当前命中的进程 PID
    eco_menu_pid: Option<u32>,
    /// 记录展开的规则索引，自动刷新重建树后恢复
    expanded_rules: std::collections::HashSet<usize>,
    eco_tree_expanded: bool,
}

impl App {
    fn new(config_path: PathBuf, config: Config, start_to_tray: bool, initial_report: Option<ApplyReport>) -> Self {
        let auto_refresh = config.auto_refresh;
        Self {
            hwnd: std::ptr::null_mut(), hproc: std::ptr::null_mut(),
            heco_tree: std::ptr::null_mut(), hrule_tree: std::ptr::null_mut(),
            hsearch: std::ptr::null_mut(), hstatus: std::ptr::null_mut(), hlabels: Vec::new(),
            hfont: std::ptr::null_mut(), hfont_btn: std::ptr::null_mut(), hmenu: std::ptr::null_mut(), hicon: std::ptr::null_mut(),
            config_path, config, procs: Vec::new(), visible_rows: Vec::new(),
            sort_column: COL_NAME, sort_ascending: true,
            himages: std::ptr::null_mut(), icon_cache: std::collections::HashMap::new(), default_icon: 0,
            tray: unsafe { std::mem::zeroed() }, tray_added: false,
            auto_refresh, start_to_tray,
            last_message: "就绪。双击进程切换效率模式；点列头排序。".to_string(),
            initial_report, sampler: perf::Sampler::new(),
            rule_nodes: Vec::new(), rule_proc_nodes: Vec::new(),             rule_menu_index: None,
            rule_menu_pid: None,
            rule_menu_name: None,
            eco_menu_pid: None,
            expanded_rules: std::collections::HashSet::new(),
            eco_tree_expanded: true,
        }
    }

    fn create_control(&self, id: u32, class: &str, text: &str, style: u32, ex: u32) -> w::HWND {
        let hwnd = unsafe {
            w::CreateWindowExW(ex, w::to_wide(class).as_ptr(), w::to_wide(text).as_ptr(),
                style, 0, 0, 10, 10, self.hwnd, id as usize as w::HMENU,
                self_instance(), std::ptr::null_mut())
        };
        if !hwnd.is_null() {
            let font = if class == "Button" && !self.hfont_btn.is_null() { self.hfont_btn } else { self.hfont };
            if !font.is_null() {
                unsafe { w::SendMessageW(hwnd, w::WM_SETFONT, font as usize, 1); }
            }
        }
        hwnd
    }

    fn load_icon(&mut self) {
        let icon = unsafe {
            w::LoadImageW(self_instance(), 1usize as *const u16, w::IMAGE_ICON, 0, 0, w::LR_DEFAULTSIZE)
        };
        self.hicon = if icon.is_null() {
            unsafe { w::LoadIconW(std::ptr::null_mut(), IDI_APPLICATION as usize as *const u16) }
        } else { icon as w::HICON };
        unsafe {
            w::SendMessageW(self.hwnd, 0x0080, 1, self.hicon as isize);
            w::SendMessageW(self.hwnd, 0x0080, 0, self.hicon as isize);
        }
    }

    fn on_create(&mut self) {
        self.hfont = unsafe { w::CreateFontW(-16, 0, 0, 0, 400, 0, 0, 0, 1, 4, 0, 0, 0,
            w::to_wide("Microsoft YaHei UI").as_ptr()) };
        if self.hfont.is_null() { self.hfont = unsafe { w::GetStockObject(w::DEFAULT_GUI_FONT) }; }
        // 按钮字体小一号，保证文字在按钮内显示完整
        self.hfont_btn = unsafe { w::CreateFontW(-14, 0, 0, 0, 400, 0, 0, 0, 1, 4, 0, 0, 0,
            w::to_wide("Microsoft YaHei UI").as_ptr()) };
        if self.hfont_btn.is_null() { self.hfont_btn = self.hfont; }
        let icc = w::INITCOMMONCONTROLSEX {
            dwSize: std::mem::size_of::<w::INITCOMMONCONTROLSEX>() as u32,
            dwICC: w::ICC_LISTVIEW_CLASSES,
        };
        unsafe { w::InitCommonControlsEx(&icc); }
        self.load_icon();

        let mut labels = Vec::new();
        labels.push(self.create_control(ID_LABEL_SEARCH, "Static", "搜索：", 0, 0));
        self.hsearch = self.create_control(ID_SEARCH, "Edit", "",
            w::WS_CHILD | w::WS_VISIBLE | w::WS_BORDER | w::WS_TABSTOP | w::ES_AUTOHSCROLL,
            w::WS_EX_CLIENTEDGE);
        self.create_control(ID_BTN_REFRESH, "Button", "刷新 (F5)", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP, 0);
        self.create_control(ID_BTN_ECO_ON, "Button", "设为效能模式", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP, 0);
        self.create_control(ID_BTN_ECO_OFF, "Button", "移除效能模式", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP, 0);
        self.create_control(ID_BTN_ADD_RULE, "Button", "加入规则", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP, 0);
        self.create_control(ID_BTN_APPLY, "Button", "立即应用规则", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP | w::BS_DEFPUSHBUTTON, 0);
        self.create_control(ID_BTN_RESTORE_ALL, "Button", "移除所有效能", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP, 0);
        self.create_control(ID_BTN_AUTO_REFRESH, "Button", "自动刷新：开", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP, 0);
        self.create_control(ID_BTN_AUTOSTART, "Button", "生成开机启动脚本", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP, 0);
        self.create_control(ID_BTN_OPEN_STARTUP_ALL, "Button", "所有用户启动文件夹", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP, 0);
        self.create_control(ID_BTN_OPEN_STARTUP_USER, "Button", "当前用户启动文件夹", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP, 0);
        self.create_control(ID_BTN_SETTINGS, "Button", "设置", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP, 0);
        self.create_control(ID_BTN_EXIT, "Button", "退出", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP, 0);
        labels.push(self.create_control(ID_LABEL_PROC, "Static", "进程（双击切换 · 点列头排序 · 右键菜单）", 0, 0));
        labels.push(self.create_control(ID_LABEL_ECO, "Static", "当前效能进程", 0, 0));
        labels.push(self.create_control(ID_LABEL_RULE, "Static", "规则（点规则展开其效能进程 · 右键移出）", 0, 0));
        self.hlabels = labels;

        let list_style = w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP | w::WS_BORDER
            | w::WS_VSCROLL | w::LVS_REPORT | w::LVS_SHOWSELALWAYS;
        self.hproc = self.create_control(ID_PROC_LIST, "SysListView32", "", list_style, 0);
        unsafe {
            let ex = w::LVS_EX_FULLROWSELECT | w::LVS_EX_GRIDLINES | w::LVS_EX_DOUBLEBUFFER;
            w::SendMessageW(self.hproc, w::LVM_SETEXTENDEDLISTVIEWSTYLE, 0, ex as isize);
            // 进程图标：16x16 小图标 image list
            self.himages = w::ImageList_Create(16, 16, w::ILC_COLOR32 | w::ILC_MASK, 8, 32);
            let default_icon = get_or_add_icon(self, "");
            self.default_icon = default_icon;
            w::SendMessageW(self.hproc, w::LVM_SETIMAGELIST, w::LVSIL_SMALL, self.himages as isize);
        }
        self.add_columns(self.hproc, &[
            (170, "进程名"), (66, "PID"), (66, "CPU"), (92, "内存"),
            (92, "磁盘"), (70, "网络"), (76, "效能模式"), (300, "路径"),
        ]);

        let tree_style = w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP | w::WS_BORDER
            | w::WS_VSCROLL | w::TVS_HASLINES | w::TVS_LINESATROOT
            | w::TVS_HASBUTTONS | w::TVS_SHOWSELALWAYS;
        self.heco_tree = self.create_control(ID_ECO_TREE, "SysTreeView32", "", tree_style, 0);
        self.hrule_tree = self.create_control(ID_RULE_TREE, "SysTreeView32", "", tree_style, 0);
        self.create_control(ID_BTN_NEW_RULE, "Button", "新建规则", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP, 0);
        self.create_control(ID_BTN_RENAME_RULE, "Button", "重命名规则", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP, 0);
        self.hstatus = self.create_control(ID_STATIC_STATUS, "Static", "", 0, 0);

        self.hmenu = unsafe { w::CreateMenu() };
        let file = unsafe { w::CreatePopupMenu() };
        let view = unsafe { w::CreatePopupMenu() };
        let help = unsafe { w::CreatePopupMenu() };
        unsafe {
            w::AppendMenuW(self.hmenu, 0x0010, file as usize, w::to_wide("文件(&F)").as_ptr());
            w::AppendMenuW(self.hmenu, 0x0010, view as usize, w::to_wide("视图(&V)").as_ptr());
            w::AppendMenuW(self.hmenu, 0x0010, help as usize, w::to_wide("帮助(&H)").as_ptr());
            w::AppendMenuW(file, 0, IDM_SETTINGS as usize, w::to_wide("设置...").as_ptr());
            w::AppendMenuW(file, 0x0008, 0, std::ptr::null());
            w::AppendMenuW(file, 0, IDM_APPLY as usize, w::to_wide("立即应用规则").as_ptr());
            w::AppendMenuW(file, 0, IDM_RESTORE_ALL as usize, w::to_wide("移除所有进程的效能模式").as_ptr());
            w::AppendMenuW(file, 0x0008, 0, std::ptr::null());
            w::AppendMenuW(file, 0, IDM_EXIT as usize, w::to_wide("退出").as_ptr());
            w::AppendMenuW(view, 0, IDM_REFRESH as usize, w::to_wide("刷新进程列表\tF5").as_ptr());
            w::AppendMenuW(view, 0, IDM_AUTO_REFRESH as usize, w::to_wide("自动刷新").as_ptr());
            w::AppendMenuW(help, 0, IDM_ABOUT as usize, w::to_wide("关于 / 使用说明").as_ptr());
            w::SetMenu(self.hwnd, self.hmenu);
            let rf = if self.auto_refresh { w::MF_CHECKED } else { w::MF_UNCHECKED };
            unsafe {
                w::CheckMenuItem(self.hmenu, IDM_AUTO_REFRESH, w::MF_BYCOMMAND | rf);
                let btn = if self.auto_refresh { "自动刷新：开" } else { "自动刷新：关" };
                w::SetWindowTextW(self.control(ID_BTN_AUTO_REFRESH), w::to_wide(btn).as_ptr());
            }
        }

        self.setup_tray();
        // 定时器由 sync_timer 统一管理：仅窗口可见且开启自动刷新时运行
        // 规则默认全部展开，用户一眼看到成员；之后单击折叠/展开状态会被记住
        for i in 0..self.config.rules.len() { self.expanded_rules.insert(i); }
        self.refresh_trees();
        self.refresh_processes(true, false);
        self.update_status();
        if self.initial_report.is_some() && !self.start_to_tray {
            unsafe { w::PostMessageW(self.hwnd, WM_APP_APPLY, 0, 0); }
        } else { self.initial_report = None; }
    }

    fn add_columns(&self, list: w::HWND, columns: &[(i32, &str)]) {
        for (i, (width, title)) in columns.iter().enumerate() {
            let mut text = w::to_wide(title);
            let mut col = w::LVCOLUMNW::zeroed();
            col.mask = w::LVCF_FMT | w::LVCF_WIDTH | w::LVCF_TEXT | w::LVCF_SUBITEM;
            col.fmt = w::LVCFMT_LEFT;
            col.cx = *width;
            col.pszText = text.as_mut_ptr();
            col.iSubItem = i as i32;
            unsafe { w::SendMessageW(list, w::LVM_INSERTCOLUMNW, i, &col as *const _ as isize); }
        }
    }

    fn setup_tray(&mut self) {
        if !self.config.show_tray_icon { return; }
        let mut tray: w::NOTIFYICONDATAW = unsafe { std::mem::zeroed() };
        tray.cbSize = std::mem::size_of::<w::NOTIFYICONDATAW>() as u32;
        tray.hWnd = self.hwnd;
        tray.uID = 1;
        tray.uFlags = w::NIF_MESSAGE | w::NIF_ICON | w::NIF_TIP;
        tray.uCallbackMessage = w::WM_APP_TRAY;
        tray.hIcon = self.hicon;
        let tip = w::to_wide(APP_TITLE);
        for (slot, ch) in tray.szTip.iter_mut().zip(tip.iter()) { *slot = *ch; }
        self.tray = tray;
        unsafe { w::Shell_NotifyIconW(w::NIM_ADD, &mut self.tray); }
        self.tray_added = true;
    }

    fn remove_tray(&mut self) {
        if self.tray_added {
            unsafe { w::Shell_NotifyIconW(w::NIM_DELETE, &mut self.tray); }
            self.tray_added = false;
        }
    }

    fn layout(&self) {
        let mut rect = w::RECT::default();
        unsafe { w::GetClientRect(self.hwnd, &mut rect); }
        let cw = rect.right; let ch = rect.bottom;
        let split = ((cw - 30) * 68 / 100).max(620).min(cw - 300);
        let right_x = split + 16;
        let right_w = (cw - right_x - 10).max(180);
        unsafe {
            w::MoveWindow(self.hlabels[0], 10, 14, 52, 22, 1);
            w::MoveWindow(self.hsearch, 64, 8, 190, 28, 1);
            let mut x = 266;
            for (id, width) in [
                (ID_BTN_REFRESH, 70), (ID_BTN_ECO_ON, 105), (ID_BTN_ECO_OFF, 108),
                (ID_BTN_ADD_RULE, 80), (ID_BTN_APPLY, 105),
                (ID_BTN_RESTORE_ALL, 100), (ID_BTN_AUTO_REFRESH, 100),
                (ID_BTN_AUTOSTART, 120), (ID_BTN_OPEN_STARTUP_ALL, 130), (ID_BTN_OPEN_STARTUP_USER, 130),
                (ID_BTN_SETTINGS, 56), (ID_BTN_EXIT, 48),
            ] {
                w::MoveWindow(self.control(id), x, 8, width, 28, 1);
                x += width + 5;
            }
            w::MoveWindow(self.hlabels[1], 10, 46, split - 10, 20, 1);
            let list_y = 70;
            let list_h = (ch - list_y - 40).max(80);
            w::MoveWindow(self.hproc, 10, list_y, split - 10, list_h, 1);
            // 右侧两树加高，令底部按钮与左侧进程列表底边对齐
            let half = (list_h - 60) / 2;
            w::MoveWindow(self.hlabels[2], right_x, 46, right_w, 20, 1);
            w::MoveWindow(self.heco_tree, right_x, list_y, right_w, half, 1);
            w::MoveWindow(self.hlabels[3], right_x, list_y + half + 4, right_w, 20, 1);
            let rule_tree_y = list_y + half + 26;
            let rule_tree_h = (list_h - half - 56).max(80);
            w::MoveWindow(self.hrule_tree, right_x, rule_tree_y, right_w, rule_tree_h, 1);
            let rule_btn_y = rule_tree_y + rule_tree_h + 4;
            w::MoveWindow(self.control(ID_BTN_NEW_RULE), right_x, rule_btn_y, right_w / 2 - 4, 26, 1);
            w::MoveWindow(self.control(ID_BTN_RENAME_RULE), right_x + right_w / 2 + 4, rule_btn_y, right_w / 2 - 4, 26, 1);
            w::MoveWindow(self.hstatus, 10, ch - 26, cw - 20, 20, 1);
        }
    }

    fn control(&self, id: u32) -> w::HWND {
        unsafe { w::GetDlgItemCompat(self.hwnd, id) }
    }

    /// keep_selection=保留选中；focus_match=搜索后自动定位第一个匹配项
    fn refresh_processes(&mut self, keep_selection: bool, focus_match: bool) {
        let previous_selection = if keep_selection { selected_pids(self.hproc) } else { Vec::new() };
        let search = unsafe { w::window_text(self.hsearch) }.trim().to_lowercase();
        self.procs = eco::list_processes_with_metrics(Some(&mut self.sampler));

        // 过滤 + 排序（在 Rust 里排，避免 LVM_SORTITEMS 回调的 FFI 复杂性）
        let mut indices: Vec<usize> = self.procs.iter().enumerate()
            .filter(|(_, p)| search.is_empty()
                || p.name.to_lowercase().contains(&search)
                || p.path.to_lowercase().contains(&search))
            .map(|(i, _)| i).collect();
        let col = self.sort_column;
        let asc = self.sort_ascending;
        let procs = &self.procs;
        indices.sort_by(|&a, &b| {
            let pa = &procs[a]; let pb = &procs[b];
            let ord = match col {
                COL_PID => pa.pid.cmp(&pb.pid),
                COL_CPU => pa.cpu.partial_cmp(&pb.cpu).unwrap_or(std::cmp::Ordering::Equal),
                COL_MEM => pa.memory_mb.partial_cmp(&pb.memory_mb).unwrap_or(std::cmp::Ordering::Equal),
                COL_DISK => pa.disk_kb_s.partial_cmp(&pb.disk_kb_s).unwrap_or(std::cmp::Ordering::Equal),
                COL_NET => pa.connections.cmp(&pb.connections),
                COL_MODE => mode_rank(pa, &self.config)
                    .cmp(&mode_rank(pb, &self.config))
                    .then_with(|| pa.name.to_lowercase().cmp(&pb.name.to_lowercase())),
                COL_PATH => pa.path.to_lowercase().cmp(&pb.path.to_lowercase()),
                _ => pa.name.to_lowercase().cmp(&pb.name.to_lowercase()),
            };
            if asc { ord } else { ord.reverse() }
        });

        // 先提取并缓存图标（需要 &mut self，避免与 self.procs 借用冲突）
        let icon_of: Vec<i32> = indices.iter().map(|&i| {
            let path = self.procs[i].path.clone();
            get_or_add_icon(self, &path)
        }).collect();
        unsafe { w::SendMessageW(self.hproc, w::LVM_DELETEALLITEMS, 0, 0); }
        self.visible_rows = indices.clone();
        let mut rows: Vec<(Vec<String>, isize, i32)> = Vec::new();
        for (row, &index) in indices.iter().enumerate() {
            let p = &self.procs[index];
            let mode = if p.is_eco() { "已效能" }
                else {
                    match eco::protection_kind(&p.name, &self.config) {
                        1 => "内置保护",
                        2 => "用户保护",
                        _ => "普通",
                    }
                };
            rows.push((vec![
                p.name.clone(), p.pid.to_string(), perf::format_cpu(p.cpu),
                perf::format_memory(p.memory_mb), perf::format_disk(p.disk_kb_s),
                if p.connections > 0 { format!("{} 连接", p.connections) } else { "-".to_string() },
                mode.to_string(), p.path.clone(),
            ], p.pid as isize, icon_of[row]));
        }
        for (row, (cells, param, icon)) in rows.iter().enumerate() {
            self.insert_row(self.hproc, row, cells, *param, *icon);
        }

        if focus_match && !search.is_empty() && !indices.is_empty() {
            self.select_row(self.hproc, 0);
            unsafe { w::SendMessageW(self.hproc, w::LVM_ENSUREVISIBLE, 0, 0); }
        } else if !previous_selection.is_empty() {
            let to_select: Vec<i32> = self.visible_rows.iter().enumerate()
                .filter(|(_, index)| previous_selection.contains(&(self.procs[**index].pid as isize)))
                .map(|(row, _)| row as i32).collect();
            for row in to_select { self.select_row(self.hproc, row); }
        }
        self.refresh_trees();
        self.update_status();
    }

    fn insert_row(&self, list: w::HWND, row: usize, cells: &[String], param: isize, icon: i32) {
        let mut first = w::to_wide(&cells[0]);
        let mut item = w::LVITEMW::zeroed();
        item.mask = w::LVIF_TEXT | w::LVIF_PARAM | w::LVIF_IMAGE;
        item.iItem = row as i32; item.iSubItem = 0;
        item.pszText = first.as_mut_ptr(); item.lParam = param; item.iImage = icon;
        unsafe { w::SendMessageW(list, w::LVM_INSERTITEMW, 0, &item as *const _ as isize); }
        for (i, cell) in cells.iter().enumerate().skip(1) {
            let mut text = w::to_wide(cell);
            let mut sub = w::LVITEMW::zeroed();
            sub.mask = w::LVIF_TEXT; sub.iItem = row as i32; sub.iSubItem = i as i32;
            sub.pszText = text.as_mut_ptr();
            unsafe { w::SendMessageW(list, w::LVM_SETITEMW, 0, &sub as *const _ as isize); }
        }
    }

    fn select_row(&self, list: w::HWND, row: i32) {
        let mut item = w::LVITEMW::zeroed();
        item.state = LVIS_SELECTED; item.stateMask = LVIS_SELECTED;
        unsafe { w::SendMessageW(list, LVM_SETITEMSTATE, row as usize, &item as *const _ as isize); }
    }

    fn add_tree_item(&self, tree: w::HWND, parent: isize, text: &str, lparam: isize) -> isize {
        let mut label = w::to_wide(text);
        let mut item = w::TVITEMW::zeroed();
        item.mask = w::TVIF_TEXT | w::TVIF_PARAM;
        item.pszText = label.as_mut_ptr(); item.lParam = lparam;
        let insert = w::TVINSERTSTRUCTW { hParent: parent, hInsertAfter: -0x0FFFE, item };
        unsafe { w::SendMessageW(tree, w::TVM_INSERTITEMW, 0, &insert as *const _ as isize) as isize }
    }

    fn refresh_trees(&mut self) {
        // 右上：已效能进程（展开状态不受自动刷新影响）
        unsafe { w::SendMessageW(self.heco_tree, w::TVM_DELETEITEM, 0, w::TVI_ROOT); }
        let eco_data: Vec<(String, u32, String)> = self.procs.iter().filter(|p| p.is_eco())
            .map(|p| (p.name.clone(), p.pid, perf::format_cpu(p.cpu))).collect();
        let root = self.add_tree_item(self.heco_tree, 0, &format!("当前效能进程 ({})", eco_data.len()), 0x7000_0000);
        for (name, pid, cpu) in eco_data {
            self.add_tree_item(self.heco_tree, root, &format!("{}  PID {}  {}", name, pid, cpu), pid as isize);
        }
        if self.eco_tree_expanded {
            unsafe { w::SendMessageW(self.heco_tree, w::TVM_EXPAND, w::TVE_EXPAND as usize, root); }
        }

        // 右下：规则树（每条规则展开显示其所有成员进程 + 状态）
        unsafe { w::SendMessageW(self.hrule_tree, w::TVM_DELETEITEM, 0, w::TVI_ROOT); }
        self.rule_nodes.clear(); self.rule_proc_nodes.clear();
        let rule_data: Vec<(String, usize, Vec<(String, u32, bool)>)> = self.config.rules.iter().enumerate()
            .map(|(i, r)| {
                let label = format!("{}{}（{} 个进程）",
                    if r.enabled { "" } else { "[停用] " }, r.display_name(), r.members.len());
                // 显示规则的全部成员进程，标注是否在效能模式
                let members: Vec<(String, u32, bool)> = r.members.iter().map(|m| {
                    if let Some(proc) = self.procs.iter().find(|p| p.name.eq_ignore_ascii_case(m)) {
                        (proc.name.clone(), proc.pid, proc.is_eco())
                    } else {
                        (m.clone(), 0, false)
                    }
                }).collect();
                (label, i, members)
            }).collect();
        for (label, index, members) in rule_data {
            let node = self.add_tree_item(self.hrule_tree, 0, &label, 0x6000_0000 | index as isize);
            self.rule_nodes.push((node, index));
            if members.is_empty() {
                self.add_tree_item(self.hrule_tree, node, "（暂无成员，选中进程后点加入规则）", 0x5000_0000);
            } else {
                for (name, pid, on) in members {
                    let text = if pid == 0 {
                        format!("{}（未运行）", name)
                    } else {
                        format!("{}  PID {}  {}", name, pid, if on { "效能中" } else { "普通" })
                    };
                    let child = self.add_tree_item(self.hrule_tree, node, &text, pid as isize);
                    self.rule_proc_nodes.push((child, pid, index, name.clone()));
                }
            }
            // 恢复该规则的展开状态
            if self.expanded_rules.contains(&index) {
                unsafe { w::SendMessageW(self.hrule_tree, w::TVM_EXPAND, w::TVE_EXPAND as usize, node); }
            }
        }
    }

    fn update_status(&self) {
        let eco_count = self.procs.iter().filter(|p| p.is_eco()).count();
        let enabled = self.config.rules.iter().filter(|r| r.enabled).count();
        let text = format!("进程 {}（效能 {}） | 规则 {} 条（启用 {}） | {} | 配置目录：{}",
            self.procs.len(), eco_count, self.config.rules.len(), enabled,
            self.last_message.chars().take(160).collect::<String>(), eco::app_dir().display());
        unsafe { w::SetWindowTextW(self.hstatus, w::to_wide(&text).as_ptr()); }
    }

    fn selected_processes(&self) -> Vec<ProcessInfo> {
        let pids = selected_pids(self.hproc);
        self.procs.iter().filter(|p| pids.contains(&(p.pid as isize))).cloned().collect()
    }

    fn cmd_set_efficiency(&mut self, enable: bool) {
        let targets = self.selected_processes();
        if targets.is_empty() {
            self.message("请先选中至少一个进程。", "提示", w::MB_ICONINFORMATION); return;
        }
        let mut ok = 0; let mut errors = Vec::new(); let mut prot = Vec::new();
        for p in &targets {
            if eco::is_protected(&p.name, &self.config) { prot.push(p.name.clone()); continue; }
            let restore = self.config.rules.iter().find(|r| r.matches(p)).map(|r| r.restore_priority)
                .unwrap_or(if p.priority == w::IDLE_PRIORITY_CLASS { w::NORMAL_PRIORITY_CLASS } else { p.priority });
            match eco::set_efficiency(p.pid, enable, restore) { Ok(_) => ok += 1, Err(e) => errors.push(e) }
        }
        self.last_message = format!("处理 {} 个，失败 {} 个。", ok, errors.len());
        if !prot.is_empty() { self.last_message.push_str(&format!(" 受保护：{}。", prot.join("、"))); }
        self.refresh_processes(true, false);
        if !errors.is_empty() {
            self.message(&format!("成功 {}，失败 {}：\n\n{}", ok, errors.len(), errors[..errors.len().min(6)].join("\n")),
                "部分失败", w::MB_ICONERROR);
        }
    }

    fn cmd_toggle_process(&mut self, row: i32) {
        let Some(&index) = self.visible_rows.get(row as usize) else { return };
        let p = self.procs[index].clone();
        if eco::is_protected(&p.name, &self.config) {
            self.message(&format!("{} 在保护名单中。", p.name), "受保护进程", w::MB_ICONINFORMATION); return;
        }
        let enable = !p.is_eco();
        let restore = self.config.rules.iter().find(|r| r.matches(&p)).map(|r| r.restore_priority)
            .unwrap_or(w::NORMAL_PRIORITY_CLASS);
        match eco::set_efficiency(p.pid, enable, restore) {
            Ok(msg) => self.last_message = msg,
            Err(msg) => { self.last_message = msg.clone(); self.message(&msg, "设置失败", w::MB_ICONERROR); }
        }
        self.refresh_processes(true, false);
    }

    fn save_rules(&mut self) {
        if let Err(msg) = eco::save_rules_to_files(&self.config) {
            self.message(&msg, "保存规则失败", w::MB_ICONERROR);
        }
        // 规则增删改会同步影响 auto_apply_rules（重命名/删除），必须一并写回 settings.json，
        // 否则内存里改了名、磁盘里还是旧名，下次启动就匹配不到规则。
        if let Err(msg) = eco::save_config(&self.config_path, &self.config) {
            self.message(&msg, "保存配置失败", w::MB_ICONERROR);
        }
        self.last_message = format!("规则已保存到 {}（{} 个 JSON 文件）。", eco::rules_dir().display(), self.config.rules.len());
        for i in 0..self.config.rules.len() { self.expanded_rules.insert(i); }
        self.refresh_trees();
        self.update_status();
    }

    /// 加入规则：弹对话框让用户选已有规则或新建规则，而不是自动建规则
    fn cmd_add_rule(&mut self) {
        let targets = self.selected_processes();
        if targets.is_empty() { self.message("请先选中进程。", "提示", w::MB_ICONINFORMATION); return; }
        let rule_names: Vec<String> = self.config.rules.iter().map(|r| r.name.clone()).collect();
        let default_new = targets[0].name.clone();
        let mut state = PickState { result: None, rule_names, default_new };
        unsafe { show_pick_dialog(self.hwnd, &mut state); }
        let Some(result) = state.result.take() else { return };
        let rule_index = match result {
            PickResult::Existing(i) => i,
            PickResult::New(name) => {
                let name = name.trim().to_string();
                if name.is_empty() { return; }
                if let Some(existing) = self.config.rules.iter().position(|r| r.name.eq_ignore_ascii_case(&name)) {
                    existing
                } else {
                    self.config.rules.push(Rule {
                        name: name.clone(), members: Vec::new(), path: String::new(),
                        match_path: false, enabled: true,
                        restore_priority: w::NORMAL_PRIORITY_CLASS, note: String::new(),
                    });
                    self.config.rules.len() - 1
                }
            }
        };
        let mut added = 0;
        for p in &targets {
            let rule = &mut self.config.rules[rule_index];
            if !rule.members.iter().any(|m| m.eq_ignore_ascii_case(&p.name)) {
                rule.members.push(p.name.clone());
                added += 1;
            }
        }
        self.save_rules();
        self.last_message = format!("已把 {} 个进程加入规则「{}」。", added, self.config.rules[rule_index].name);
    }

    /// 新建空规则（规则树右键）
    fn cmd_new_rule(&mut self) {
        let mut state = RenameState { result: None, initial: "新建规则".to_string() };
        unsafe { show_rename_dialog(self.hwnd, &mut state, "新建规则"); }
        let Some(name) = state.result.take() else { return };
        let name = name.trim().to_string();
        if name.is_empty() { return; }
        if self.config.rules.iter().any(|r| r.name.eq_ignore_ascii_case(&name)) {
            self.message("同名规则已存在。", "提示", w::MB_ICONINFORMATION); return;
        }
        self.config.rules.push(Rule {
            name, members: Vec::new(), path: String::new(), match_path: false,
            enabled: true, restore_priority: w::NORMAL_PRIORITY_CLASS, note: String::new(),
        });
        self.save_rules();
    }

    /// 重命名规则（规则树右键）
    fn cmd_rename_rule(&mut self, index: usize) {
        if index >= self.config.rules.len() { return; }
        let old = self.config.rules[index].name.clone();
        let mut state = RenameState { result: None, initial: old.clone() };
        unsafe { show_rename_dialog(self.hwnd, &mut state, "重命名规则"); }
        let Some(name) = state.result.take() else { return };
        let name = name.trim().to_string();
        if name.is_empty() || name.eq_ignore_ascii_case(&old) { return; }
        if self.config.rules.iter().any(|r| r.name.eq_ignore_ascii_case(&name)) {
            self.message("同名规则已存在。", "提示", w::MB_ICONINFORMATION); return;
        }
        self.config.rules[index].name = name.clone();
        self.config.auto_apply_rules.iter_mut()
            .filter(|n| n.eq_ignore_ascii_case(&old))
            .for_each(|n| *n = name.clone());
        self.save_rules();
    }

    /// 移出本规则：从命中规则里移除该进程名，而不是删除整条规则
    fn cmd_remove_from_rule(&mut self) {
        // 规则树右键：用命中的 PID 精确定位进程名（不依赖左侧列表选中）
        if let Some(pid) = self.rule_menu_pid.take() {
            // 优先用右键命中时记录的名字（对未运行进程也有效），否则按 PID 查
            let name = self.rule_menu_name.take()
                .or_else(|| self.procs.iter().find(|p| p.pid == pid).map(|p| p.name.clone()));
            let idx = self.rule_menu_index.take();
            if let (Some(name), Some(i)) = (name, idx) {
                if i < self.config.rules.len() {
                    let rule_name = self.config.rules[i].name.clone();
                    let before = self.config.rules[i].members.len();
                    self.config.rules[i].members.retain(|m| !m.eq_ignore_ascii_case(&name));
                    if self.config.rules[i].members.len() != before {
                        self.save_rules();
                        self.last_message = format!("已把 {} 从规则「{}」移出。", name, rule_name);
                        return;
                    }
                }
            } else if let Some(i) = idx {
                // 进程不在运行列表：用规则成员里所有条目兜底不清（避免误删）
                let _ = i;
            }
            self.message("该进程不在规则成员中，或进程已退出。", "提示", w::MB_ICONINFORMATION);
            return;
        }
        self.rule_menu_index = None;
        let targets: Vec<String> = self.selected_processes().iter().map(|p| p.name.clone()).collect();
        if targets.is_empty() { self.message("请先选中进程。", "提示", w::MB_ICONINFORMATION); return; }
        let mut changed = 0;
        for rule in &mut self.config.rules {
            let before = rule.members.len();
            rule.members.retain(|m| !targets.iter().any(|n| n.eq_ignore_ascii_case(m)));
            if rule.members.len() != before { changed += 1; }
        }
        self.save_rules();
        self.last_message = format!("已从 {} 条规则中移出这些进程。", changed);
    }

    fn cmd_delete_rule(&mut self, index: usize) {
        if index < self.config.rules.len() {
            let name = self.config.rules[index].name.clone();
            self.config.rules.remove(index);
            self.config.auto_apply_rules.retain(|n| !n.eq_ignore_ascii_case(&name));
            self.save_rules();
            self.last_message = format!("已删除规则 {}。", name);
        }
    }

    fn cmd_apply_rules(&mut self) {
        let report = eco::apply_rules(&self.config, &self.procs, &[]);
        self.report_apply(&report);
    }

    fn report_apply(&mut self, report: &ApplyReport) {
        self.last_message = format!("匹配 {}，成功 {}，失败 {}。", report.matched, report.applied.len(), report.failed.len());
        self.refresh_processes(true, false);
        if report.failed.is_empty() {
            let extra = if report.matched == 0 { "（无规则命中：规则成员进程当前未运行）" } else { "" };
            self.last_message.push_str(extra);
            self.update_status();
            self.message(&format!("已应用：匹配 {}，成功 {}。{}", report.matched, report.applied.len(), extra),
                "应用结果", w::MB_ICONINFORMATION);
            return;
        }
        // 有失败：弹出失败进程清单，可勾选一键从规则移出
        let summary = format!("匹配 {}，成功 {}，失败 {}\n失败的进程多为瞬时进程已退出，或权限不足（可用管理员身份运行）。\n勾选后点「从规则移出」以后不再尝试。",
            report.matched, report.applied.len(), report.failed.len());
        let mut state = ApplyDialogState {
            summary, failed: report.failed_procs.clone(), to_remove: Vec::new(),
        };
        unsafe { show_apply_dialog(self.hwnd, &mut state); }
        // 处理勾选的移出请求
        if !state.to_remove.is_empty() {
            let mut removed = 0;
            for (name, rule_name) in &state.to_remove {
                if let Some(rule) = self.config.rules.iter_mut().find(|r| r.name.eq_ignore_ascii_case(rule_name)) {
                    let before = rule.members.len();
                    rule.members.retain(|m| !m.eq_ignore_ascii_case(name));
                    if rule.members.len() != before { removed += 1; }
                }
            }
            if removed > 0 {
                self.save_rules();
                self.last_message = format!("已从规则移出 {} 个失败进程。", removed);
                self.update_status();
            }
        }
    }

    /// 从右上效能树右键恢复单个进程
    fn cmd_restore_pid(&mut self, pid: u32) {
        let restore = self.procs.iter().find(|p| p.pid == pid)
            .and_then(|p| self.config.rules.iter().find(|r| r.matches(p)).map(|r| r.restore_priority))
            .unwrap_or(w::NORMAL_PRIORITY_CLASS);
        match eco::set_efficiency(pid, false, restore) {
            Ok(msg) => self.last_message = msg,
            Err(msg) => { self.last_message = msg.clone(); self.message(&msg, "恢复失败", w::MB_ICONERROR); }
        }
        self.refresh_processes(true, false);
    }

    fn cmd_restore_all(&mut self) {
        let (done, errors) = eco::restore_all(&self.procs, &self.config);
        self.last_message = format!("已恢复 {} 个，失败 {} 个。", done, errors.len());
        self.refresh_processes(true, false);
        if !errors.is_empty() {
            self.message(&errors[..errors.len().min(6)].join("\n"), "部分失败", w::MB_ICONERROR);
        }
    }

    fn cmd_protect(&mut self, add: bool) {
        let targets = self.selected_processes();
        if targets.is_empty() { self.message("请先选中进程。", "提示", w::MB_ICONINFORMATION); return; }
        let mut changed = 0;
        for p in &targets {
            let lower = p.name.to_lowercase();
            if add {
                if !self.config.protected.iter().any(|x| x.to_lowercase() == lower) {
                    self.config.protected.push(p.name.clone()); changed += 1;
                }
            } else {
                let n = self.config.protected.len();
                self.config.protected.retain(|x| x.to_lowercase() != lower);
                if self.config.protected.len() != n { changed += 1; }
            }
        }
        if let Err(msg) = eco::save_config(&self.config_path, &self.config) {
            self.message(&msg, "保存失败", w::MB_ICONERROR);
        }
        self.last_message = format!("保护名单{} {} 个。", if add { "新增" } else { "移除" }, changed);
        self.refresh_processes(true, false);
    }

    fn cmd_import(&mut self) {
        let Some(path) = (unsafe { file_dialog(self.hwnd, false, "导入规则 JSON（合并到 rules 目录）",
            "JSON\0*.json\0所有文件\0*.*\0\0", "") }) else { return };
        let text = match std::fs::read_to_string(&path) {
            Ok(t) => t, Err(e) => { self.message(&format!("读取失败：{}", e), "导入失败", w::MB_ICONERROR); return; }
        };
        let root = match mini_json::parse(&text) {
            Ok(j) => j, Err(e) => { self.message(&format!("格式错误：{}", e), "导入失败", w::MB_ICONERROR); return; }
        };
        if let Some(items) = root.get("rules").and_then(mini_json::Json::as_arr) {
            for item in items {
                if let Some(name) = item.get("name").and_then(mini_json::Json::as_str) {
                    if self.config.rules.iter().any(|r| r.name.eq_ignore_ascii_case(name)) { continue; }
                    let members = item.get("members").and_then(mini_json::Json::as_arr)
                        .map(|a| a.iter().filter_map(|i| i.as_str().map(str::to_string)).collect())
                        .unwrap_or_default();
                    let rule = Rule {
                        name: name.to_string(),
                        members,
                        path: item.get("path").and_then(mini_json::Json::as_str).unwrap_or("").to_string(),
                        match_path: item.get("match_path").and_then(mini_json::Json::as_bool).unwrap_or(false),
                        enabled: item.get("enabled").and_then(mini_json::Json::as_bool).unwrap_or(true),
                        restore_priority: item.get("restore_priority").and_then(mini_json::Json::as_f64)
                            .map(|n| n as u32).unwrap_or(w::NORMAL_PRIORITY_CLASS),
                        note: item.get("note").and_then(mini_json::Json::as_str).unwrap_or("").to_string(),
                    };
                    self.config.rules.push(rule);
                }
            }
        }
        self.save_rules();
        self.last_message = format!("已从 {} 合并规则。", path.display());
    }

    fn cmd_export(&mut self) {
        let Some(path) = (unsafe { file_dialog(self.hwnd, true, "导出全部规则",
            "JSON\0*.json\0所有文件\0*.*\0\0", "EcoPilot-规则.json") }) else { return };
        let items: Vec<mini_json::Json> = self.config.rules.iter().map(|r| {
            let mut o = mini_json::Json::Obj(Vec::new());
            o.set("name", mini_json::Json::Str(r.name.clone()));
            let mem: Vec<mini_json::Json> = r.members.iter().map(|m| mini_json::Json::Str(m.clone())).collect();
            o.set("members", mini_json::Json::Arr(mem));
            o.set("path", mini_json::Json::Str(r.path.clone()));
            o.set("match_path", mini_json::Json::Bool(r.match_path));
            o.set("enabled", mini_json::Json::Bool(r.enabled));
            o.set("restore_priority", mini_json::Json::Num(r.restore_priority as f64));
            o.set("note", mini_json::Json::Str(r.note.clone()));
            o
        }).collect();
        let mut root = mini_json::Json::Obj(Vec::new());
        root.set("rules", mini_json::Json::Arr(items));
        if let Err(e) = std::fs::write(&path, root.stringify()) {
            self.message(&format!("导出失败：{}", e), "错误", w::MB_ICONERROR);
        } else {
            self.last_message = format!("已导出 {} 条规则到 {}", self.config.rules.len(), path.display());
        }
        self.update_status();
    }

    /// 用资源管理器打开启动文件夹（explorer 会展开环境变量）
    fn cmd_open_startup(&mut self, all_users: bool) {
        let path = if all_users {
            r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Startup".to_string()
        } else {
            // %APPDATA% 用环境变量展开，兼容任意用户名
            std::env::var("APPDATA").map(|p| format!(r"{}\Microsoft\Windows\Start Menu\Programs\Startup", p))
                .unwrap_or_else(|_| r"C:\Users\你的用户名\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup".to_string())
        };
        self.last_message = format!("正在打开启动文件夹：{}", path);
        self.update_status();
        // ShellExecuteW 是同步的：要等 explorer 通过 DDE 应答才返回，
        // 连续点击时 explorer 正忙于前一个文件夹窗口，第二次调用会卡住整个 UI 线程。
        // 放到后台线程执行，主线程立即返回；用静态互斥串行化，避免竞态。
        std::thread::spawn(move || {
            static OPEN_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
            let _guard = match OPEN_LOCK.lock() { Ok(g) => g, Err(e) => e.into_inner() };
            let file = w::to_wide(&path);
            let op = w::to_wide("open");
            let rc = unsafe {
                w::ShellExecuteW(std::ptr::null_mut(), op.as_ptr(), file.as_ptr(),
                    std::ptr::null(), std::ptr::null(), 5) // SW_SHOW
            } as usize;
            if rc <= 32 {
                eprintln!("打开启动文件夹失败：{}（错误码 {}）", path, rc);
            }
        });
    }

    /// 生成开机启动脚本（bat + 快捷方式），并弹出完整复制说明
    fn cmd_autostart(&mut self) {
        let bat = eco::generate_startup_script();
        let lnk = eco::generate_startup_shortcut();
        match (bat, lnk) {
            (Ok(bat_path), Ok(lnk_path)) => {
                let exe_name = eco::exe_path().file_name()
                    .and_then(|s| s.to_str()).unwrap_or("EcoPilot.exe").to_string();
                let stem = eco::startup_file_stem();
                self.last_message = format!("已生成启动脚本与快捷方式（指向 {}）", exe_name);
                self.message(&format!(concat!(
                    "已在程序所在文件夹生成两个文件（均指向真实程序 {}，带 --apply --tray 参数）：\n",
                    "• {}.bat\n",
                    "• {}.lnk（快捷方式）\n\n",
                    "请任选其一复制到系统启动文件夹：\n",
                    "• 所有用户：C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\n",
                    "• 当前用户：C:\\Users\\你的用户名\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\n\n",
                    "也可按 Win+R 输入 shell:startup 快速打开当前用户的启动文件夹。\n\n",
                    "复制到启动文件夹后，下次登录会自动以静默模式应用规则。"
                ), exe_name, stem, stem), "生成开机启动脚本", w::MB_ICONINFORMATION);
            }
            (Err(msg), _) | (_, Err(msg)) => {
                self.message(&msg, "生成失败", w::MB_ICONERROR);
            }
        }
        self.update_status();
    }

    fn cmd_settings(&mut self) {
        let mut state = SettingsState { result: None, config: self.config.clone() };
        unsafe { show_settings_dialog(self.hwnd, &mut state); }
        if let Some(result) = state.result.take() {
            if result.show_tray_icon != self.config.show_tray_icon {
                self.remove_tray(); self.config.show_tray_icon = result.show_tray_icon;
                if self.config.show_tray_icon { self.setup_tray(); }
            }
            self.config.start_to_tray = result.start_to_tray;
            self.config.auto_apply = result.auto_apply;
            self.config.auto_apply_rules = result.auto_apply_rules;
            self.config.exit_after_apply = result.exit_after_apply;
            // 自动刷新开关：按可见性启停定时器
            self.auto_refresh = result.auto_refresh;
            self.config.auto_refresh = result.auto_refresh;
            self.sync_timer();
            let rf = if self.auto_refresh { w::MF_CHECKED } else { w::MF_UNCHECKED };
            unsafe {
                w::CheckMenuItem(self.hmenu, IDM_AUTO_REFRESH, w::MF_BYCOMMAND | rf);
                let btn = if self.auto_refresh { "自动刷新：开" } else { "自动刷新：关" };
                w::SetWindowTextW(self.control(ID_BTN_AUTO_REFRESH), w::to_wide(btn).as_ptr());
            }
            if let Err(msg) = eco::save_config(&self.config_path, &self.config) {
                self.message(&msg, "保存设置失败", w::MB_ICONERROR);
            }
            self.last_message = "设置已保存到程序目录。".to_string();
            self.update_status();
            // 若同时勾了"自动应用"和"成功后退出"，点确定后直接退出。
            // 关键：绝不能在这里调用 apply_rules —— 用户可能刚点过"移除所有效能"，
            // 此处再应用会把已恢复的进程重新效能化。规则只在下次启动时应用。
            if self.config.exit_after_apply && self.config.auto_apply {
                unsafe { w::PostMessageW(self.hwnd, WM_APP_EXIT, 0, 0); }
            }
        }
    }

    fn message(&self, text: &str, caption: &str, flags: u32) {
        unsafe { w::MessageBoxW(self.hwnd, w::to_wide(text).as_ptr(), w::to_wide(caption).as_ptr(), flags); }
    }

    /// 定时器只在"窗口可见 且 用户开启自动刷新"时运行；隐藏/托盘时彻底停掉 → 零 CPU
    fn sync_timer(&mut self) {
        let should_run = self.auto_refresh && unsafe { w::IsWindowVisible(self.hwnd) } != 0;
        unsafe {
            if should_run {
                w::SetTimer(self.hwnd, 1, self.config.refresh_seconds.max(1) * 1000, std::ptr::null_mut());
            } else {
                w::KillTimer(self.hwnd, 1);
            }
        }
    }

    fn show(&mut self) {
        unsafe { w::ShowWindow(self.hwnd, w::SW_SHOW); w::SetForegroundWindow(self.hwnd); }
        self.sync_timer(); // 界面显示 → 恢复刷新
    }

    fn on_destroy(&mut self) {
        unsafe {
            w::KillTimer(self.hwnd, 1); self.remove_tray();
            if !self.hfont.is_null() { w::DeleteObject(self.hfont); }
            if !self.hfont_btn.is_null() { w::DeleteObject(self.hfont_btn); }
            if !self.hmenu.is_null() { w::DestroyMenu(self.hmenu); }
            if !self.himages.is_null() { w::ImageList_Destroy(self.himages); }
        }
    }
}

/// 效能模式排序档位：已效能 > 保护 > 普通，保证同类聚在一起。
fn mode_rank(p: &ProcessInfo, config: &Config) -> u8 {
    if p.is_eco() { 2 } else if eco::is_protected(&p.name, config) { 1 } else { 0 }
}

/// 取规则树当前选中节点对应的规则索引
fn rule_tree_selection(app: &App) -> Option<usize> {
    let caret = unsafe { w::SendMessageW(app.hrule_tree, w::TVM_GETNEXTITEM, w::TVGN_CARET as usize, 0) } as isize;
    if caret == 0 { return None; }
    app.rule_nodes.iter().find(|(h, _)| *h == caret).map(|(_, i)| *i)
}

/// 从可执行文件路径提取小图标，加入 image list 并缓存；路径为空用默认 exe 图标。
fn get_or_add_icon(app: &mut App, path: &str) -> i32 {
    let key = if path.is_empty() { "<default>".to_string() } else { path.to_lowercase() };
    if let Some(&idx) = app.icon_cache.get(&key) { return idx; }
    let mut info: w::SHFILEINFOW = unsafe { std::mem::zeroed() };
    let path_wide = if path.is_empty() {
        w::to_wide("C:\\Windows\\System32\\svchost.exe")
    } else {
        w::to_wide(path)
    };
    let flags = w::SHGFI_ICON | w::SHGFI_SMALLICON;
    let ok = unsafe {
        w::SHGetFileInfoW(path_wide.as_ptr(), 0, &mut info,
            std::mem::size_of::<w::SHFILEINFOW>() as u32, flags)
    };
    let idx = if ok != 0 && !info.hIcon.is_null() {
        let i = unsafe { w::ImageList_AddIcon(app.himages, info.hIcon) };
        unsafe { w::DestroyIcon(info.hIcon); }
        i
    } else {
        app.default_icon
    };
    app.icon_cache.insert(key, idx);
    idx
}

fn matching_rule_local(rule: &Rule, p: &ProcessInfo) -> bool {
    if rule.match_path && !rule.path.is_empty() {
        !p.path.is_empty() && p.path.eq_ignore_ascii_case(&rule.path)
    } else { p.name.eq_ignore_ascii_case(&rule.name) }
}

fn selected_pids(list: w::HWND) -> Vec<isize> {
    let mut result = Vec::new();
    if list.is_null() { return result; }
    let mut index: i32 = -1;
    loop {
        index = unsafe { w::SendMessageW(list, w::LVM_GETNEXTITEM, index as usize, w::LVNI_SELECTED as isize) as i32 };
        if index < 0 { break; }
        let mut item = w::LVITEMW::zeroed();
        item.mask = w::LVIF_PARAM; item.iItem = index;
        unsafe { w::SendMessageW(list, w::LVM_GETITEMW, 0, &mut item as *mut _ as isize); }
        result.push(item.lParam);
        if result.len() > 10000 { break; }
    }
    result
}

unsafe fn self_instance() -> w::HINSTANCE { w::GetModuleHandleW(std::ptr::null()) }

unsafe extern "system" fn wndproc(hwnd: w::HWND, msg: u32, wparam: w::WPARAM, lparam: w::LPARAM) -> w::LRESULT {
    if msg == w::WM_CREATE {
        let create = &*(lparam as *const CREATESTRUCTW);
        w::SetWindowLongPtrW(hwnd, w::GWLP_USERDATA, create.lpCreateParams as isize);
        let app = &mut *(create.lpCreateParams as *mut App);
        app.hwnd = hwnd; app.on_create();
        if app.start_to_tray { w::ShowWindow(hwnd, w::SW_HIDE); }
        app.sync_timer(); // 隐藏状态不启动定时器
        return 0;
    }
    let ptr = w::GetWindowLongPtrW(hwnd, w::GWLP_USERDATA);
    if ptr == 0 { return w::DefWindowProcW(hwnd, msg, wparam, lparam); }
    let app = &mut *(ptr as *mut App);

    match msg {
        w::WM_SIZE => { app.layout(); 0 }
        w::WM_TIMER => {
            if w::IsWindowVisible(hwnd) != 0 && app.auto_refresh { app.refresh_processes(true, false); }
            0
        }
        WM_APP_APPLY => { if let Some(r) = app.initial_report.take() { app.report_apply(&r); } 0 }
        WM_APP_EXIT => { w::DestroyWindow(hwnd); 0 }
        w::WM_COMMAND => {
            let id = (wparam & 0xFFFF) as u32;
            let code = (wparam >> 16) as u32;
            if code == w::EN_CHANGE && id == ID_SEARCH { app.refresh_processes(false, true); return 0; }
            match id {
                ID_BTN_REFRESH | IDM_REFRESH => app.refresh_processes(true, false),
                ID_BTN_ECO_ON => app.cmd_set_efficiency(true),
                ID_BTN_ECO_OFF => app.cmd_set_efficiency(false),
                ID_BTN_ADD_RULE => app.cmd_add_rule(),
                ID_BTN_APPLY | IDM_APPLY => app.cmd_apply_rules(),
                ID_BTN_RESTORE_ALL | IDM_RESTORE_ALL => app.cmd_restore_all(),
                ID_BTN_AUTOSTART => app.cmd_autostart(),
                ID_BTN_OPEN_STARTUP_ALL => app.cmd_open_startup(true),
                ID_BTN_OPEN_STARTUP_USER => app.cmd_open_startup(false),
                ID_BTN_SETTINGS | IDM_SETTINGS => app.cmd_settings(),
                ID_BTN_EXIT => { w::DestroyWindow(hwnd); }
                ID_BTN_NEW_RULE => app.cmd_new_rule(),
                ID_BTN_RENAME_RULE | 2016 => {
                    if let Some(i) = app.rule_menu_index.take().or_else(|| rule_tree_selection(&app)) {
                        app.cmd_rename_rule(i);
                    }
                }
                IDM_IMPORT => app.cmd_import(),
                IDM_EXPORT => app.cmd_export(),
                IDM_PROT_ADD => app.cmd_protect(true),
                IDM_PROT_REMOVE => app.cmd_protect(false),
                IDM_REMOVE_FROM_RULE => app.cmd_remove_from_rule(),
                ID_BTN_AUTO_REFRESH | IDM_AUTO_REFRESH => {
                    app.auto_refresh = !app.auto_refresh;
                    app.config.auto_refresh = app.auto_refresh;
                    let _ = eco::save_config(&app.config_path, &app.config);
                    app.sync_timer(); // 隐藏时即便开启也不启动定时器
                    let f = if app.auto_refresh { w::MF_CHECKED } else { w::MF_UNCHECKED };
                    w::CheckMenuItem(app.hmenu, IDM_AUTO_REFRESH, w::MF_BYCOMMAND | f);
                    let text = if app.auto_refresh { "自动刷新：开" } else { "自动刷新：关" };
                    w::SetWindowTextW(app.control(ID_BTN_AUTO_REFRESH), w::to_wide(text).as_ptr());
                }
                IDM_EXIT | IDM_TRAY_EXIT => { w::DestroyWindow(hwnd); }
                IDM_ABOUT => unsafe { show_about_dialog(hwnd); },
                _ => {}
            }
            0
        }
        w::WM_NOTIFY => {
            let header = &*(lparam as *const w::NMHDR);
            if header.code == w::NM_DBLCLK && header.idFrom == ID_PROC_LIST as usize {
                let info = &*(lparam as *const w::NMITEMACTIVATE);
                app.cmd_toggle_process(info.iItem);
            } else if header.code == w::LVN_COLUMNCLICK && header.idFrom == ID_PROC_LIST as usize {
                let info = &*(lparam as *const w::NMLISTVIEW);
                let col = info.iSubItem.max(0) as usize;
                if col == app.sort_column { app.sort_ascending = !app.sort_ascending; }
                else { app.sort_column = col; app.sort_ascending = true; }
                app.refresh_processes(true, false);
            } else if header.code == w::NM_CLICK && (header.idFrom == ID_ECO_TREE as usize || header.idFrom == ID_RULE_TREE as usize) {
                let tree = if header.idFrom == ID_RULE_TREE as usize { app.hrule_tree } else { app.heco_tree };
                // TreeView 的 NM_CLICK 带 NMTREEVIEW，ptDrag 就是客户区坐标，最可靠；
                // GetMessagePos 作回退，避免不同通知结构差异。
                let nt = &*(lparam as *const w::NMTREEVIEWW);
                let pos = w::GetMessagePos();
                let mut hit: w::TVHITTESTINFO = std::mem::zeroed();
                hit.pt = if nt.ptDrag.x != 0 || nt.ptDrag.y != 0 {
                    nt.ptDrag
                } else {
                    let mut p = w::POINT { x: (pos & 0xFFFF) as i16 as i32, y: (pos >> 16) as i16 as i32 };
                    w::ScreenToClient(tree, &mut p);
                    p
                };
                w::SendMessageW(tree, w::TVM_HITTEST, 0, &mut hit as *mut _ as isize);
                let mut caret = hit.hItem;
                let valid_hit = caret != 0 && (hit.flags & (w::TVHT_ONITEMLABEL | w::TVHT_ONITEMBUTTON)) != 0;
                if !valid_hit {
                    // 回退：TreeView 点击节点会自动设为当前项；点空白则 caret 为 0
                    caret = w::SendMessageW(tree, w::TVM_GETNEXTITEM, w::TVGN_CARET as usize, 0) as isize;
                }
                if caret != 0 && (valid_hit || hit.flags == 0) {
                    let mut item = w::TVITEMW::zeroed();
                    item.mask = w::TVIF_STATE | w::TVIF_PARAM;
                    item.stateMask = w::TVIS_EXPANDED;
                    item.hItem = caret;
                    w::SendMessageW(tree, w::TVM_GETITEMW, 0, &mut item as *mut _ as isize);
                    let on_button = hit.flags & w::TVHT_ONITEMBUTTON != 0;
                    if item.lParam >= 0x5000_0000 {
                        // 根节点：点 +/- 按钮交给树自身处理；点标签则手动切换（单击即生效）
                        let mut expanded = item.state & w::TVIS_EXPANDED != 0;
                        if !on_button {
                            let action = if expanded { w::TVE_COLLAPSE } else { w::TVE_EXPAND };
                            w::SendMessageW(tree, w::TVM_EXPAND, action as usize, caret);
                            expanded = !expanded;
                        } else {
                            // 树已切换，稍后回读真实状态
                            let mut after = w::TVITEMW::zeroed();
                            after.mask = w::TVIF_STATE; after.stateMask = w::TVIS_EXPANDED; after.hItem = caret;
                            w::SendMessageW(tree, w::TVM_GETITEMW, 0, &mut after as *mut _ as isize);
                            expanded = after.state & w::TVIS_EXPANDED != 0;
                        }
                        // 记录展开状态，自动刷新重建后恢复
                        if header.idFrom == ID_RULE_TREE as usize {
                            let idx = (item.lParam & 0x0FFF_FFFF) as usize;
                            if expanded { app.expanded_rules.insert(idx); } else { app.expanded_rules.remove(&idx); }
                        } else {
                            app.eco_tree_expanded = expanded;
                        }
                    } else if item.lParam > 0 {
                        // 叶子进程节点：只在左侧选中，绝不刷新列表（避免闪烁）
                        let pid = item.lParam as u32;
                        if let Some(row) = app.visible_rows.iter().position(|i| app.procs[*i].pid == pid) {
                            app.select_row(app.hproc, row as i32);
                        }
                    }
                }
            } else if header.code == w::NM_RCLICK {
                let from_proc = header.idFrom == ID_PROC_LIST as usize;
                let from_rule_tree = header.idFrom == ID_RULE_TREE as usize;
                let from_eco_tree = header.idFrom == ID_ECO_TREE as usize;
                let info = &*(lparam as *const w::NMITEMACTIVATE);
                // 右上效能树右键：命中测试取该进程的 PID
                if from_eco_tree {
                    app.eco_menu_pid = None;
                    let pos = w::GetMessagePos();
                    let mut hit: w::TVHITTESTINFO = std::mem::zeroed();
                    hit.pt = w::POINT { x: (pos & 0xFFFF) as i16 as i32, y: (pos >> 16) as i16 as i32 };
                    w::ScreenToClient(app.heco_tree, &mut hit.pt);
                    w::SendMessageW(app.heco_tree, w::TVM_HITTEST, 0, &mut hit as *mut _ as isize);
                    // 命中测试失败时回退到当前选中项，保证右键总能定位到节点
                    let mut target = hit.hItem;
                    if target == 0 || (hit.flags & w::TVHT_ONITEM) == 0 {
                        target = w::SendMessageW(app.heco_tree, w::TVM_GETNEXTITEM, w::TVGN_CARET as usize, 0) as isize;
                    }
                    if target != 0 {
                        let mut item = w::TVITEMW::zeroed();
                        item.mask = w::TVIF_PARAM;
                        item.hItem = target;
                        w::SendMessageW(app.heco_tree, w::TVM_GETITEMW, 0, &mut item as *mut _ as isize);
                        if item.lParam > 0 && item.lParam < 0x5000_0000 {
                            app.eco_menu_pid = Some(item.lParam as u32);
                        }
                    }
                }
                if from_proc && info.iItem >= 0 {
                    let mut clear = w::LVITEMW::zeroed();
                    clear.state = 0; clear.stateMask = LVIS_SELECTED;
                    w::SendMessageW(app.hproc, LVM_SETITEMSTATE, usize::MAX, &clear as *const _ as isize);
                    app.select_row(app.hproc, info.iItem);
                }
                // 规则树右键：命中测试确定点的是规则根节点还是成员进程节点
                if from_rule_tree {
                    app.rule_menu_index = None;
                    app.rule_menu_pid = None;
                    let pos = w::GetMessagePos();
                    let mut hit: w::TVHITTESTINFO = std::mem::zeroed();
                    hit.pt = w::POINT { x: (pos & 0xFFFF) as i16 as i32, y: (pos >> 16) as i16 as i32 };
                    w::ScreenToClient(app.hrule_tree, &mut hit.pt);
                    w::SendMessageW(app.hrule_tree, w::TVM_HITTEST, 0, &mut hit as *mut _ as isize);
                    let mut target = hit.hItem;
                    if target == 0 || (hit.flags & w::TVHT_ONITEM) == 0 {
                        target = w::SendMessageW(app.hrule_tree, w::TVM_GETNEXTITEM, w::TVGN_CARET as usize, 0) as isize;
                    }
                    if target != 0 {
                        if let Some((_, idx)) = app.rule_nodes.iter().find(|(h, _)| *h == target) {
                            app.rule_menu_index = Some(*idx);
                        } else if let Some((_, pid, idx, name)) = app.rule_proc_nodes.iter().find(|(h, _, _, _)| *h == target) {
                            // 点在成员进程节点上：记录 PID、名字与其所属规则
                            app.rule_menu_pid = Some(*pid);
                            app.rule_menu_name = Some(name.clone());
                            app.rule_menu_index = Some(*idx);
                        } else {
                            // 点在空白处：若当前选中的是规则根，仍允许新建规则
                            app.rule_menu_index = rule_tree_selection(app);
                        }
                    }
                }
                let menu = w::CreatePopupMenu();
                if from_proc {
                    w::AppendMenuW(menu, 0, ID_BTN_ECO_ON as usize, w::to_wide("设为效能模式").as_ptr());
                    w::AppendMenuW(menu, 0, ID_BTN_ECO_OFF as usize, w::to_wide("移除效能模式").as_ptr());
                    w::AppendMenuW(menu, 0, ID_BTN_ADD_RULE as usize, w::to_wide("加入规则").as_ptr());
                    w::AppendMenuW(menu, 0, IDM_REMOVE_FROM_RULE as usize, w::to_wide("移出本规则").as_ptr());
                    w::AppendMenuW(menu, 0x0008, 0, std::ptr::null());
                    w::AppendMenuW(menu, 0, IDM_PROT_ADD as usize, w::to_wide("加入保护名单").as_ptr());
                    w::AppendMenuW(menu, 0, IDM_PROT_REMOVE as usize, w::to_wide("移出保护名单").as_ptr());
                } else if from_rule_tree {
                    w::AppendMenuW(menu, 0, 2015usize, w::to_wide("新建规则...").as_ptr());
                    if app.rule_menu_index.is_some() {
                        w::AppendMenuW(menu, 0, 2016usize, w::to_wide("重命名规则...").as_ptr());
                    }
                    w::AppendMenuW(menu, 0x0008, 0, std::ptr::null());
                    w::AppendMenuW(menu, 0, IDM_DELETE_RULE as usize, w::to_wide("删除此规则").as_ptr());
                    w::AppendMenuW(menu, 0, IDM_REMOVE_FROM_RULE as usize, w::to_wide("移出本规则").as_ptr());
                } else if from_eco_tree {
                    if app.eco_menu_pid.is_some() {
                        w::AppendMenuW(menu, 0, IDM_RESTORE_PID as usize, w::to_wide("移除效能模式").as_ptr());
                    }
                    w::AppendMenuW(menu, 0, ID_BTN_RESTORE_ALL as usize, w::to_wide("移除所有进程的效能模式").as_ptr());
                    w::AppendMenuW(menu, 0x0008, 0, std::ptr::null());
                    w::AppendMenuW(menu, 0, ID_BTN_SETTINGS as usize, w::to_wide("设置").as_ptr());
                } else {
                    w::AppendMenuW(menu, 0, ID_BTN_SETTINGS as usize, w::to_wide("设置").as_ptr());
                }
                let mut point: w::POINT = std::mem::zeroed();
                w::GetCursorPosCompat(&mut point);
                w::SetForegroundWindow(hwnd);
                let command = w::TrackPopupMenu(menu,
                    w::TPM_LEFTALIGN | w::TPM_RETURNCMD | w::TPM_RIGHTBUTTON,
                    point.x, point.y, 0, hwnd, std::ptr::null());
                w::DestroyMenu(menu);
                match command as u32 {
                    ID_BTN_ECO_ON => app.cmd_set_efficiency(true),
                    ID_BTN_ECO_OFF => app.cmd_set_efficiency(false),
                    ID_BTN_ADD_RULE => app.cmd_add_rule(),
                    IDM_REMOVE_FROM_RULE => app.cmd_remove_from_rule(),
                    2015 => app.cmd_new_rule(),
                    2016 => { if let Some(i) = app.rule_menu_index.take() { app.cmd_rename_rule(i); } }
                    IDM_DELETE_RULE => { if let Some(i) = app.rule_menu_index.take() { app.cmd_delete_rule(i); } }
                    IDM_RESTORE_PID => { if let Some(pid) = app.eco_menu_pid.take() { app.cmd_restore_pid(pid); } }
                    ID_BTN_RESTORE_ALL | IDM_RESTORE_ALL => app.cmd_restore_all(),
                    IDM_PROT_ADD => app.cmd_protect(true),
                    IDM_PROT_REMOVE => app.cmd_protect(false),
                    ID_BTN_SETTINGS => app.cmd_settings(),
                    _ => {}
                }
            }
            0
        }
        w::WM_APP_TRAY => {
            match lparam as u32 {
                0x0203 => app.show(),
                0x0205 => {
                    let menu = w::CreatePopupMenu();
                    w::AppendMenuW(menu, 0, IDM_TRAY_SHOW as usize, w::to_wide("显示主窗口").as_ptr());
                    w::AppendMenuW(menu, 0, IDM_TRAY_APPLY as usize, w::to_wide("立即应用规则").as_ptr());
                    w::AppendMenuW(menu, 0, IDM_TRAY_RESTORE as usize, w::to_wide("移除所有进程的效能模式").as_ptr());
                    w::AppendMenuW(menu, 0x0008, 0, std::ptr::null());
                    w::AppendMenuW(menu, 0, IDM_TRAY_EXIT as usize, w::to_wide("退出").as_ptr());
                    let mut point: w::POINT = std::mem::zeroed();
                    w::GetCursorPosCompat(&mut point);
                    w::SetForegroundWindow(hwnd);
                    let command = w::TrackPopupMenu(menu,
                        w::TPM_LEFTALIGN | w::TPM_RETURNCMD | w::TPM_RIGHTBUTTON,
                        point.x, point.y, 0, hwnd, std::ptr::null());
                    w::DestroyMenu(menu);
                    match command as u32 {
                        IDM_TRAY_SHOW => app.show(),
                        IDM_TRAY_APPLY => app.cmd_apply_rules(),
                        IDM_TRAY_RESTORE => app.cmd_restore_all(),
                        IDM_TRAY_EXIT => { w::DestroyWindow(hwnd); }
                        _ => {}
                    }
                }
                _ => {}
            }
            0
        }
        w::WM_CLOSE => {
            // 关闭按钮缩到托盘；托盘菜单里可真正退出
            if app.config.show_tray_icon {
                w::ShowWindow(hwnd, w::SW_HIDE);
                app.sync_timer(); // 缩到托盘 → 停掉定时器，零 CPU
            } else { w::DestroyWindow(hwnd); }
            0
        }
        w::WM_DESTROY => { app.on_destroy(); w::PostQuitMessage(0); 0 }
        _ => w::DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

/// 比系统默认 GUI 字体大一号（用于设置/关于对话框，字体存静态变量持有到进程退出）
fn big_font() -> w::HFONT {
    use std::sync::OnceLock;
    static FONT: OnceLock<usize> = OnceLock::new();
    let h = FONT.get_or_init(|| {
        // 默认 GUI 字体高度 +3px，Segoe UI 在中文系统会自动回退到微软雅黑
        let face = w::to_wide("Segoe UI");
        unsafe {
            w::CreateFontW(-18, 0, 0, 0, 400, 0, 0, 0, 1,
                0, 0, 0, 0, face.as_ptr()) as usize
        }
    });
    *h as w::HFONT
}

unsafe extern "system" fn about_wndproc(hwnd: w::HWND, msg: u32, wparam: w::WPARAM, lparam: w::LPARAM) -> w::LRESULT {
    if msg == w::WM_CREATE {
        let font = big_font();
        // 用 AUTOVSCROLL + MULTILINE（不加 ES_AUTOHSCROLL），长行自动换行而非横向截断
        let text = w::CreateWindowExW(0, w::to_wide("Edit").as_ptr(), w::to_wide(ABOUT_TEXT).as_ptr(),
            w::WS_CHILD | w::WS_VISIBLE | w::WS_VSCROLL | w::ES_MULTILINE | w::ES_READONLY
                | w::ES_AUTOVSCROLL | w::WS_BORDER,
            16, 16, 640, 510, hwnd, ID_ABOUT_TEXT as usize as w::HMENU,
            self_instance(), std::ptr::null_mut());
        w::SendMessageW(text, w::WM_SETFONT, font as usize, 1);
        let ok = w::CreateWindowExW(0, w::to_wide("Button").as_ptr(), w::to_wide("确定").as_ptr(),
            w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP | w::BS_DEFPUSHBUTTON,
            290, 536, 100, 32, hwnd, ID_ABOUT_OK as usize as w::HMENU,
            self_instance(), std::ptr::null_mut());
        w::SendMessageW(ok, w::WM_SETFONT, font as usize, 1);
        return 0;
    }
    match msg {
        w::WM_COMMAND => {
            if (wparam & 0xFFFF) as u32 == ID_ABOUT_OK { w::DestroyWindow(hwnd); }
            0
        }
        w::WM_CLOSE => { w::DestroyWindow(hwnd); 0 }
        _ => w::DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn show_about_dialog(owner: w::HWND) {
    let class = w::to_wide(ABOUT_CLASS);
    let hicon = w::LoadIconW(std::ptr::null_mut(), IDI_APPLICATION as usize as *const u16);
    let wc = w::WNDCLASSEXW {
        cbSize: std::mem::size_of::<w::WNDCLASSEXW>() as u32, style: 0,
        lpfnWndProc: about_wndproc, cbClsExtra: 0, cbWndExtra: 0,
        hInstance: self_instance(), hIcon: hicon,
        hCursor: w::LoadCursorW(std::ptr::null_mut(), 32512usize as *const u16),
        hbrBackground: w::GetStockObject(0) as w::HBRUSH,
        lpszMenuName: std::ptr::null(), lpszClassName: class.as_ptr(), hIconSm: hicon,
    };
    let atom = w::RegisterClassExW(&wc);
    if atom == 0 && w::GetLastError() != 1410 { return; }
    let (cx, cy) = centered_pos(owner, 680, 620);
    let hwnd = w::CreateWindowExW(0, class.as_ptr(), w::to_wide(APP_TITLE).as_ptr(),
        w::WS_OVERLAPPEDWINDOW & !0x0001_0000 & !0x0000_0400,
        cx, cy, 680, 620, std::ptr::null_mut(), std::ptr::null_mut(),
        self_instance(), std::ptr::null_mut());
    if hwnd.is_null() { return; }
    w::EnableWindow(owner, 0);
    w::ShowWindow(hwnd, w::SW_SHOWNORMAL);
    w::UpdateWindow(hwnd);
    let mut msg: w::MSG = std::mem::zeroed();
    while w::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
        w::TranslateMessage(&msg); w::DispatchMessageW(&msg);
        if w::IsWindow(hwnd) == 0 { break; }
    }
    w::EnableWindow(owner, 1);
    w::SetForegroundWindow(owner);
}

// ---------------- 设置对话框 ----------------

unsafe extern "system" fn settings_wndproc(hwnd: w::HWND, msg: u32, wparam: w::WPARAM, lparam: w::LPARAM) -> w::LRESULT {
    if msg == w::WM_CREATE {
        let create = &*(lparam as *const CREATESTRUCTW);
        w::SetWindowLongPtrW(hwnd, w::GWLP_USERDATA, create.lpCreateParams as isize);
        let state = &mut *(create.lpCreateParams as *mut SettingsState);
        let font = big_font();
        let mk = |id: u32, class: &str, text: &str, style: u32| -> w::HWND {
            let h = w::CreateWindowExW(0, w::to_wide(class).as_ptr(), w::to_wide(text).as_ptr(),
                style, 0, 0, 10, 10, hwnd, id as usize as w::HMENU,
                self_instance(), std::ptr::null_mut());
            if !h.is_null() { w::SendMessageW(h, w::WM_SETFONT, font as usize, 1); }
            h
        };
        let cs = w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP | w::BS_AUTOCHECKBOX;
        mk(w::ID_CHK_AUTO_REFRESH, "Button", "定时自动刷新进程列表（开启会有周期性 CPU 采样；关闭后空闲零占用）", cs);
        mk(w::ID_CHK_START_TRAY, "Button", "启动时缩到托盘", cs);
        mk(w::ID_CHK_TRAY_ICON, "Button", "在托盘显示图标", cs);
        mk(w::ID_CHK_AUTO_APPLY, "Button", "启动/进入时自动应用规则", cs);
        mk(4001, "Static", "启动时应用哪些规则（勾选；全不勾 = 应用全部启用规则）：", 0);
        let list = mk(w::ID_RULES_CHECK, "SysListView32", "",
            w::WS_CHILD | w::WS_VISIBLE | w::WS_BORDER | w::WS_VSCROLL | w::WS_TABSTOP | w::LVS_REPORT | w::LVS_SHOWSELALWAYS);
        let ex = w::LVS_EX_CHECKBOXES | w::LVS_EX_FULLROWSELECT | w::LVS_EX_DOUBLEBUFFER;
        w::SendMessageW(list, w::LVM_SETEXTENDEDLISTVIEWSTYLE, 0, ex as isize);
        let mut title = w::to_wide("规则名称");
        let mut col = w::LVCOLUMNW::zeroed();
        col.mask = w::LVCF_FMT | w::LVCF_WIDTH | w::LVCF_TEXT | w::LVCF_SUBITEM;
        col.fmt = w::LVCFMT_LEFT; col.cx = 420; col.pszText = title.as_mut_ptr(); col.iSubItem = 0;
        w::SendMessageW(list, w::LVM_INSERTCOLUMNW, 0, &col as *const _ as isize);
        for (i, rule) in state.config.rules.iter().enumerate() {
            let mut text = w::to_wide(&rule.display_name());
            let mut item = w::LVITEMW::zeroed();
            item.mask = w::LVIF_TEXT | w::LVIF_PARAM; item.iItem = i as i32;
            item.pszText = text.as_mut_ptr(); item.lParam = i as isize;
            w::SendMessageW(list, w::LVM_INSERTITEMW, 0, &item as *const _ as isize);
            let checked = state.config.auto_apply_rules.is_empty()
                || state.config.auto_apply_rules.iter().any(|n| n.eq_ignore_ascii_case(&rule.name));
            let mut st = w::LVITEMW::zeroed();
            st.state = if checked { 2 << 12 } else { 1 << 12 };
            st.stateMask = w::LVIS_STATEIMAGEMASK;
            w::SendMessageW(list, LVM_SETITEMSTATE, i, &st as *const _ as isize);
        }
        // 长说明：用 BS_MULTILINE 让文字换行显示
        mk(w::ID_CHK_EXIT_AFTER_APPLY, "Button",
            "自动应用规则成功后自动退出本程序（静默后台模式）；启动本选项后，就看不到程序界面了，想改回来，就用记事本打开程序所在目录里面的“settings.json”，将 \"exit_after_apply\": true 这句里面的 true 改成 false",
            cs | 0x0000_2000); // BS_MULTILINE
        w::CheckDlgButtonCompat(hwnd, w::ID_CHK_AUTO_REFRESH, state.config.auto_refresh);
        w::CheckDlgButtonCompat(hwnd, w::ID_CHK_START_TRAY, state.config.start_to_tray);
        w::CheckDlgButtonCompat(hwnd, w::ID_CHK_TRAY_ICON, state.config.show_tray_icon);
        w::CheckDlgButtonCompat(hwnd, w::ID_CHK_AUTO_APPLY, state.config.auto_apply);
        w::CheckDlgButtonCompat(hwnd, w::ID_CHK_EXIT_AFTER_APPLY, state.config.exit_after_apply);
        mk(w::ID_BTN_SETTINGS_OK, "Button", "确定", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP);
        mk(w::ID_BTN_SETTINGS_CANCEL, "Button", "取消", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP);
        // 行高30、步进40，留10px 行间距，避免大字体下相邻行文字挤在一起
        w::MoveWindow(w::GetDlgItemCompat(hwnd, w::ID_CHK_START_TRAY), 20, 20, 620, 30, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, w::ID_CHK_TRAY_ICON), 20, 60, 620, 30, 1);
        // 自动刷新在前；自动应用其后，子选项"自动退出"缩进紧跟
        w::MoveWindow(w::GetDlgItemCompat(hwnd, w::ID_CHK_AUTO_REFRESH), 20, 100, 620, 30, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, w::ID_CHK_AUTO_APPLY), 20, 140, 620, 30, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, w::ID_CHK_EXIT_AFTER_APPLY), 38, 180, 600, 120, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, 4001), 20, 308, 620, 26, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, w::ID_RULES_CHECK), 20, 338, 620, 220, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, w::ID_BTN_SETTINGS_OK), 400, 572, 110, 34, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, w::ID_BTN_SETTINGS_CANCEL), 530, 572, 110, 34, 1);
        return 0;
    }
    let ptr = w::GetWindowLongPtrW(hwnd, w::GWLP_USERDATA);
    if ptr == 0 { return w::DefWindowProcW(hwnd, msg, wparam, lparam); }
    let state = &mut *(ptr as *mut SettingsState);
    match msg {
        w::WM_COMMAND => {
            let id = (wparam & 0xFFFF) as u32;
            if id == w::ID_BTN_SETTINGS_OK || id == w::ID_BTN_SETTINGS_CANCEL {
                if id == w::ID_BTN_SETTINGS_OK {
                    let list = w::GetDlgItemCompat(hwnd, w::ID_RULES_CHECK);
                    let mut selected = Vec::new();
                    let count = w::SendMessageW(list, w::LVM_GETITEMCOUNT, 0, 0) as i32;
                    for i in 0..count {
                        let st = w::SendMessageW(list, w::LVM_GETITEMSTATE, i as usize, w::LVIS_STATEIMAGEMASK as isize) as u32;
                        if (st >> 12) == 2 {
                            if let Some(rule) = state.config.rules.get(i as usize) { selected.push(rule.name.clone()); }
                        }
                    }
                    state.result = Some(SettingsResult {
                        start_to_tray: w::IsDlgButtonCheckedCompat(hwnd, w::ID_CHK_START_TRAY),
                        show_tray_icon: w::IsDlgButtonCheckedCompat(hwnd, w::ID_CHK_TRAY_ICON),
                        auto_apply: w::IsDlgButtonCheckedCompat(hwnd, w::ID_CHK_AUTO_APPLY),
                        auto_apply_rules: selected,
                        exit_after_apply: w::IsDlgButtonCheckedCompat(hwnd, w::ID_CHK_EXIT_AFTER_APPLY),
                        auto_refresh: w::IsDlgButtonCheckedCompat(hwnd, w::ID_CHK_AUTO_REFRESH),
                    });
                }
                w::DestroyWindow(hwnd);
            }
            0
        }
        w::WM_CLOSE => { w::DestroyWindow(hwnd); 0 }
        w::WM_DESTROY => 0,
        _ => w::DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe extern "system" fn pick_wndproc(hwnd: w::HWND, msg: u32, wparam: w::WPARAM, lparam: w::LPARAM) -> w::LRESULT {
    if msg == w::WM_CREATE {
        let create = &*(lparam as *const CREATESTRUCTW);
        w::SetWindowLongPtrW(hwnd, w::GWLP_USERDATA, create.lpCreateParams as isize);
        let state = &mut *(create.lpCreateParams as *mut PickState);
        let font = w::GetStockObject(w::DEFAULT_GUI_FONT);
        let mk = |id: u32, class: &str, text: &str, style: u32| -> w::HWND {
            let h = w::CreateWindowExW(0, w::to_wide(class).as_ptr(), w::to_wide(text).as_ptr(),
                style, 0, 0, 10, 10, hwnd, id as usize as w::HMENU,
                self_instance(), std::ptr::null_mut());
            if !h.is_null() { w::SendMessageW(h, w::WM_SETFONT, font as usize, 1); }
            h
        };
        mk(ID_PICK_LABEL, "Static", "选择要加入的规则（双击规则名，或点新建）：", 0);
        let list = mk(ID_PICK_LIST, "SysListView32", "",
            w::WS_CHILD | w::WS_VISIBLE | w::WS_BORDER | w::WS_VSCROLL | w::WS_TABSTOP
                | w::LVS_REPORT | w::LVS_SHOWSELALWAYS | w::LVS_SINGLESEL);
        let mut title = w::to_wide("规则");
        let mut col = w::LVCOLUMNW::zeroed();
        col.mask = w::LVCF_FMT | w::LVCF_WIDTH | w::LVCF_TEXT | w::LVCF_SUBITEM;
        col.fmt = w::LVCFMT_LEFT; col.cx = 400; col.pszText = title.as_mut_ptr();
        w::SendMessageW(list, w::LVM_INSERTCOLUMNW, 0, &col as *const _ as isize);
        let mut texts: Vec<String> = vec!["新建规则（在下面输入框内输入规则名字）".to_string()];
        texts.extend(state.rule_names.iter().cloned());
        for (i, t) in texts.iter().enumerate() {
            let mut name = w::to_wide(t);
            let mut item = w::LVITEMW::zeroed();
            item.mask = w::LVIF_TEXT | w::LVIF_PARAM;
            item.iItem = i as i32; item.pszText = name.as_mut_ptr(); item.lParam = i as isize;
            w::SendMessageW(list, w::LVM_INSERTITEMW, 0, &item as *const _ as isize);
        }
        app_select_row(list, 0);
        mk(ID_PICK_NEWNAME, "Edit", &state.default_new,
            w::WS_CHILD | w::WS_VISIBLE | w::WS_BORDER | w::WS_TABSTOP | w::ES_AUTOHSCROLL);
        mk(ID_PICK_OK, "Button", "确定", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP);
        mk(ID_PICK_CANCEL, "Button", "取消", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, ID_PICK_LABEL), 16, 14, 440, 22, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, ID_PICK_LIST), 16, 42, 440, 240, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, ID_PICK_NEWNAME), 16, 292, 440, 26, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, ID_PICK_OK), 240, 330, 100, 30, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, ID_PICK_CANCEL), 356, 330, 100, 30, 1);
        return 0;
    }
    let ptr = w::GetWindowLongPtrW(hwnd, w::GWLP_USERDATA);
    if ptr == 0 { return w::DefWindowProcW(hwnd, msg, wparam, lparam); }
    let state = &mut *(ptr as *mut PickState);
    match msg {
        w::WM_COMMAND => {
            let id = (wparam & 0xFFFF) as u32;
            if id == ID_PICK_OK || id == ID_PICK_CANCEL {
                if id == ID_PICK_OK {
                    let sel = selected_pids(w::GetDlgItemCompat(hwnd, ID_PICK_LIST)).first().copied().unwrap_or(0) as usize;
                    if sel == 0 {
                        let mut buf = vec![0u16; 512];
                        w::GetWindowTextW(w::GetDlgItemCompat(hwnd, ID_PICK_NEWNAME), buf.as_mut_ptr(), buf.len() as i32);
                        state.result = Some(PickResult::New(w::from_wide(buf.as_ptr())));
                    } else if sel - 1 < state.rule_names.len() {
                        state.result = Some(PickResult::Existing(sel - 1));
                    }
                }
                w::DestroyWindow(hwnd);
            }
            0
        }
        w::WM_NOTIFY => {
            let header = &*(lparam as *const w::NMHDR);
            if header.code == w::NM_DBLCLK && header.idFrom == ID_PICK_LIST as usize {
                w::SendMessageW(hwnd, w::WM_COMMAND, ID_PICK_OK as usize, 0);
            }
            0
        }
        w::WM_CLOSE => { w::DestroyWindow(hwnd); 0 }
        w::WM_DESTROY => 0,
        _ => w::DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn show_pick_dialog(owner: w::HWND, state: &mut PickState) {
    let class = w::to_wide(PICK_CLASS);
    let hicon = w::LoadIconW(std::ptr::null_mut(), IDI_APPLICATION as usize as *const u16);
    let wc = w::WNDCLASSEXW {
        cbSize: std::mem::size_of::<w::WNDCLASSEXW>() as u32, style: 0,
        lpfnWndProc: pick_wndproc, cbClsExtra: 0, cbWndExtra: 0,
        hInstance: self_instance(), hIcon: hicon,
        hCursor: w::LoadCursorW(std::ptr::null_mut(), 32512usize as *const u16),
        hbrBackground: w::GetStockObject(0) as w::HBRUSH,
        lpszMenuName: std::ptr::null(), lpszClassName: class.as_ptr(), hIconSm: hicon,
    };
    let atom = w::RegisterClassExW(&wc);
    if atom == 0 && w::GetLastError() != 1410 { return; }
    let (cx, cy) = centered_pos(owner, 480, 400);
    let hwnd = w::CreateWindowExW(0, class.as_ptr(), w::to_wide("加入规则").as_ptr(),
        w::WS_OVERLAPPEDWINDOW & !0x0001_0000 & !0x0000_0400,
        cx, cy, 480, 400, std::ptr::null_mut(), std::ptr::null_mut(),
        self_instance(), state as *mut PickState as *mut c_void);
    if hwnd.is_null() { return; }
    w::EnableWindow(owner, 0);
    w::ShowWindow(hwnd, w::SW_SHOWNORMAL);
    w::UpdateWindow(hwnd);
    let mut msg: w::MSG = std::mem::zeroed();
    while w::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
        w::TranslateMessage(&msg); w::DispatchMessageW(&msg);
        if w::IsWindow(hwnd) == 0 { break; }
    }
    w::EnableWindow(owner, 1);
    w::SetForegroundWindow(owner);
}

fn app_select_row(list: w::HWND, row: i32) {
    let mut item = w::LVITEMW::zeroed();
    item.state = LVIS_SELECTED; item.stateMask = LVIS_SELECTED;
    unsafe { w::SendMessageW(list, LVM_SETITEMSTATE, row as usize, &item as *const _ as isize); }
}

unsafe extern "system" fn rename_wndproc(hwnd: w::HWND, msg: u32, wparam: w::WPARAM, lparam: w::LPARAM) -> w::LRESULT {
    if msg == w::WM_CREATE {
        let create = &*(lparam as *const CREATESTRUCTW);
        w::SetWindowLongPtrW(hwnd, w::GWLP_USERDATA, create.lpCreateParams as isize);
        let state = &mut *(create.lpCreateParams as *mut RenameState);
        let font = w::GetStockObject(w::DEFAULT_GUI_FONT);
        let mk = |id: u32, text: &str, style: u32| -> w::HWND {
            let h = w::CreateWindowExW(0, w::to_wide("Button").as_ptr(), w::to_wide(text).as_ptr(),
                style, 0, 0, 10, 10, hwnd, id as usize as w::HMENU,
                self_instance(), std::ptr::null_mut());
            if !h.is_null() { w::SendMessageW(h, w::WM_SETFONT, font as usize, 1); }
            h
        };
        let edit = w::CreateWindowExW(0, w::to_wide("Edit").as_ptr(), w::to_wide(&state.initial).as_ptr(),
            w::WS_CHILD | w::WS_VISIBLE | w::WS_BORDER | w::WS_TABSTOP | w::ES_AUTOHSCROLL,
            20, 20, 380, 28, hwnd, ID_RENAME_EDIT as usize as w::HMENU,
            self_instance(), std::ptr::null_mut());
        w::SendMessageW(edit, w::WM_SETFONT, font as usize, 1);
        mk(ID_RENAME_OK, "确定", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP);
        mk(ID_RENAME_CANCEL, "取消", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, ID_RENAME_OK), 80, 60, 100, 30, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, ID_RENAME_CANCEL), 210, 60, 100, 30, 1);
        return 0;
    }
    let ptr = w::GetWindowLongPtrW(hwnd, w::GWLP_USERDATA);
    if ptr == 0 { return w::DefWindowProcW(hwnd, msg, wparam, lparam); }
    let state = &mut *(ptr as *mut RenameState);
    match msg {
        w::WM_COMMAND => {
            let id = (wparam & 0xFFFF) as u32;
            if id == ID_RENAME_OK || id == ID_RENAME_CANCEL {
                if id == ID_RENAME_OK {
                    let mut buf = vec![0u16; 512];
                    w::GetWindowTextW(w::GetDlgItemCompat(hwnd, ID_RENAME_EDIT), buf.as_mut_ptr(), buf.len() as i32);
                    state.result = Some(w::from_wide(buf.as_ptr()));
                }
                w::DestroyWindow(hwnd);
            }
            0
        }
        w::WM_CLOSE => { w::DestroyWindow(hwnd); 0 }
        w::WM_DESTROY => 0,
        _ => w::DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn show_rename_dialog(owner: w::HWND, state: &mut RenameState, title: &str) {
    let class = w::to_wide(RENAME_CLASS);
    let hicon = w::LoadIconW(std::ptr::null_mut(), IDI_APPLICATION as usize as *const u16);
    let wc = w::WNDCLASSEXW {
        cbSize: std::mem::size_of::<w::WNDCLASSEXW>() as u32, style: 0,
        lpfnWndProc: rename_wndproc, cbClsExtra: 0, cbWndExtra: 0,
        hInstance: self_instance(), hIcon: hicon,
        hCursor: w::LoadCursorW(std::ptr::null_mut(), 32512usize as *const u16),
        hbrBackground: w::GetStockObject(0) as w::HBRUSH,
        lpszMenuName: std::ptr::null(), lpszClassName: class.as_ptr(), hIconSm: hicon,
    };
    let atom = w::RegisterClassExW(&wc);
    if atom == 0 && w::GetLastError() != 1410 { return; }
    let (cx, cy) = centered_pos(owner, 430, 150);
    let hwnd = w::CreateWindowExW(0, class.as_ptr(), w::to_wide(title).as_ptr(),
        w::WS_OVERLAPPEDWINDOW & !0x0001_0000 & !0x0000_0400,
        cx, cy, 430, 150, std::ptr::null_mut(), std::ptr::null_mut(),
        self_instance(), state as *mut RenameState as *mut c_void);
    if hwnd.is_null() { return; }
    w::EnableWindow(owner, 0);
    w::ShowWindow(hwnd, w::SW_SHOWNORMAL);
    w::UpdateWindow(hwnd);
    let mut msg: w::MSG = std::mem::zeroed();
    while w::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
        w::TranslateMessage(&msg); w::DispatchMessageW(&msg);
        if w::IsWindow(hwnd) == 0 { break; }
    }
    w::EnableWindow(owner, 1);
    w::SetForegroundWindow(owner);
}

/// 计算对话框在 owner 窗口中央的屏幕坐标（创建时直接传入，比事后移动可靠）
unsafe fn centered_pos(owner: w::HWND, ww: i32, wh: i32) -> (i32, i32) {
    let mut r: w::RECT = std::mem::zeroed();
    if w::GetWindowRect(owner, &mut r) != 0 {
        let ow = r.right - r.left;
        let oh = r.bottom - r.top;
        let x = r.left + (ow - ww) / 2;
        let y = r.top + (oh - wh) / 2;
        // 别让对话框跑到屏幕外
        let sx = unsafe { w::GetSystemMetrics(w::SM_CXSCREEN) };
        let sy = unsafe { w::GetSystemMetrics(w::SM_CYSCREEN) };
        return (x.max(0).min(sx - ww), y.max(0).min(sy - wh));
    }
    (200, 200)
}

unsafe extern "system" fn apply_wndproc(hwnd: w::HWND, msg: u32, wparam: w::WPARAM, lparam: w::LPARAM) -> w::LRESULT {
    if msg == w::WM_CREATE {
        let create = &*(lparam as *const CREATESTRUCTW);
        w::SetWindowLongPtrW(hwnd, w::GWLP_USERDATA, create.lpCreateParams as isize);
        let state = &mut *(create.lpCreateParams as *mut ApplyDialogState);
        let font = w::GetStockObject(w::DEFAULT_GUI_FONT);
        let mk = |id: u32, class: &str, text: &str, style: u32| -> w::HWND {
            let h = w::CreateWindowExW(0, w::to_wide(class).as_ptr(), w::to_wide(text).as_ptr(),
                style, 0, 0, 10, 10, hwnd, id as usize as w::HMENU,
                self_instance(), std::ptr::null_mut());
            if !h.is_null() { w::SendMessageW(h, w::WM_SETFONT, font as usize, 1); }
            h
        };
        mk(ID_APPLY_SUMMARY, "Static", &state.summary, 0);
        let list = mk(ID_APPLY_LIST, "SysListView32", "",
            w::WS_CHILD | w::WS_VISIBLE | w::WS_BORDER | w::WS_VSCROLL | w::WS_TABSTOP
                | w::LVS_REPORT | w::LVS_SHOWSELALWAYS);
        let ex = w::LVS_EX_FULLROWSELECT | w::LVS_EX_CHECKBOXES | w::LVS_EX_DOUBLEBUFFER;
        w::SendMessageW(list, w::LVM_SETEXTENDEDLISTVIEWSTYLE, 0, ex as isize);
        for (i, (title, width)) in [("进程名", 130), ("PID", 70), ("所属规则", 130), ("失败原因", 300)].iter().enumerate() {
            let mut t = w::to_wide(title);
            let mut col = w::LVCOLUMNW::zeroed();
            col.mask = w::LVCF_FMT | w::LVCF_WIDTH | w::LVCF_TEXT | w::LVCF_SUBITEM;
            col.fmt = w::LVCFMT_LEFT; col.cx = *width; col.pszText = t.as_mut_ptr(); col.iSubItem = i as i32;
            w::SendMessageW(list, w::LVM_INSERTCOLUMNW, i as usize, &col as *const _ as isize);
        }
        for (i, (name, pid, rule, err)) in state.failed.iter().enumerate() {
            let mut cells = vec![name.clone(), pid.to_string(), rule.clone(), err.clone()];
            let mut first = w::to_wide(&cells[0]);
            let mut item = w::LVITEMW::zeroed();
            item.mask = w::LVIF_TEXT | w::LVIF_PARAM;
            item.iItem = i as i32; item.pszText = first.as_mut_ptr(); item.lParam = i as isize;
            w::SendMessageW(list, w::LVM_INSERTITEMW, 0, &item as *const _ as isize);
            for (j, cell) in cells.iter_mut().enumerate().skip(1) {
                let mut text = w::to_wide(cell);
                let mut sub = w::LVITEMW::zeroed();
                sub.mask = w::LVIF_TEXT; sub.iItem = i as i32; sub.iSubItem = j as i32;
                sub.pszText = text.as_mut_ptr();
                w::SendMessageW(list, w::LVM_SETITEMW, 0, &sub as *const _ as isize);
            }
            // 默认勾选，方便一键移出
            let mut st = w::LVITEMW::zeroed();
            st.state = 2 << 12; st.stateMask = w::LVIS_STATEIMAGEMASK;
            w::SendMessageW(list, LVM_SETITEMSTATE, i as usize, &st as *const _ as isize);
        }
        mk(ID_APPLY_REMOVE, "Button", "从规则移出勾选的进程", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP);
        mk(ID_APPLY_CLOSE, "Button", "关闭", w::WS_CHILD | w::WS_VISIBLE | w::WS_TABSTOP);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, ID_APPLY_SUMMARY), 16, 14, 620, 62, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, ID_APPLY_LIST), 16, 84, 620, 300, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, ID_APPLY_REMOVE), 180, 398, 200, 30, 1);
        w::MoveWindow(w::GetDlgItemCompat(hwnd, ID_APPLY_CLOSE), 400, 398, 110, 30, 1);
        return 0;
    }
    let ptr = w::GetWindowLongPtrW(hwnd, w::GWLP_USERDATA);
    if ptr == 0 { return w::DefWindowProcW(hwnd, msg, wparam, lparam); }
    let state = &mut *(ptr as *mut ApplyDialogState);
    match msg {
        w::WM_COMMAND => {
            let id = (wparam & 0xFFFF) as u32;
            if id == ID_APPLY_REMOVE {
                let list = w::GetDlgItemCompat(hwnd, ID_APPLY_LIST);
                let count = w::SendMessageW(list, w::LVM_GETITEMCOUNT, 0, 0) as i32;
                for i in 0..count {
                    let st = w::SendMessageW(list, w::LVM_GETITEMSTATE, i as usize, w::LVIS_STATEIMAGEMASK as isize) as u32;
                    if (st >> 12) == 2 {
                        if let Some((name, _, rule, _)) = state.failed.get(i as usize) {
                            state.to_remove.push((name.clone(), rule.clone()));
                        }
                    }
                }
                w::DestroyWindow(hwnd);
            } else if id == ID_APPLY_CLOSE {
                w::DestroyWindow(hwnd);
            }
            0
        }
        w::WM_CLOSE => { w::DestroyWindow(hwnd); 0 }
        w::WM_DESTROY => 0,
        _ => w::DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn show_apply_dialog(owner: w::HWND, state: &mut ApplyDialogState) {
    let class = w::to_wide(APPLY_CLASS);
    let hicon = w::LoadIconW(std::ptr::null_mut(), IDI_APPLICATION as usize as *const u16);
    let wc = w::WNDCLASSEXW {
        cbSize: std::mem::size_of::<w::WNDCLASSEXW>() as u32, style: 0,
        lpfnWndProc: apply_wndproc, cbClsExtra: 0, cbWndExtra: 0,
        hInstance: self_instance(), hIcon: hicon,
        hCursor: w::LoadCursorW(std::ptr::null_mut(), 32512usize as *const u16),
        hbrBackground: w::GetStockObject(0) as w::HBRUSH,
        lpszMenuName: std::ptr::null(), lpszClassName: class.as_ptr(), hIconSm: hicon,
    };
    let atom = w::RegisterClassExW(&wc);
    if atom == 0 && w::GetLastError() != 1410 { return; }
    let (cx, cy) = centered_pos(owner, 660, 460);
    let hwnd = w::CreateWindowExW(0, class.as_ptr(), w::to_wide("应用结果 - 失败进程").as_ptr(),
        w::WS_OVERLAPPEDWINDOW & !0x0001_0000 & !0x0000_0400,
        cx, cy, 660, 460, std::ptr::null_mut(), std::ptr::null_mut(),
        self_instance(), state as *mut ApplyDialogState as *mut c_void);
    if hwnd.is_null() { return; }
    w::EnableWindow(owner, 0);
    w::ShowWindow(hwnd, w::SW_SHOWNORMAL);
    w::UpdateWindow(hwnd);
    let mut msg: w::MSG = std::mem::zeroed();
    while w::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
        w::TranslateMessage(&msg); w::DispatchMessageW(&msg);
        if w::IsWindow(hwnd) == 0 { break; }
    }
    w::EnableWindow(owner, 1);
    w::SetForegroundWindow(owner);
}

unsafe fn show_settings_dialog(owner: w::HWND, state: &mut SettingsState) {
    let class = w::to_wide(SETTINGS_CLASS);
    let hicon = w::LoadIconW(std::ptr::null_mut(), IDI_APPLICATION as usize as *const u16);
    let wc = w::WNDCLASSEXW {
        cbSize: std::mem::size_of::<w::WNDCLASSEXW>() as u32, style: 0,
        lpfnWndProc: settings_wndproc, cbClsExtra: 0, cbWndExtra: 0,
        hInstance: self_instance(), hIcon: hicon,
        hCursor: w::LoadCursorW(std::ptr::null_mut(), 32512usize as *const u16),
        hbrBackground: w::GetStockObject(0) as w::HBRUSH,
        lpszMenuName: std::ptr::null(), lpszClassName: class.as_ptr(), hIconSm: hicon,
    };
    let atom = w::RegisterClassExW(&wc);
    if atom == 0 && w::GetLastError() != 1410 { return; }
    let (cx, cy) = centered_pos(owner, 680, 745);
    let hwnd = w::CreateWindowExW(0, class.as_ptr(), w::to_wide("设置 - EcoPilot").as_ptr(),
        w::WS_OVERLAPPEDWINDOW & !0x0001_0000 & !0x0000_0400,
        cx, cy, 680, 745, std::ptr::null_mut(), std::ptr::null_mut(),
        self_instance(), state as *mut SettingsState as *mut c_void);
    if hwnd.is_null() { return; }
    w::EnableWindow(owner, 0);
    w::ShowWindow(hwnd, w::SW_SHOWNORMAL);
    w::UpdateWindow(hwnd);
    let mut msg: w::MSG = std::mem::zeroed();
    while w::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
        w::TranslateMessage(&msg); w::DispatchMessageW(&msg);
        if w::IsWindow(hwnd) == 0 { break; }
    }
    w::EnableWindow(owner, 1);
    w::SetForegroundWindow(owner);
}

unsafe fn file_dialog(owner: w::HWND, save: bool, title: &str, filter: &str, default_name: &str) -> Option<PathBuf> {
    let filter_wide = w::to_wide(filter);
    let title_wide = w::to_wide(title);
    let mut file = w::to_wide(default_name);
    file.resize(32768, 0);
    let mut ofn: w::OPENFILENAMEW = std::mem::zeroed();
    ofn.lStructSize = std::mem::size_of::<w::OPENFILENAMEW>() as u32;
    ofn.hwndOwner = owner;
    ofn.lpstrFilter = filter_wide.as_ptr();
    ofn.lpstrFile = file.as_mut_ptr();
    ofn.nMaxFile = file.len() as u32;
    ofn.lpstrTitle = title_wide.as_ptr();
    ofn.Flags = w::OFN_EXPLORER | w::OFN_HIDEREADONLY | w::OFN_PATHMUSTEXIST
        | if save { w::OFN_OVERWRITEPROMPT } else { w::OFN_FILEMUSTEXIST };
    let ok = if save { w::GetSaveFileNameW(&mut ofn) } else { w::GetOpenFileNameW(&mut ofn) };
    if ok == 0 { return None; }
    let end = file.iter().position(|c| *c == 0).unwrap_or(file.len());
    Some(PathBuf::from(String::from_utf16_lossy(&file[..end])))
}

fn enable_dpi_awareness() {
    type FnDpi = unsafe extern "system" fn(*mut c_void) -> i32;
    unsafe {
        let user32 = w::GetModuleHandleW(w::to_wide("user32").as_ptr());
        if user32.is_null() { return; }
        let address = w::GetProcAddress(user32, b"SetProcessDpiAwarenessContext\0".as_ptr());
        if address.is_null() { return; }
        let function: FnDpi = std::mem::transmute(address);
        function(usize::MAX as *mut c_void);
    }
}

fn print_usage() {
    println!("自由效能进程EcoPilot-netdna518 · v1.58");
    println!();
    println!("用法：");
    println!("  EcoPilot.exe                 启动界面");
    println!("  EcoPilot.exe --selftest      自检 EcoQoS");
    println!("  EcoPilot.exe --list          列出进程及效能状态");
    println!("  EcoPilot.exe --apply         应用规则后启动界面");
    println!("  EcoPilot.exe --apply --tray  应用规则后仅在托盘运行");
    println!();
    println!("纯绿色：settings.json 与 rules/*.json 全部在程序目录，不写注册表。");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let config_path = eco::default_config_path();

    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") { print_usage(); return; }
    if args.iter().any(|a| a == "--selftest") {
        match eco::selftest() {
            Ok(text) => { println!("{}", text); println!("自检通过。"); }
            Err(msg) => { eprintln!("自检失败：{}", msg); std::process::exit(1); }
        }
        return;
    }
    if args.iter().any(|a| a == "--list") {
        let procs = eco::list_processes();
        println!("{:<34} {:>8} {:<9} {}", "进程名", "PID", "状态", "路径");
        for p in procs.iter().take(400) {
            let short: String = p.name.chars().take(32).collect();
            println!("{:<34} {:>8} {:<9} {}", short, p.pid, if p.is_eco() { "效能中" } else { "普通" }, p.path);
        }
        println!("共 {} 个进程。", procs.len());
        return;
    }

    // 纯绿色：清除 v1/v2 遗留的注册表自启项（v3 自身永不写注册表）
    eco::cleanup_legacy_registry_autostart();

    let mut config = match eco::load_config(&config_path) {
        Ok(c) => c, Err(msg) => { eprintln!("{}", msg); std::process::exit(1); }
    };
    // 从 rules 目录加载所有规则（几个 JSON 几条规则）
    match eco::load_rules_from_files(&mut config) {
        Ok(n) => { if n == 0 && config.rules.is_empty() { /* 首次运行，空规则 */ } }
        Err(msg) => eprintln!("加载规则失败：{}", msg),
    }

    let apply_on_start = args.iter().any(|a| a == "--apply") || config.auto_apply;
    let start_to_tray = args.iter().any(|a| a == "--tray") || config.start_to_tray;
    let selected_rules: &[String] = if config.auto_apply_rules.is_empty() { &[] } else { &config.auto_apply_rules };

    let initial_report = if apply_on_start {
        let procs = eco::list_processes();
        let report = eco::apply_rules(&config, &procs, selected_rules);
        if !args.iter().any(|a| a == "--tray") {
            println!("开机应用：匹配 {}，成功 {}，失败 {}。", report.matched, report.applied.len(), report.failed.len());
        }
        // 静默后台模式：自动应用成功后退出（仅在规则生效且托盘模式下，避免误退出交互会话）
        if config.exit_after_apply {
            // 勾了"应用成功后退出"：应用流程已执行完就退出，不因 0 命中或个别失败而卡住
            return;
        }
        Some(report)
    } else { None };

    enable_dpi_awareness();
    unsafe { w::FreeConsole(); }

    let mutex_name = w::to_wide("EcoPilot_SingleInstance_V3");
    let mutex = unsafe { w::CreateMutexW(std::ptr::null(), 0, mutex_name.as_ptr()) };
    if !mutex.is_null() && unsafe { w::GetLastError() } == w::ERROR_ALREADY_EXISTS {
        unsafe { w::MessageBoxW(std::ptr::null_mut(), w::to_wide("EcoPilot 已经在运行。").as_ptr(),
            w::to_wide(APP_TITLE).as_ptr(), w::MB_ICONINFORMATION); }
        return;
    }

    let class = w::to_wide(CLASS_NAME);
    let wc = w::WNDCLASSEXW {
        cbSize: std::mem::size_of::<w::WNDCLASSEXW>() as u32, style: 0,
        lpfnWndProc: wndproc, cbClsExtra: 0, cbWndExtra: 0,
        hInstance: unsafe { self_instance() },
        hIcon: unsafe { w::LoadImageW(self_instance(), 1usize as *const u16, w::IMAGE_ICON, 0, 0, w::LR_DEFAULTSIZE) as w::HICON },
        hCursor: unsafe { w::LoadCursorW(std::ptr::null_mut(), 32512usize as *const u16) },
        hbrBackground: unsafe { w::GetStockObject(0) as w::HBRUSH },
        lpszMenuName: std::ptr::null(), lpszClassName: class.as_ptr(), hIconSm: std::ptr::null_mut(),
    };
    if unsafe { w::RegisterClassExW(&wc) } == 0 {
        eprintln!("注册窗口类失败：{}", w::last_error_text("RegisterClassExW")); return;
    }
    let screen_w = unsafe { w::GetSystemMetrics(w::SM_CXSCREEN) };
    let screen_h = unsafe { w::GetSystemMetrics(w::SM_CYSCREEN) };
    let win_w = 1600i32.min(screen_w - 40).max(900);
    let win_h = 860i32.min(screen_h - 60).max(560);

    let mut app = App::new(config_path, config, start_to_tray, initial_report);
    let hwnd = unsafe {
        w::CreateWindowExW(w::WS_EX_CONTROLPARENT, class.as_ptr(), w::to_wide(APP_TITLE).as_ptr(),
            w::WS_OVERLAPPEDWINDOW, (screen_w - win_w) / 2, (screen_h - win_h) / 2,
            win_w, win_h, std::ptr::null_mut(), std::ptr::null_mut(),
            self_instance(), &mut app as *mut App as *mut c_void)
    };
    if hwnd.is_null() { eprintln!("创建窗口失败：{}", w::last_error_text("CreateWindowExW")); return; }
    unsafe {
        if start_to_tray { w::ShowWindow(hwnd, w::SW_HIDE); } else { w::ShowWindow(hwnd, w::SW_SHOWNORMAL); }
        app.sync_timer(); // 按最终可见性启停定时器
        w::UpdateWindow(hwnd);
        w::SendMessageW(hwnd, w::WM_SIZE, 0, 0);
    }
    let mut message: w::MSG = unsafe { std::mem::zeroed() };
    while unsafe { w::GetMessageW(&mut message, std::ptr::null_mut(), 0, 0) } > 0 {
        unsafe { w::TranslateMessage(&message); w::DispatchMessageW(&message); }
    }
}
