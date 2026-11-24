//! 效率模式核心：进程枚举、EcoQoS 读写、规则持久化、开机自启。

use crate::mini_json::{self, Json};
use crate::winapi as w;
use std::ffi::c_void;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub path: String,
    pub priority: u32,
    pub eco: Option<bool>,
    pub cpu: f64,
    pub memory_mb: f64,
    pub disk_kb_s: f64,
    pub connections: usize,
}

/// 内置保护名单：这些进程不允许设成效能模式。
pub const BUILTIN_PROTECTED: &[&str] = &[
    "system", "system idle process", "registry", "memory compression",
    "smss.exe", "csrss.exe", "wininit.exe", "winlogon.exe", "services.exe",
    "lsass.exe", "svchost.exe", "fontdrvhost.exe", "dwm.exe",
    "sihost.exe", "ctfmon.exe", "shellexperiencehost.exe", "startmenuexperiencehost.exe",
    "searchhost.exe", "searchindexer.exe", "audiodg.exe", "spoolsv.exe",
    "winDefend.exe", "msmpeng.exe", "securityhealthservice.exe",
    "ecoPilot.exe",
];

impl ProcessInfo {
    pub fn is_eco(&self) -> bool {
        self.eco.unwrap_or(self.priority == w::IDLE_PRIORITY_CLASS)
    }

    pub fn priority_text(&self) -> String {
        priority_name(self.priority)
    }
}

#[derive(Clone, Debug)]
pub struct Rule {
    /// 规则名称（可重命名），同时也是 JSON 文件名的一部分
    pub name: String,
    /// 该规则管辖的进程名列表（可多个进程）
    pub members: Vec<String>,
    pub path: String,
    pub match_path: bool,
    pub enabled: bool,
    pub restore_priority: u32,
    pub note: String,
}

impl Rule {
    pub fn matches(&self, proc: &ProcessInfo) -> bool {
        if !self.enabled {
            return false;
        }
        // 新模型：成员列表命中任一进程名
        if !self.members.is_empty() {
            return self.members.iter().any(|m| proc.name.eq_ignore_ascii_case(m));
        }
        // 兼容旧单进程规则
        if self.match_path && !self.path.is_empty() {
            return !proc.path.is_empty() && proc.path.eq_ignore_ascii_case(&self.path);
        }
        proc.name.eq_ignore_ascii_case(&self.name)
    }

    /// 该规则是否管辖某个进程名（用于"移出本规则"）
    pub fn covers(&self, proc_name: &str) -> bool {
        self.members.iter().any(|m| m.eq_ignore_ascii_case(proc_name))
            || (self.members.is_empty() && self.name.eq_ignore_ascii_case(proc_name))
    }

    pub fn display_name(&self) -> String {
        if self.match_path && !self.path.is_empty() {
            self.path.clone()
        } else {
            self.name.clone()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub version: u32,
    pub rules: Vec<Rule>,
    pub auto_apply: bool,
    pub autostart: bool,
    pub refresh_seconds: u32,
    pub start_to_tray: bool,
    pub show_tray_icon: bool,
    /// 启动时自动应用的规则名；为空表示应用全部启用规则。
    pub auto_apply_rules: Vec<String>,
    /// 用户在界面里右键加入的保护进程名。
    pub protected: Vec<String>,
    /// 启动自动应用规则成功后自动退出程序（用于纯后台静默应用）
    pub exit_after_apply: bool,
    /// 是否定时自动刷新进程列表（默认关闭，避免空闲时周期性 CPU 占用）
    pub auto_refresh: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            rules: Vec::new(),
            auto_apply: true,
            autostart: false,
            refresh_seconds: 5,
            start_to_tray: false,
            show_tray_icon: true,
            auto_apply_rules: Vec::new(),
            protected: Vec::new(),
            exit_after_apply: false,
            auto_refresh: true,
        }
    }
}

