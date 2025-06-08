#!/bin/bash

# Trans CLI Test Script
echo "=== Trans CLI Translation Tool Test ==="
echo ""

echo "1. Showing help information:"
cargo run -- --help
echo ""

echo "2. Showing config file location:"
cargo run -- --config
echo ""

echo "3. Config file contents:"
echo "Current config file needs your Baidu API credentials:"
cat ~/.config/trans-cli/config.toml
echo ""

echo "4. To use the translation tool:"
echo "   - Get API credentials from: https://fanyi-api.baidu.com/"
echo "   - Edit the config file: ~/.config/trans-cli/config.toml"
echo "   - Replace 'INPUT_YOUR_APPID' and 'INPUT_YOUR_APPKEY' with your actual credentials"
echo ""

echo "5. Example usage (after configuring credentials):"
echo "   cargo run -- \"Hello World\"                    # Translate to Chinese"
echo "   cargo run -- \"Hello World\" --to es            # Translate to Spanish"
echo "   cargo run -- \"你好世界\" --to en                # Translate to English"
echo "   cargo run -- \"Bonjour\" --from fr --to en      # French to English"
echo ""

echo "6. Building release version:"
cargo build --release
echo "Release binary available at: target/release/trans-cli"
echo ""

echo "=== Test Complete ==="