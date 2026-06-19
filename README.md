# zhurl

`zhurl` is a small curl-like CLI for authenticated Zhihu API requests.
It is self-contained: HTTP is handled by `ureq`, `--jq` is handled by embedded
`jaq`, and web request signing is handled by `zhihu_sign = "=0.1.0"`.

It reads the Zhihu++ desktop account file from:

```text
~/.zhihu-plus-plus/account.json
```

The account must be logged in and contain `z_c0`. Web mode also requires `d_c0`.

## Usage

```bash
zhurl [--web|--android] [--jq FILTER] [-X METHOD] [-H 'Name: value'] [-d DATA] [-i] [-o FILE] URL
```

Examples:

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