impl Config {
    pub fn to_json(&self) -> Json {
        let mut root = Json::Obj(Vec::new());
        root.set("version", Json::Num(self.version as f64));
        root.set("auto_apply", Json::Bool(self.auto_apply));
        root.set("autostart", Json::Bool(self.autostart));
        root.set("refresh_seconds", Json::Num(self.refresh_seconds as f64));
        root.set("start_to_tray", Json::Bool(self.start_to_tray));
        root.set("show_tray_icon", Json::Bool(self.show_tray_icon));
        let auto_rules: Vec<Json> = self.auto_apply_rules.iter().map(|s| Json::Str(s.clone())).collect();
        root.set("auto_apply_rules", Json::Arr(auto_rules));
        let protected: Vec<Json> = self.protected.iter().map(|s| Json::Str(s.clone())).collect();
        root.set("protected", Json::Arr(protected));
        root.set("exit_after_apply", Json::Bool(self.exit_after_apply));
        root.set("auto_refresh", Json::Bool(self.auto_refresh));
        root
    }

    pub fn from_json(root: &Json) -> Result<Self, String> {
        let mut cfg = Config::default();
        if let Some(v) = root.get("version").and_then(Json::as_f64) {
            cfg.version = v as u32;
        }
        if let Some(v) = root.get("auto_apply").and_then(Json::as_bool) {
            cfg.auto_apply = v;
        }
        if let Some(v) = root.get("autostart").and_then(Json::as_bool) {
            cfg.autostart = v;
        }
        if let Some(v) = root.get("start_to_tray").and_then(Json::as_bool) {
            cfg.start_to_tray = v;
        }
        if let Some(v) = root.get("show_tray_icon").and_then(Json::as_bool) {
            cfg.show_tray_icon = v;
        }
        if let Some(v) = root.get("refresh_seconds").and_then(Json::as_f64) {
            cfg.refresh_seconds = (v as u32).clamp(1, 60);
        }
        if let Some(items) = root.get("auto_apply_rules").and_then(Json::as_arr) {
            cfg.auto_apply_rules = items.iter().filter_map(|i| i.as_str().map(str::to_string)).collect();
        }
        if let Some(items) = root.get("protected").and_then(Json::as_arr) {
            cfg.protected = items.iter().filter_map(|i| i.as_str().map(str::to_string)).collect();
        }
        if let Some(v) = root.get("exit_after_apply").and_then(Json::as_bool) {
            cfg.exit_after_apply = v;
        }
        if let Some(v) = root.get("auto_refresh").and_then(Json::as_bool) {
            cfg.auto_refresh = v;
        }
        if let Some(items) = root.get("rules").and_then(Json::as_arr) {
            for item in items {
                let name = item.get("name").and_then(Json::as_str).unwrap_or("").to_string();
                if name.is_empty() {
                    continue;
                }
                let members = item.get("members").and_then(Json::as_arr)
                    .map(|a| a.iter().filter_map(|i| i.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                let path = item.get("path").and_then(Json::as_str).unwrap_or("").to_string();
                let match_path = item.get("match_path").and_then(Json::as_bool).unwrap_or(false);
                let enabled = item.get("enabled").and_then(Json::as_bool).unwrap_or(true);
                let restore_priority = item.get("restore_priority").and_then(Json::as_f64)
                    .map(|n| n as u32).unwrap_or(w::NORMAL_PRIORITY_CLASS);
                let note = item.get("note").and_then(Json::as_str).unwrap_or("").to_string();
                cfg.rules.push(Rule { name, members, path, match_path, enabled, restore_priority, note });
            }
        }
        Ok(cfg)
    }
}

pub fn priority_name(priority: u32) -> String {
    match priority {
        w::IDLE_PRIORITY_CLASS => "低（Idle）".to_string(),
        w::BELOW_NORMAL_PRIORITY_CLASS => "低于正常".to_string(),
        w::NORMAL_PRIORITY_CLASS => "正常".to_string(),
        w::ABOVE_NORMAL_PRIORITY_CLASS => "高于正常".to_string(),
        w::HIGH_PRIORITY_CLASS => "高".to_string(),
        0x0000_0100 => "实时".to_string(),
        other => format!("0x{:X}", other),
    }
}

type ProcessInfoFn = unsafe extern "system" fn(
    w::HANDLE, u32, *mut c_void, u32,
) -> i32;

unsafe fn kernel32_fn(name: &[u8]) -> Option<ProcessInfoFn> {
    let kernel = w::GetModuleHandleW(w::to_wide("kernel32").as_ptr());
    if kernel.is_null() {
        return None;
    }
    let address = w::GetProcAddress(kernel, name.as_ptr());
    if address.is_null() {
        return None;
    }
    Some(std::mem::transmute::<*mut c_void, ProcessInfoFn>(address))
}

/// 通过 GetProcessInformation 读取 EcoQoS 状态。API 不存在时返回 None。
pub fn query_eco(handle: w::HANDLE) -> Option<bool> {
    let get_info = unsafe { kernel32_fn(b"GetProcessInformation\0") }?;
    let mut state = w::PROCESS_POWER_THROTTLING_STATE {
        Version: w::PROCESS_POWER_THROTTLING_CURRENT_VERSION,
        ControlMask: 0,
        StateMask: 0,
    };
    let ok = unsafe {
        get_info(
            handle,
            w::PROCESS_POWER_THROTTLING_CLASS,
            &mut state as *mut _ as *mut c_void,
            std::mem::size_of::<w::PROCESS_POWER_THROTTLING_STATE>() as u32,
        )
    };
    if ok == 0 {
        None
    } else {
        Some(state.StateMask & w::PROCESS_POWER_THROTTLING_EXECUTION_SPEED != 0)
    }
}

/// 保护来源：0=未保护 1=内置保护 2=用户保护
pub fn protection_kind(name: &str, config: &Config) -> u8 {
    let lower = name.to_lowercase();
    if BUILTIN_PROTECTED.iter().any(|p| lower == *p) { 1 }
    else if config.protected.iter().any(|p| lower == p.to_lowercase()) { 2 }
    else { 0 }
}

/// 进程是否在保护名单里（内置 + 用户添加），保护进程禁止设成效能模式。
pub fn is_protected(name: &str, config: &Config) -> bool {
    protection_kind(name, config) != 0
}

/// 与任务管理器「效率模式」一致：Idle 优先级 + EcoQoS(EXECUTION_SPEED)。
pub fn set_efficiency(pid: u32, enable: bool, restore_priority: u32) -> Result<String, String> {
    let handle = unsafe {
        w::OpenProcess(
            w::PROCESS_QUERY_LIMITED_INFORMATION | w::PROCESS_SET_INFORMATION,
            0,
            pid,
        )
    };
    if handle.is_null() {
        return Err(format!("无法打开进程 {}（访问被拒或进程已退出，可尝试管理员身份运行）", pid));
    }

    let set_info = unsafe { kernel32_fn(b"SetProcessInformation\0") };
    if set_info.is_none() {
        unsafe { w::CloseHandle(handle); }
        return Err("系统缺少 SetProcessInformation（需要 Windows 10 1709 以上）".to_string());
    }

    let mut state = w::PROCESS_POWER_THROTTLING_STATE {
        Version: w::PROCESS_POWER_THROTTLING_CURRENT_VERSION,
        ControlMask: w::PROCESS_POWER_THROTTLING_EXECUTION_SPEED,
        StateMask: if enable { w::PROCESS_POWER_THROTTLING_EXECUTION_SPEED } else { 0 },
    };
    let eco_ok = unsafe {
        set_info.unwrap()(
            handle,
            w::PROCESS_POWER_THROTTLING_CLASS,
            &mut state as *mut _ as *mut c_void,
            std::mem::size_of::<w::PROCESS_POWER_THROTTLING_STATE>() as u32,
        )
    } != 0;

    let target_priority = if enable {
        w::IDLE_PRIORITY_CLASS
    } else if is_valid_priority(restore_priority) {
        restore_priority
    } else {
        w::NORMAL_PRIORITY_CLASS
    };
    let priority_ok = unsafe { w::SetPriorityClass(handle, target_priority) } != 0;

    let verified = query_eco(handle);
    unsafe { w::CloseHandle(handle); }

    if !eco_ok && !priority_ok {
        return Err(format!("进程 {} 设置失败（EcoQoS 与优先级均被拒绝）", pid));
    }

    let mut message = format!(
        "进程 {}：{}（EcoQoS {}，优先级 {}）",
        pid,
        if enable { "已开启效率模式" } else { "已恢复正常" },
        if eco_ok { "已写入" } else { "被拒绝" },
        if priority_ok { "已写入" } else { "被拒绝" },
    );
    if let Some(on) = verified {
        message.push_str(if on == enable { "，回读正确" } else { "，回读与请求不一致" });
    }
    Ok(message)
}

fn is_valid_priority(p: u32) -> bool {
    matches!(
        p,
        w::IDLE_PRIORITY_CLASS
            | w::BELOW_NORMAL_PRIORITY_CLASS
            | w::NORMAL_PRIORITY_CLASS
            | w::ABOVE_NORMAL_PRIORITY_CLASS
            | w::HIGH_PRIORITY_CLASS
    )
}

pub fn process_status(pid: u32) -> Option<ProcessInfo> {
    let handle = unsafe {
        w::OpenProcess(w::PROCESS_QUERY_LIMITED_INFORMATION, 0, pid)
    };
    if handle.is_null() {
        return None;
    }
    let priority = unsafe { w::GetPriorityClass(handle) };
    let eco = query_eco(handle);
    let mut path = String::new();
    let mut buffer = vec![0u16; 32768];
    let mut size = buffer.len() as u32;
    if unsafe {
        w::QueryFullProcessImageNameW(handle, 0, buffer.as_mut_ptr(), &mut size)
    } != 0 && size > 0 {
        path = String::from_utf16_lossy(&buffer[..size as usize]);
    }
    unsafe { w::CloseHandle(handle); }
    Some(ProcessInfo {
        pid, name: String::new(), path, priority, eco,
        cpu: 0.0, memory_mb: 0.0, disk_kb_s: 0.0, connections: 0,
    })
}

pub fn list_processes() -> Vec<ProcessInfo> {
    list_processes_with_metrics(None)
}

pub fn list_processes_with_metrics(sampler: Option<&mut crate::perf::Sampler>) -> Vec<ProcessInfo> {
    let mut result = Vec::new();
    let snapshot = unsafe { w::CreateToolhelp32Snapshot(w::TH32CS_SNAPPROCESS, 0) };
    if snapshot == w::INVALID_HANDLE_VALUE {
        return result;
    }
    let mut entry: w::PROCESSENTRY32W = unsafe { std::mem::zeroed() };
    entry.dwSize = std::mem::size_of::<w::PROCESSENTRY32W>() as u32;
    if unsafe { w::Process32FirstW(snapshot, &mut entry) } == 0 {
        unsafe { w::CloseHandle(snapshot); }
        return result;
    }
    loop {
        let name = unsafe { w::from_wide(entry.szExeFile.as_ptr()) };
        let pid = entry.th32ProcessID;
        if pid != 0 {
            if let Some(mut info) = process_status(pid) {
                info.name = name;
                result.push(info);
            } else {
                result.push(ProcessInfo {
                    pid,
                    name,
                    path: String::new(),
                    priority: entry.pcPriClassBase as u32,
                    eco: None,
                    cpu: 0.0, memory_mb: 0.0, disk_kb_s: 0.0, connections: 0,
                });
            }
        }
        if unsafe { w::Process32NextW(snapshot, &mut entry) } == 0 {
            break;
        }
    }
    unsafe { w::CloseHandle(snapshot); }

    if let Some(sampler) = sampler {
        sampler.poll_connections();
        for info in result.iter_mut() {
            let handle = unsafe {
                w::OpenProcess(w::PROCESS_QUERY_LIMITED_INFORMATION, 0, info.pid)
            };
            let metrics = sampler.sample(handle, info.pid);
            if !handle.is_null() {
                unsafe { w::CloseHandle(handle); }
            }
            info.cpu = metrics.cpu_percent;
            info.memory_mb = metrics.memory_mb;
            info.disk_kb_s = metrics.disk_kb_s;
            info.connections = metrics.connections;
        }
        sampler.finish_cycle();
    }

    result.sort_by(|a, b| b.cpu.abs().partial_cmp(&a.cpu.abs()).unwrap_or(std::cmp::Ordering::Equal)
        .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        .then(a.pid.cmp(&b.pid)));
    result
}

pub struct ApplyReport {
    pub applied: Vec<String>,
    pub failed: Vec<String>,
    /// 失败的进程明细：(进程名, PID, 所属规则名, 错误)
    pub failed_procs: Vec<(String, u32, String, String)>,
    pub matched: usize,
}

/// selected_only 非空时只应用这些名字对应的规则。
pub fn apply_rules(config: &Config, processes: &[ProcessInfo], selected_only: &[String]) -> ApplyReport {
    let mut report = ApplyReport { applied: Vec::new(), failed: Vec::new(), failed_procs: Vec::new(), matched: 0 };
    for proc in processes {
        if is_protected(&proc.name, config) {
            continue;
        }
        let mut matched_rule = None;
        for rule in &config.rules {
            if !selected_only.is_empty()
                && !selected_only.iter().any(|n| n.eq_ignore_ascii_case(&rule.name))
            {
                continue;
            }
            if rule.matches(proc) {
                matched_rule = Some(rule);
                break;
            }
        }
        let Some(rule) = matched_rule else { continue };
        report.matched += 1;
        match set_efficiency(proc.pid, true, rule.restore_priority) {
            Ok(msg) => report.applied.push(msg),
            Err(msg) => {
                report.failed_procs.push((proc.name.clone(), proc.pid, rule.name.clone(), msg.clone()));
                report.failed.push(msg);
            }
        }
    }
    report
}

/// 找出某个进程命中的规则（不受 enabled 影响，供树形列表展示）。
pub fn matching_rule<'a>(config: &'a Config, proc: &ProcessInfo) -> Option<&'a Rule> {
    for rule in &config.rules {
        let hit = if rule.match_path && !rule.path.is_empty() {
            !proc.path.is_empty() && proc.path.eq_ignore_ascii_case(&rule.path)
        } else {
            proc.name.eq_ignore_ascii_case(&rule.name)
        };
        if hit {
            return Some(rule);
        }
    }
    None
}

