#!/usr/bin/env bash
set -euo pipefail

VERSION="0.1.0"
BINARY_NAME="owlguard-mcp"
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST_DIR="$PROJECT_ROOT/dist"

rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

echo "编译 Release 版本..."
cd "$PROJECT_ROOT"
cargo build --release

echo "创建 Linux/macOS 发布包..."

for TARGET in "x86_64-unknown-linux-gnu" "x86_64-apple-darwin" "aarch64-apple-darwin"; do
    case "$TARGET" in
        "x86_64-unknown-linux-gnu")
            SUFFIX="linux-amd64"
            BINARY_PATH="target/$TARGET/release/$BINARY_NAME"
            echo "交叉编译 $TARGET..."
            cargo build --release --target "$TARGET" 2>/dev/null || continue
            ;;
        "x86_64-apple-darwin")
            SUFFIX="darwin-amd64"
            BINARY_PATH="target/$TARGET/release/$BINARY_NAME"
            echo "交叉编译 $TARGET..."
            cargo build --release --target "$TARGET" 2>/dev/null || continue
            ;;
        "aarch64-apple-darwin")
            SUFFIX="darwin-arm64"
            BINARY_PATH="target/$TARGET/release/$BINARY_NAME"
            echo "交叉编译 $TARGET..."
            cargo build --release --target "$TARGET" 2>/dev/null || continue
            ;;
    esac

    if [ -f "$BINARY_PATH" ]; then
        PKG_DIR="$DIST_DIR/$BINARY_NAME-$VERSION-$SUFFIX"
        mkdir -p "$PKG_DIR"
        cp "$BINARY_PATH" "$PKG_DIR/"
        chmod +x "$PKG_DIR/$BINARY_NAME"
        cp -r "$PROJECT_ROOT/rules" "$PKG_DIR/"
        cp "$PROJECT_ROOT/README.md" "$PKG_DIR/"
        cp "$PROJECT_ROOT/LICENSE" "$PKG_DIR/"

        cd "$DIST_DIR"
        tar -czf "$BINARY_NAME-$VERSION-$SUFFIX.tar.gz" -C "$PKG_DIR" .
        rm -rf "$PKG_DIR"
        cd "$PROJECT_ROOT"
        echo "  已创建: $BINARY_NAME-$VERSION-$SUFFIX.tar.gz"
    fi
done

echo ""
echo "发布包已创建:"
ls -lh "$DIST_DIR"

echo ""
echo "下一步操作:"
echo "1. 在 GitHub 创建 Release v$VERSION"
echo "2. 上传 dist/ 目录中的压缩包"
echo "3. cd npm && npm publish --access public"
