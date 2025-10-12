@echo off
set "INSTALL_DIR=%APPDATA%\prtgn\"
set "BIN_DIR=%INSTALL_DIR%\bin"

title PRTGN Install Script -- Windows
echo --
echo .PRTGN
echo A protogen inspired file extension written in Rust.
echo --
echo .prtgn  Copyright (C) 2025  ExoticDG
echo This program comes with ABSOLUTELY NO WARRANTY.
echo This is free software, and you are welcome to redistribute it
echo under certain conditions.
echo --
echo Licensed under the GNU General Public License v3.0
echo --
echo --
echo --
echo ENSURE the prtgn executable is in the working directory. I.E. The same folder as this script s ruining in.
echo --
echo --
pause

echo Installing YourProgramName to %INSTALL_DIR%...

:: Create directory if it doesn't exist
if not exist "%BIN_DIR%" mkdir "%BIN_DIR%"

:: Copy your compiled Rust executable (e.g., your_program.exe)
copy /Y "prtgn.exe" "%BIN_DIR%"

echo Adding %BIN_DIR% to your User PATH...
:: setx updates the user's permanent environment variables in the Registry.
:: /M would be for System PATH, but User PATH is generally safer and doesn't require administrator rights.

:: Check if path is already set (basic check, not perfect)
setx PATH "%%PATH%%;%BIN_DIR%"

echo.
echo Installation complete! You may need to open a new Command Prompt or PowerShell window to use the command.
pause