pub fn restore_all(processes: &[ProcessInfo], config: &Config) -> (usize, Vec<String>) {
    let mut done = 0;
    let mut errors = Vec::new();
    for proc in processes.iter().filter(|p| p.is_eco() && !is_protected(&p.name, config)) {
        let restore = config.rules.iter()
            .find(|r| r.matches(proc))
            .map(|r| r.restore_priority)
            .unwrap_or(w::NORMAL_PRIORITY_CLASS);
        match set_efficiency(proc.pid, false, restore) {
            Ok(_) => done += 1,
            Err(msg) => errors.push(msg),
        }
    }
    (done, errors)
}

pub fn app_dir() -> PathBuf {
    let mut buffer = vec![0u16; 32768];
    let len = unsafe { w::GetModuleFileNameW(std::ptr::null_mut(), buffer.as_mut_ptr(), buffer.len() as u32) };
    let exe = if len > 0 {
        PathBuf::from(String::from_utf16_lossy(&buffer[..len as usize]))
    } else {
        PathBuf::from("EcoPilot.exe")
    };
    exe.parent().unwrap_or(Path::new(".")).to_path_buf()
}

pub fn default_config_path() -> PathBuf {
    app_dir().join("settings.json")
}

/// 规则文件直接放程序目录（纯绿色，用户一眼能看到）
pub fn rules_dir() -> PathBuf {
    app_dir()
}

