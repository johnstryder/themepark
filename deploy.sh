#!/bin/bash
# Deploy themepark to production at mywebsitesareathemepark.com
set -e

cd "$(dirname "$0")"

echo "==> Building Leptos project..."
cargo leptos build --release

echo "==> Creating site directory..."
sudo mkdir -p /var/www/themepark
sudo cp -r target/site/* /var/www/themepark/
sudo chown -R www-data:www-data /var/www/themepark 2>/dev/null || sudo chown -R $(whoami):$(whoami) /var/www/themepark

echo "==> Installing nginx config..."
# Use conf.d (Arch default) - works without sites-available/sites-enabled setup
sudo cp nginx/nginx.conf /etc/nginx/conf.d/themepark.conf
# Remove old sites-enabled symlink if it exists (from previous deploy)
sudo rm -f /etc/nginx/sites-enabled/themepark 2>/dev/null || true
sudo nginx -t && sudo systemctl reload nginx

echo "==> Restarting themepark server..."
sudo systemctl restart themepark 2>/dev/null || true

echo ""
echo "Done! Site should be live at http://mywebsitesareathemepark.com"
echo "If systemd service isn't set up, run: ./target/release/server"
