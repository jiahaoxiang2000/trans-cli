# Language Codes Reference

This file contains the language codes supported by Baidu Translation API for use with trans-cli.

## Common Language Codes

| Code | Language |
|------|----------|
| `auto` | Auto-detect (default for source) |
| `en` | English |
| `zh` | Chinese (Simplified) |
| `zh-tw` | Chinese (Traditional) |
| `es` | Spanish |
| `fr` | French |
| `de` | German |
| `ja` | Japanese |
| `ko` | Korean |
| `ru` | Russian |
| `ar` | Arabic |
| `pt` | Portuguese |
| `it` | Italian |
| `nl` | Dutch |
| `sv` | Swedish |
| `da` | Danish |
| `no` | Norwegian |
| `fi` | Finnish |
| `th` | Thai |
| `vi` | Vietnamese |
| `hi` | Hindi |

## Usage Examples

```bash
# Auto-detect source language, translate to Chinese (default)
trans-cli "Hello World"

# English to Spanish
trans-cli "Hello World" --from en --to es

# Chinese to English
trans-cli "你好世界" --from zh --to en

# Auto-detect to Japanese
trans-cli "Good morning" --to ja

# French to German
trans-cli "Bonjour le monde" --from fr --to de
```

## Notes

- Use `auto` as the source language to enable automatic language detection
- The default target language is Chinese (`zh`) if not specified
- Some language combinations may not be supported
- Refer to [Baidu API documentation](https://api.fanyi.baidu.com/doc/21) for the complete and up-to-date list

## Error Handling

If you encounter errors:
1. Check your API credentials in the config file
2. Verify the language codes are supported
3. Ensure you have internet connectivity
4. Check if you've exceeded API rate limits