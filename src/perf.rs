//! 进程性能采样：CPU 使用率、内存、磁盘字节速率、网络连接数。
//! 全部来自真实 Win32 API，跨刷新保存上一帧做差值计算。

use crate::winapi as w;
use std::collections::HashMap;
use std::ffi::c_void;

#[derive(Clone, Default)]
pub struct Sample {
    pub cpu_ticks: u64,
    pub disk_bytes: u64,
    pub stamp: u64,
}

#[derive(Clone, Default)]
pub struct Metrics {
    pub cpu_percent: f64,
    pub memory_mb: f64,
    pub disk_kb_s: f64,
    pub connections: usize,
}

pub struct Sampler {
    previous: HashMap<u32, Sample>,
    last_stamp: u64,
    cpu_count: u32,
    connections: HashMap<u32, usize>,
}

impl Sampler {
    pub fn new() -> Self {
        let mut info: w::SYSTEM_INFO = unsafe { std::mem::zeroed() };
        unsafe { w::GetSystemInfo(&mut info); }
        Self {
            previous: HashMap::new(),
            last_stamp: unsafe { w::GetTickCount64() },
            cpu_count: info.dwNumberOfProcessors.max(1),
            connections: HashMap::new(),
        }
    }

    /// 刷新前先采集连接表（一次采集全机），再逐进程算指标。
    pub fn poll_connections(&mut self) {
        let mut map: HashMap<u32, usize> = HashMap::new();
        let mut size = 0u32;
        unsafe {
            w::GetExtendedTcpTable(
                std::ptr::null_mut(), &mut size, 0, w::AF_INET, w::TCP_TABLE_OWNER_PID_ALL, 0,
            );
        }
        if size > 0 {
            let mut buffer = vec![0u8; size as usize];
            let rc = unsafe {
                w::GetExtendedTcpTable(
                    buffer.as_mut_ptr() as *mut c_void, &mut size, 0,
                    w::AF_INET, w::TCP_TABLE_OWNER_PID_ALL, 0,
                )
            };
            if rc == 0 {
                let table = buffer.as_ptr() as *const w::MIB_TCPTABLE_OWNER_PID;
                let count = unsafe { (*table).dwNumEntries } as usize;
                for i in 0..count.min(20000) {
                    let row = unsafe { &*((*table).table.as_ptr().add(i)) };
                    *map.entry(row.dwOwningPid).or_insert(0) += 1;
                }
            }
        }
        let mut size = 0u32;
        unsafe {
            w::GetExtendedUdpTable(
                std::ptr::null_mut(), &mut size, 0, w::AF_INET, w::UDP_TABLE_OWNER_PID, 0,
            );
        }
        if size > 0 {
            let mut buffer = vec![0u8; size as usize];
            let rc = unsafe {
                w::GetExtendedUdpTable(
                    buffer.as_mut_ptr() as *mut c_void, &mut size, 0,
                    w::AF_INET, w::UDP_TABLE_OWNER_PID, 0,
                )
            };
            if rc == 0 {
                let table = buffer.as_ptr() as *const w::MIB_UDPTABLE_OWNER_PID;
                let count = unsafe { (*table).dwNumEntries } as usize;
                for i in 0..count.min(20000) {
                    let row = unsafe { &*((*table).table.as_ptr().add(i)) };
                    *map.entry(row.dwOwningPid).or_insert(0) += 1;
                }
            }
        }
        self.connections = map;
    }

    pub fn sample(&mut self, handle: w::HANDLE, pid: u32) -> Metrics {
        let mut metrics = Metrics {
            connections: self.connections.get(&pid).copied().unwrap_or(0),
            ..Metrics::default()
        };
        if handle.is_null() {
            return metrics;
        }

        let mut mem: w::PROCESS_MEMORY_COUNTERS = unsafe { std::mem::zeroed() };
        mem.cb = std::mem::size_of::<w::PROCESS_MEMORY_COUNTERS>() as u32;
        if unsafe { w::GetProcessMemoryInfo(handle, &mut mem, mem.cb) } != 0 {
            metrics.memory_mb = mem.WorkingSetSize as f64 / (1024.0 * 1024.0);
        }

        let mut creation = w::FILETIME::default();
        let mut exit = w::FILETIME::default();
        let mut kernel = w::FILETIME::default();
        let mut user = w::FILETIME::default();
        let mut io: w::IO_COUNTERS = unsafe { std::mem::zeroed() };
        let now = unsafe { w::GetTickCount64() };
        let elapsed_ms = now.saturating_sub(self.last_stamp);
        let got_times = unsafe {
            w::GetProcessTimes(handle, &mut creation, &mut exit, &mut kernel, &mut user)
        } != 0;
        let got_io = unsafe { w::GetProcessIoCounters(handle, &mut io) } != 0;

        if got_times && got_io && elapsed_ms >= 100 {
            let cpu_ticks = kernel.ticks() + user.ticks();
            let disk_bytes = io.ReadTransferCount + io.WriteTransferCount;
            if let Some(prev) = self.previous.get(&pid) {
                let cpu_delta = cpu_ticks.saturating_sub(prev.cpu_ticks);
                let cpu_total_ms = elapsed_ms * self.cpu_count as u64;
                if cpu_total_ms > 0 {
                    // FILETIME 100ns -> ms：除以 10000；再除以所有核的总毫秒数得到百分比
                    metrics.cpu_percent = (cpu_delta as f64 / 10000.0) / cpu_total_ms as f64 * 100.0;
                }
                let disk_delta = disk_bytes.saturating_sub(prev.disk_bytes);
                metrics.disk_kb_s = disk_delta as f64 / 1024.0 / (elapsed_ms as f64 / 1000.0);
            }
            self.previous.insert(pid, Sample { cpu_ticks, disk_bytes, stamp: now });
        } else if !got_times || !got_io {
            self.previous.remove(&pid);
        }

        metrics
    }

    pub fn finish_cycle(&mut self) {
        self.last_stamp = unsafe { w::GetTickCount64() };
        if self.previous.len() > 2000 {
            self.previous.clear();
        }
    }
}

pub fn format_cpu(value: f64) -> String {
    if value <= 0.0 { "0%".to_string() } else { format!("{:.1}%", value.min(999.0)) }
}

pub fn format_memory(mb: f64) -> String {
    if mb >= 1024.0 { format!("{:.1} GB", mb / 1024.0) } else { format!("{:.1} MB", mb) }
}

pub fn format_disk(kb_s: f64) -> String {
    if kb_s <= 0.0 { "0 MB/s".to_string() }
    else if kb_s >= 1024.0 { format!("{:.1} MB/s", kb_s / 1024.0) }
    else { format!("{:.0} KB/s", kb_s) }
}
