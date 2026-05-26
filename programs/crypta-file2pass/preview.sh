#!/bin/bash
cd "$(dirname "$0")"

echo "==========================================="
echo "   Crypta File2pass Live UI Previewer"
echo "==========================================="
echo "[+] Starting live UI preview..."
echo "[*] Edits to ui/app.slint reload automatically."

../../programs/slint-viewer/slint-viewer --auto-reload ui/app.slint
