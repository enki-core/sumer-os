#!/bin/bash
cd "$(dirname "$0")"

echo "==========================================="
echo "   Atlas Filemanager Live UI Previewer"
echo "==========================================="
echo "[+] Starting live UI preview..."
echo "[*] Edits to ui/app_window.slint reload automatically."

../../programs/slint-viewer/slint-viewer --auto-reload ui/app_window.slint

