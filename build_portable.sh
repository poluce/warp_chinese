#!/bin/bash
# Warp 便携版一键编译脚本
set -e

# 1. 设置 PATH（protoc, cmake, jq）
export PATH="/c/Users/13491/AppData/Local/Microsoft/WinGet/Packages/Google.Protobuf_Microsoft.Winget.Source_8wekyb3d8bbwe/bin:$PATH"
export PATH="/d/application_install/Visual Studio/2022/Common7/IDE/CommonExtensions/Microsoft/CMake/CMake/bin:$PATH"
export PATH="/c/Users/13491/AppData/Local/Microsoft/WinGet/Packages/jqlang.jq_Microsoft.Winget.Source_8wekyb3d8bbwe/bin:$PATH"

# 2. 设置 Cargo 完整 profile 名（build script 需要）
export CARGO_FULL_PROFILE=rlto

# 3. 编译
cd "$(dirname "$0")"
cargo build -p warp --profile rlto --bin warp-oss --features "release_bundle,gui" --target x86_64-pc-windows-msvc

# 4. 打包便携文件夹
PORTABLE_DIR="$(pwd)/portable_warp"
mkdir -p "$PORTABLE_DIR/x64"

cp -f target/x86_64-pc-windows-msvc/rlto/warp-oss.exe "$PORTABLE_DIR/"
cp -f target/rlto/conpty.dll "$PORTABLE_DIR/"
cp -f target/rlto/dxcompiler.dll "$PORTABLE_DIR/"
cp -f target/rlto/dxil.dll "$PORTABLE_DIR/"
cp -f target/rlto/x64/OpenConsole.exe "$PORTABLE_DIR/x64/"
cp -f app/assets/windows/x64/vcruntime140.dll "$PORTABLE_DIR/"
cp -f app/assets/windows/x64/vcruntime140_1.dll "$PORTABLE_DIR/"
cp -f app/assets/windows/x64/msvcp140.dll "$PORTABLE_DIR/"

echo ""
echo "============================================"
echo "  编译完成！便携文件夹: $PORTABLE_DIR"
echo "============================================"
du -sh "$PORTABLE_DIR"
