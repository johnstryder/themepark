#!/bin/bash
# Deploy themepark to production at mywebsitesareathemepark.com
set -e

cd "$(dirname "$0")"

echo "==> Building Leptos project..."
cargo leptos build --release

echo "==> Deploying (requires sudo)..."
if sudo -n true 2>/dev/null; then
    sudo mkdir -p /var/www/themepark
    sudo cp -r target/site/* /var/www/themepark/
    sudo chown -R www-data:www-data /var/www/themepark 2>/dev/null || sudo chown -R $(whoami):$(whoami) /var/www/themepark
    sudo cp nginx/nginx.conf /etc/nginx/conf.d/themepark.conf
    sudo rm -f /etc/nginx/sites-enabled/themepark 2>/dev/null || true
    sudo cp themepark.service /etc/systemd/system/ 2>/dev/null || true
    sudo systemctl daemon-reload 2>/dev/null || true
    sudo nginx -t && sudo systemctl reload nginx
    sudo systemctl enable themepark 2>/dev/null || true
    sudo systemctl restart themepark 2>/dev/null || true
    echo ""
    echo "Done! Site should be live at http://mywebsitesareathemepark.com"
else
    echo ""
    echo "Build complete. To finish deployment (requires sudo), run:"
    echo "  sudo ./deploy-install.sh"
    echo ""
    echo "Or start the server manually: ./target/release/server"
fi
