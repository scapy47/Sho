#!/usr/bin/env sh
set -e

OWNER="Scapy47"
REPO="Shio"
BASE_URL="https://github.com/$OWNER/$REPO/releases/latest/download"

case "$(uname)" in
  Darwin) OS="macOS" ;;
  Linux)  OS="Linux" ;;
  *)      echo "Unsupported OS"; exit 1 ;;
esac

FILENAME="shio-${OS}-x86_64"

while true; do
    printf "Try shio before installation? (!! Run directly !!) (y/n): "
    read choice

    case "$choice" in
        y|Y)
            TMP_DIR=$(mktemp -d)
            curl -fL -o "$TMP_DIR/shio" "$BASE_URL/$FILENAME" || { echo "Download failed"; exit 1; }
            chmod +x "$TMP_DIR/shio"
            "$TMP_DIR/shio" "$@"
            break
            ;;
        n|N)
            exit
            ;;
        *)
            echo "Please answer y or n."
            ;;
    esac
done

INSTALL_DIR="${XDG_BIN_HOME:-$HOME/.local/bin}"
FINAL_PATH="$INSTALL_DIR/shio"

mkdir -p "$INSTALL_DIR"

echo "Downloading to $FINAL_PATH"
curl -fL -o "$FINAL_PATH" "$BASE_URL/$FILENAME" || { echo "Download failed"; exit 1; }
chmod +x "$FINAL_PATH"

echo "Installed to $FINAL_PATH"

case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    echo ""
    echo "Warning: $INSTALL_DIR is not in your PATH"
    echo "Add to your shell config (~/.bashrc, ~/.zshrc, etc.):"
    echo '  export PATH="$HOME/.local/bin:$PATH"'
    ;;
esac

echo "Run 'shio --version' to verify."
echo 'To enable playback, add to your shell config:'
echo '  export SHIO_PLAYER_CMD="mpv --user-agent={user_agent} --http-header-fields=\"Referer: {referer}\" {url}"'
