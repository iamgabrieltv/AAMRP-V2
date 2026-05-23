@echo off
setlocal enableextensions enabledelayedexpansion
set "OUT_DIR=src-tauri\binaries"
set "URL=https://aamrp.iamgabriel.dev/binaries/ffmpeg-x86_64-pc-windows-msvc.exe"
set "OUT_FILE=%OUT_DIR%\ffmpeg-x86_64-pc-windows-msvc.exe"
if not exist "%OUT_DIR%" md "%OUT_DIR%"
echo Downloading %URL% to %OUT_FILE%
powershell -NoProfile -Command "try { Invoke-WebRequest -Uri '%URL%' -OutFile '%OUT_FILE%' -UseBasicParsing; exit 0 } catch { Write-Error $_; exit 1 }"
if errorlevel 1 (
    echo Download failed.
    exit /b 1
)
echo Done.
