# zhurl

非官方知乎API收集整理，可用于个人学习、AI Agent、信息收集、爬虫等用途。

安装方法：
```bash
cargo install zhurl
```

它从`~/.zhihu-plus-plus/account.json`读取知乎的cookie。这个cookie文件可以通过在电脑端登录[知乎++](https://github.com/zly2006/zhihu-plus-plus)自动生成。

## 用法

```bash
zhurl [--web|--android] [--jq FILTER] [-X METHOD] [-H 'Name: value'] [-d DATA] [-i] [-o FILE] URL
```

示例:

```bash
cargo run -- --jq '{id, name}' 'https://www.zhihu.com/api/v4/me'
cargo run -- --android --jq '{id, name}' 'https://www.zhihu.com/api/v4/me'
```

## API 参考

常用知乎 API 已按主题整理在 [docs/apis.md](docs/apis.md)，每个接口包含模式、登录和签名要求、参数、`zhurl` 示例和脱敏返回示例。

## Output

- When stdout is a TTY, JSON output is pretty-printed and colorized by default.
- Piped output and `-o/--output` stay uncolored and machine-oriented.
- `-i/--include` disables JSON formatting because response headers are not JSON.

## Differences from curl/jq

- `zhurl` does not execute system `curl` or system `jq`.
- HTTP is implemented with `ureq`; `--jq` is implemented with embedded `jaq`.
- Only the listed API-oriented options are supported.
- Supported methods are `GET`, `POST`, `PUT`, `PATCH`, `DELETE`, `HEAD`,
  `OPTIONS`, and `TRACE`.
- There is no curl config, `.netrc`, cookie jar, proxy flags, multipart upload,
  retry flags, HTTP/2 controls, or shell pipeline behavior.
- `4xx`/`5xx` responses are printed like `curl` without `--fail`.
- `--jq` accepts jaq-compatible jq filters for JSON responses, not every jq CLI
  flag or module-loading feature.
- `--jq` cannot be combined with `-i/--include` because response headers are not
  JSON.