/// 单条规则的 JSON 表示（一条规则一个文件）
fn rule_to_json(rule: &Rule) -> Json {
    let mut obj = Json::Obj(Vec::new());
    obj.set("name", Json::Str(rule.name.clone()));
    let members: Vec<Json> = rule.members.iter().map(|m| Json::Str(m.clone())).collect();
    obj.set("members", Json::Arr(members));
    obj.set("path", Json::Str(rule.path.clone()));
    obj.set("match_path", Json::Bool(rule.match_path));
    obj.set("enabled", Json::Bool(rule.enabled));
    obj.set("restore_priority", Json::Num(rule.restore_priority as f64));
    obj.set("note", Json::Str(rule.note.clone()));
    obj
}

fn rule_from_json(item: &Json) -> Option<Rule> {
    let name = item.get("name").and_then(Json::as_str).unwrap_or("").to_string();
    if name.is_empty() { return None; }
    let members = item.get("members").and_then(Json::as_arr)
        .map(|a| a.iter().filter_map(|i| i.as_str().map(str::to_string)).collect())
        .unwrap_or_default();
    Some(Rule {
        name,
        members,
        path: item.get("path").and_then(Json::as_str).unwrap_or("").to_string(),
        match_path: item.get("match_path").and_then(Json::as_bool).unwrap_or(false),
        enabled: item.get("enabled").and_then(Json::as_bool).unwrap_or(true),
        restore_priority: item.get("restore_priority").and_then(Json::as_f64)
            .map(|n| n as u32).unwrap_or(w::NORMAL_PRIORITY_CLASS),
        note: item.get("note").and_then(Json::as_str).unwrap_or("").to_string(),
    })
}

