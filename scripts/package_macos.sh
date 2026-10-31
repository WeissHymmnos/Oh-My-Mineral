#!/usr/bin/env bash
# Build and package Mineral GUI + bundled Mineral Daemon into a macOS .app bundle and .dmg
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST="$ROOT/dist"
APP_NAME="Mineral"
PKG_VERSION="0.1.0"
APP_DIR="$DIST/${APP_NAME}.app"
DMG_OUT="$DIST/${APP_NAME}-${PKG_VERSION}-macOS.dmg"

cd "$ROOT"

echo "==> Building Mineral GUI workspace (--release) for macOS..."
cargo build -p mineral-gui-qt -p mineral-gui-bridge --release

MINERAL_BIN="${MINERAL_BIN:-$(command -v mineral || echo "$HOME/.cargo/bin/mineral")}"
if [[ ! -f "$MINERAL_BIN" ]]; then
  echo "Installing mineral 0.5.10 daemon via cargo..."
  cargo install mineral --version 0.5.10
  MINERAL_BIN="$HOME/.cargo/bin/mineral"
fi

rm -rf "$APP_DIR"
mkdir -p "$APP_DIR/Contents/MacOS" "$APP_DIR/Contents/Resources"

cp "$ROOT/target/release/mineral-gui-qt" "$APP_DIR/Contents/MacOS/mineral-gui-qt"
cp "$ROOT/target/release/mineral-gui-bridge" "$APP_DIR/Contents/MacOS/mineral-gui-bridge"
cp "$MINERAL_BIN" "$APP_DIR/Contents/MacOS/mineral"
chmod +x "$APP_DIR/Contents/MacOS/"*

# Generate .icns icon from assets/logo.png
ICONSET="$DIST/Mineral.iconset"
rm -rf "$ICONSET"
mkdir -p "$ICONSET"
for sz in 16 32 128 256 512; do
  sips -z $sz $sz "$ROOT/assets/logo.png" --out "$ICONSET/icon_${sz}x${sz}.png" >/dev/null
  sz2=$((sz * 2))
  sips -z $sz2 $sz2 "$ROOT/assets/logo.png" --out "$ICONSET/icon_${sz}x${sz}@2x.png" >/dev/null
done
iconutil -c icns "$ICONSET" -o "$APP_DIR/Contents/Resources/mineral-gui.icns"
rm -rf "$ICONSET"

cat > "$APP_DIR/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key>
  <string>Mineral</string>
  <key>CFBundleDisplayName</key>
  <string>Mineral Music Player</string>
  <key>CFBundleIdentifier</key>
  <string>org.mineral.gui</string>
  <key>CFBundleVersion</key>
  <string>${PKG_VERSION}</string>
  <key>CFBundleShortVersionString</key>
  <string>${PKG_VERSION}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleExecutable</key>
  <string>mineral-gui-qt</string>
  <key>CFBundleIconFile</key>
  <string>mineral-gui.icns</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
EOF

if command -v macdeployqt >/dev/null 2>&1; then
  echo "==> Running macdeployqt to bundle Qt 6 frameworks..."
  macdeployqt "$APP_DIR"
fi

echo "==> Creating macOS disk image: $DMG_OUT"
rm -f "$DMG_OUT"
hdiutil create -volname "$APP_NAME" -srcfolder "$APP_DIR" -ov -format UDZO "$DMG_OUT"
echo "==> Done! Output: $DMG_OUT"
