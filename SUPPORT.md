Application Support

Thank you for using rTemplate (Rust TUI Application Template)! If you are experiencing issues, follow these steps to get help.

Run rTemplate Doctor Self-Healing Diagnostics

Before filing an issue, check if the diagnostic command can detect configuration or environmental problems. Open your terminal and run:

rtemplate doctor

Check the Logs

rTemplate logs events and diagnostics to a background log file.

Log Location: %APPDATA%\rTmp\log.txt
How to open in PowerShell:
notepad "$env:APPDATA\rTmp\log.txt"

Open an Issue

If the doctor tool did not resolve your issue, please open an issue in the official repository.

What to include:
Your Windows version (e.g. Windows 11 23H2).
The terminal environment you are using.
The relevant output or error logs from %APPDATA%\rTmp\log.txt.
