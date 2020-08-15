#!/bin/bash

cd "$(dirname "${BASH_SOURCE[0]}")" || exit

if [ -z "$TELEGRAM_BOT_TOKEN" ]; then
  echo "\$TELEGRAM_BOT_TOKEN not defined! Exiting."
  exit 1
fi

echo "Building reqex bot"
cargo build --release
echo "Building complete"
echo ""

echo "Composing systemd service"
cat << EOF > reqex.service
[Unit]
Description=reqex telegram bot
After=network.target

[Service]
Environment=TELEGRAM_BOT_TOKEN=${TELEGRAM_BOT_TOKEN}
ExecStart=$(pwd)/target/release/reqex

[Install]
WantedBy=default.target
EOF
echo "systemd service written into $(pwd)/reqex.service"

echo ""
echo "Installing the service into /usr/local/lib/systemd/service"

sudo install -Dm 0644 -t /usr/local/lib/systemd/system/ reqex.service

sudo systemctl daemon-reload
