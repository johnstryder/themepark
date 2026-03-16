#!/bin/bash
# Deploy themepark to production at mywebsitesareathemepark.com
# Runs without sudo: uses systemd user services instead of system services
set -e

cd "$(dirname "$0")"
PROJECT_ROOT="$(pwd)"

echo "==> Building Leptos project..."
cargo leptos build --release

echo "==> Installing themepark user systemd service..."
mkdir -p ~/.config/systemd/user
cat > ~/.config/systemd/user/themepark.service << EOF
[Unit]
Description=Themepark Axum/Leptos Server
After=network.target

[Service]
Type=simple
WorkingDirectory=$PROJECT_ROOT
ExecStart=$PROJECT_ROOT/target/release/server
Restart=on-failure
RestartSec=5
Environment="RUST_LOG=info"

[Install]
WantedBy=default.target
EOF

systemctl --user daemon-reload
loginctl enable-linger "$USER" 2>/dev/null || true  # allow user services at boot
systemctl --user enable themepark 2>/dev/null || true
systemctl --user restart themepark

echo ""
echo "Done! Site should be live at http://mywebsitesareathemepark.com"
echo ""
echo "Note: Nginx serves static files from target/site. If you get 403 on /pkg/,"
echo "ensure nginx can read the project: chmod o+x /home/$USER /home/$USER/themepark"
echo "Fresh setup: run once with sudo: sudo ./deploy-install.sh"