/// 从程序目录 规则-*.json 加载所有规则；同时兼容旧的 rules 子目录。
pub fn load_rules_from_files(config: &mut Config) -> Result<usize, String> {
    let dir = app_dir();
    config.rules.clear();
    let mut count = 0;
    let mut paths: Vec<PathBuf> = Vec::new();
    // 程序目录下的 规则-*.json
    if let Ok(entries) = std::fs::read_dir(&dir) {
        paths.extend(entries.filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| {
                let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
                name.starts_with("规则-") && name.to_lowercase().ends_with(".json")
            }));
    }
    // 兼容旧版 rules 子目录
    let legacy = app_dir().join("rules");
    if let Ok(entries) = std::fs::read_dir(&legacy) {
        paths.extend(entries.filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("json")).unwrap_or(false)));
    }
    paths.sort();
    for path in paths {
        let text = match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => { eprintln!("跳过规则文件 {}：{}", path.display(), e); continue; }
        };
        let root = match mini_json::parse(&text) {
            Ok(j) => j,
            Err(e) => { eprintln!("跳过规则文件 {}：格式错误 {}", path.display(), e); continue; }
        };
        // 兼容两种格式：单条规则对象，或 {"rules":[...]} 列表
        if let Some(items) = root.get("rules").and_then(Json::as_arr) {
            for item in items {
                if let Some(rule) = rule_from_json(item) { config.rules.push(rule); count += 1; }
            }
        } else if let Some(rule) = rule_from_json(&root) {
            config.rules.push(rule);
            count += 1;
        }
    }
    Ok(count)
}

