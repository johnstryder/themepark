#!/bin/bash
# Run the deploy steps that require sudo (after ./deploy.sh has built the project)
# Usage: sudo ./deploy-install.sh
set -e

cd "$(dirname "$0")"

echo "==> Creating site directory..."
mkdir -p /var/www/themepark
cp -r target/site/* /var/www/themepark/
chown -R www-data:www-data /var/www/themepark 2>/dev/null || chown -R ${SUDO_USER:-root}:${SUDO_USER:-root} /var/www/themepark

echo "==> Installing nginx config..."
rm -f /etc/nginx/conf.d/themepark.conf 2>/dev/null || true
ln -sf "$(pwd)/nginx/nginx.conf" /etc/nginx/sites-enabled/themepark.conf
nginx -t && systemctl reload nginx

echo "==> Installing and restarting themepark systemd service..."
cp themepark.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable themepark 2>/dev/null || true
systemctl restart themepark 2>/dev/null || true

echo ""
echo "Done! Site should be live at http://mywebsitesareathemepark.com"
echo "If systemd service isn't running, start manually: ./target/release/server"
