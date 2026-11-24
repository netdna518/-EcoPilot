use std::path::Path;
use std::process::Command;

/// 用 windres 把 app.rc（图标 + 版本资源）编成 COFF 对象并链接进 exe。
/// 让「属性 → 详细信息」显示文件版本/产品名称/版权，并带自定义图标。
/// windres 查找顺序：WINDRES 环境变量 → PATH → 常见 MinGW 安装路径。
/// windres 内部调用 gcc 预处理，中文路径会乱码，故先复制到纯 ASCII 临时目录。
fn main() {
    println!("cargo:rerun-if-changed=app.rc");
    println!("cargo:rerun-if-changed=app.ico");
    if !cfg!(target_os = "windows") { return; }

    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    if manifest.is_empty() { return; }

    // 纯 ASCII 工作目录，规避 gcc 命令行的中文路径编码问题
    let work = std::env::temp_dir().join("ecopilot_build_res");
    if std::fs::create_dir_all(&work).is_err() { return; }
    let rc_work = work.join("app.rc");
    let ico_work = work.join("app.ico");
    if std::fs::copy(Path::new(&manifest).join("app.rc"), &rc_work).is_err()
        || std::fs::copy(Path::new(&manifest).join("app.ico"), &ico_work).is_err() {
        println!("cargo:warning=复制资源文件失败，跳过图标/版本资源");
        return;
    }

    // 找 windres：环境变量优先，其次 PATH，再试常见安装位置
    let mut candidates: Vec<String> = Vec::new();
    if let Ok(w) = std::env::var("WINDRES") { candidates.push(w); }
    candidates.push("windres.exe".into());
    candidates.push("windres".into());
    for p in [
        r"C:\msys64\mingw64\bin\windres.exe",
        r"C:\mingw64\bin\windres.exe",
        r"C:\Program Files\mingw-w64\x86_64-8.1.0-posix-seh-rt_v6-rev0\mingw64\bin\windres.exe",
    ] { candidates.push(p.into()); }

    let mut ok = false;
    for wr in &candidates {
        let out = Command::new(wr)
            .current_dir(&work)
            .arg("-i").arg("app.rc")
            .arg("-o").arg("app_res.o")
            .arg("--input-format=rc").arg("--output-format=coff")
            .output();
        match out {
            Ok(o) if o.status.success() => { ok = true; break; }
            _ => continue,
        }
    }
    if !ok {
        println!("cargo:warning=未找到 windres，跳过图标/版本资源（可设置 WINDRES 环境变量）");
        return;
    }
    let obj = work.join("app_res.o");
    println!("cargo:rustc-link-arg={}", obj.to_string_lossy());
}