fn safe_file_name(name: &str, _index: usize) -> String {
    let base: String = name.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
        .take(48).collect();
    format!("规则-{}.json", if base.is_empty() { "未命名" } else { &base })
}

/// 把每条规则写成独立的 JSON 文件；删除已不存在的规则文件。
pub fn save_rules_to_files(config: &Config) -> Result<(), String> {
    let dir = rules_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir).map_err(|e| format!("创建规则目录失败：{}", e))?;
    }
    let mut expected = Vec::new();
    for (i, rule) in config.rules.iter().enumerate() {
        let file = dir.join(safe_file_name(&rule.name, i));
        expected.push(file.clone());
        let text = rule_to_json(rule).stringify();
        atomic_write(&file, &text)?;
    }
    // 清理程序目录里不再对应的旧"规则-*.json"（绝不碰 settings.json 等其它文件）
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if name.starts_with("规则-") && name.to_lowercase().ends_with(".json")
                && !expected.contains(&path)
            {
                let _ = std::fs::remove_file(&path);
            }
        }
    }
    // 兼容清理旧 rules 子目录里的规则文件
    let legacy = app_dir().join("rules");
    if let Ok(entries) = std::fs::read_dir(&legacy) {
        for entry in entries.filter_map(|e| e.ok()) {
            let _ = std::fs::remove_file(entry.path());
        }
        let _ = std::fs::remove_dir(&legacy);
    }
    Ok(())
}

