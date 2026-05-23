#!/usr/bin/env sh
set -e
OUT_DIR=src-tauri/binaries
URL=https://aamrp.iamgabriel.dev/binaries/ffmpeg-aarch64-apple-darwin
OUT_FILE="$OUT_DIR/ffmpeg-aarch64-apple-darwin"

mkdir -p "$OUT_DIR"
echo "Downloading $URL to $OUT_FILE"
if command -v curl >/dev/null 2>&1; then
  curl -L -o "$OUT_FILE" "$URL"
elif command -v wget >/dev/null 2>&1; then
  wget -O "$OUT_FILE" "$URL"
else
  echo "Error: curl or wget is required to download ffmpeg." >&2
  exit 1
fi

chmod +x "$OUT_FILE"
echo "Done."
