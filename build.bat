@echo off
setlocal
cd /d "%~dp0"

rem cargo 需在 PATH 中（安装 Rust 时通常已自动加入）。
rem 图标/版本资源由 build.rs 用 windres 编译；找不到 windres 时可：
rem   set WINDRES=C:\path\to\windres.exe
rem 然后重新运行本脚本。

cargo build --release
if errorlevel 1 goto fail
copy /Y target\release\EcoPilot.exe EcoPilot.exe >nul
if errorlevel 1 goto fail
echo OK: EcoPilot.exe
goto end
:fail
echo BUILD FAILED
exit /b 1
:end
endlocal