fn atomic_write(path: &Path, text: &str) -> Result<(), String> {
    let mut temp = path.to_path_buf();
    temp.set_file_name(format!(".{}.{}.tmp",
        path.file_name().unwrap_or_default().to_string_lossy(), std::process::id()));
    std::fs::write(&temp, text).map_err(|e| format!("写入临时文件失败：{}", e))?;
    const REPLACE: u32 = 0x1;
    const WRITE_THROUGH: u32 = 0x8;
    let ok = unsafe {
        w::MoveFileExW(w::to_wide(&temp.to_string_lossy()).as_ptr(),
            w::to_wide(&path.to_string_lossy()).as_ptr(), REPLACE | WRITE_THROUGH)
    };
    if ok == 0 {
        return Err(format!("替换文件失败（错误码 {}）：{}", unsafe { w::GetLastError() }, path.display()));
    }
    Ok(())
}

/// 生成开机启动脚本到程序目录（纯绿色，不写注册表/系统目录）。
/// 使用运行时真实 exe 文件名（用户可能重命名过），%~dp0 相对定位避免路径编码问题。
/// bat 按系统 ANSI 代码页（中文系统=GBK）写：cmd 默认按该代码页读取批处理，
/// 中文文件名才能被正确解析（UTF-8 写会在 cmd 里乱码导致启动失败）。
/// 启动文件的公共文件名主干：<真实程序名（不含扩展名）>开机启动-请复制到启动文件夹
pub fn startup_file_stem() -> String {
    let exe = exe_path();
    let stem = exe.file_stem().and_then(|s| s.to_str()).unwrap_or("EcoPilot");
    format!("{}开机启动-请复制到启动文件夹", stem)
}

pub fn generate_startup_script() -> Result<PathBuf, String> {
    let path = app_dir().join(format!("{}.bat", startup_file_stem()));
    let exe = exe_path();
    let exe_name = exe.file_name().and_then(|s| s.to_str()).unwrap_or("EcoPilot.exe");
    let content = format!(
        "@echo off\r\n\
         rem Copy this file to the Startup folder to run EcoPilot at logon.\r\n\
         rem Startup folder: Win+R then type shell:startup\r\n\
         start \"\" \"%~dp0{}\" --apply --tray\r\n",
        exe_name
    );
    // 按 ANSI 字节写，匹配 cmd 的默认代码页
    std::fs::write(&path, w::to_ansi(&content)).map_err(|e| format!("生成启动脚本失败：{}", e))?;
    Ok(path)
}

/// 生成开机启动快捷方式（.lnk）到程序目录，带 --apply --tray 参数，
/// 指向运行时真实 exe 路径。与 bat 一起提供给用户复制到启动文件夹。
pub fn generate_startup_shortcut() -> Result<PathBuf, String> {
    let path = app_dir().join(format!("{}.lnk", startup_file_stem()));
    let exe = exe_path();
    let exe_s = exe.to_string_lossy().to_string();
    let dir = exe.parent().unwrap_or(Path::new(".")).to_string_lossy().to_string();
    let desc = format!("自由效能进程EcoPilot v1.58 - 静默应用效能规则");
    unsafe {
        w::create_shortcut(&exe_s, "--apply --tray", &dir, &desc, "", &path.to_string_lossy())?;
    }
    Ok(path)
}

