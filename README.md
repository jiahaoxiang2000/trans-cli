# Trans CLI

A command-line translation tool using Baidu Translation API, written in Rust.

## Features

- Fast and reliable text translation
- Support for multiple languages
- Automatic language detection
- Simple CLI interface
- Secure credential management

## Installation

1. Clone this repository
2. Build the project:
   ```bash
   cargo build --release
   ```

## Configuration

1. Get your API credentials from [Baidu Fanyi API](https://fanyi-api.baidu.com/)
2. Run the tool once to generate config file:
   ```bash
   cargo run -- "hello world"
   ```
3. Edit the config file with your credentials:
   ```bash
   # Show config file location
   cargo run -- --config
   ```

The config file should look like:
```toml
appid = "your_app_id"
appkey = "your_app_key"
```

## Usage

### Basic translation
```bash
# Translate to Chinese (default)
cargo run -- "Hello World"

# Translate from English to Spanish
cargo run -- "Hello World" --from en --to es

# Auto-detect source language
cargo run -- "你好世界" --to en
```

### Language Codes

Common language codes:
- `auto` - Auto-detect (default for source)
- `en` - English
- `zh` - Chinese (Simplified)
- `zh-tw` - Chinese (Traditional)
- `es` - Spanish
- `fr` - French
- `de` - German
- `ja` - Japanese
- `ko` - Korean
- `ru` - Russian

For complete list, refer to [Baidu API documentation](https://api.fanyi.baidu.com/doc/21).

### Command Options

```bash
trans-cli [OPTIONS] <TEXT>

Arguments:
  <TEXT>  Text to translate

Options:
  -f, --from <LANG>  Source language code (default: auto)
  -t, --to <LANG>    Target language code (default: zh)
      --config       Show config file path and exit
  -h, --help         Print help
  -V, --version      Print version
```

## Examples

```bash
# English to Chinese
cargo run -- "Hello, how are you?"

# Chinese to English
cargo run -- "你好吗？" --to en

# Auto-detect to Spanish
cargo run -- "Good morning" --to es

# Japanese to English
cargo run -- "おはよう" --from ja --to en
```

## Error Handling

The tool provides clear error messages for common issues:
- Missing or invalid API credentials
- Network connectivity problems
- API rate limiting
- Unsupported language codes

## Development

### Building
```bash
cargo build
```

### Running tests
```bash
cargo test
```

### Release build
```bash
cargo build --release
```

## License

This project is open source. Please refer to the LICENSE file for details.

## API Reference

This tool uses the Baidu Translation API. For more information:
- [API Documentation](https://api.fanyi.baidu.com/doc/21)
- [Language Codes](https://api.fanyi.baidu.com/doc/21)
- [Error Codes](https://api.fanyi.baidu.com/doc/21)