pub fn load_config(path: &Path) -> Result<Config, String> {
    if !path.exists() {
        return Ok(Config::default());
    }
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("读取配置失败：{}", e))?;
    let root = mini_json::parse(&text).map_err(|e| format!("配置格式错误：{}", e))?;
    Config::from_json(&root)
}

pub fn save_config(path: &Path, config: &Config) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败：{}", e))?;
        }
    }
    atomic_write(path, &config.to_json().stringify())
}

pub fn exe_path() -> PathBuf {
    let mut buffer = vec![0u16; 32768];
    let len = unsafe { w::GetModuleFileNameW(std::ptr::null_mut(), buffer.as_mut_ptr(), buffer.len() as u32) };
    PathBuf::from(if len > 0 {
        String::from_utf16_lossy(&buffer[..len as usize])
    } else {
        "EcoPilot.exe".to_string()
    })
}

/// 一次性清理 v1/v2 遗留的注册表自启项，保证纯绿色。
/// v3 只在启动时调用一次；之后永远不会再写注册表。
pub fn cleanup_legacy_registry_autostart() {
    const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    const RUN_VALUE: &str = "EcoPilot";
    unsafe {
        let mut key: w::HKEY = std::ptr::null_mut();
        if w::RegOpenKeyExW(w::HKEY_CURRENT_USER, w::to_wide(RUN_KEY).as_ptr(),
            0, 0x0000_0002, &mut key) == 0 { // KEY_SET_VALUE
            w::RegDeleteValueW(key, w::to_wide(RUN_VALUE).as_ptr());
            w::RegCloseKey(key);
        }
    }
}

/// 纯绿色：开机启动靠程序目录里的脚本，不写注册表。
/// 这里只返回脚本是否存在（用户手动放到启动文件夹后才真正生效）。
pub fn startup_script_path() -> PathBuf {
    app_dir().join(format!("{}.bat", startup_file_stem()))
}

pub fn is_autostart() -> bool {
    startup_script_path().exists()
}

pub fn set_autostart(enable: bool) -> Result<(), String> {
    if enable {
        generate_startup_script().map(|_| ())
    } else {
        let path = startup_script_path();
        if path.exists() {
            std::fs::remove_file(&path).map_err(|e| format!("删除启动脚本失败：{}", e))?;
        }
        Ok(())
    }
}

/// 对当前进程自己做一次完整的开关回环，验证 EcoQoS API 真实可用。
pub fn selftest() -> Result<String, String> {
    let pid = unsafe { w::GetCurrentProcessId() };
    let before = process_status(pid).ok_or_else(|| "无法读取自身进程状态".to_string())?;
    let mut lines = vec![
        format!("自检进程：PID {}，原优先级 {}，原 EcoQoS {:?}", pid, priority_name(before.priority), before.eco),
    ];

    let on = set_efficiency(pid, true, before.priority)?;
    lines.push(on.clone());
    let state_on = process_status(pid).ok_or_else(|| "开启后无法回读状态".to_string())?;
    if state_on.priority != w::IDLE_PRIORITY_CLASS || !state_on.is_eco() {
        let restore = set_efficiency(pid, false, before.priority).unwrap_or_default();
        return Err(format!("开启效率模式后回读不符：优先级 {}，EcoQoS {:?}；{}",
            priority_name(state_on.priority), state_on.eco, restore));
    }
    lines.push(format!("回读确认：优先级 {}，EcoQoS 已生效", priority_name(state_on.priority)));

    let off = set_efficiency(pid, false, before.priority)?;
    lines.push(off);
    let state_off = process_status(pid).ok_or_else(|| "恢复后无法回读状态".to_string())?;
    if state_off.is_eco() && before.eco == Some(false) {
        return Err(format!("恢复后 EcoQoS 仍未关闭：{:?}", state_off.eco));
    }
    lines.push(format!("回读确认：优先级 {}，EcoQoS {:?}",
        priority_name(state_off.priority), state_off.eco));

    let count = list_processes().len();
    lines.push(format!("进程枚举：成功列出 {} 个进程", count));
    Ok(lines.join("\n"))
}
