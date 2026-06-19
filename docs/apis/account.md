# 账号与登录 API

这些接口来自知乎++的账号校验、二维码登录和 token 刷新流程。当前
`zhurl` 的主要用途是带已有登录态请求接口，所以登录前接口更多用于协议观察。

## 获取当前账号

返回当前登录账号的基础资料和通知计数。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/me
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| include | Query | 否 | 可选字段选择器。知乎++通常不传。 |

zhurl 示例：

```bash
zhurl --web --jq '{id, name, url_token, notifications_count}' \
  'https://www.zhihu.com/api/v4/me'
```

脱敏返回示例：

```json
{
  "id": "person_hash_id",
  "name": "用户昵称",
  "url_token": "user-token",
  "user_type": "people",
  "avatar_url": "https://picx.zhimg.com/avatar.jpg",
  "notifications_count": 3
}
```

## 登录 UDID 预热

二维码登录前初始化网页登录上下文。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 否 |
| 是否需要 d_c0 签名 | 否 |

URL：

```text
https://www.zhihu.com/udid
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| body | JSON Body | 是 | 知乎++发送 `{}`。 |
| x-xsrftoken | Header | 通常需要 | 有 `_xsrf` cookie 时传其值。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/json;charset=UTF-8' \
  -H 'x-requested-with: fetch' \
  -d '{}' \
  'https://www.zhihu.com/udid'
```

脱敏返回示例：

```json
{
  "udid": "browser-device-id"
}
```

## 查询登录验证码状态

获取二维码登录页使用的验证码元信息。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 否 |
| 是否需要 d_c0 签名 | 否 |

URL：

```text
https://www.zhihu.com/api/v3/oauth/captcha/v2?type=captcha_sign_in
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| type | Query | 是 | 知乎++使用 `captcha_sign_in`。 |

zhurl 示例：

```bash
zhurl --web --jq '.' \
  'https://www.zhihu.com/api/v3/oauth/captcha/v2?type=captcha_sign_in'
```

脱敏返回示例：

```json
{
  "show_captcha": false,
  "captcha_type": "cn"
}
```

## 创建二维码登录令牌

创建二维码登录 token 和扫码链接。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 否 |
| 是否需要 d_c0 签名 | 否 |

URL：

```text
https://www.zhihu.com/api/v3/account/api/login/qrcode
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| body | JSON Body | 是 | 知乎++发送 `{}`。 |
| x-xsrftoken | Header | 通常需要 | 有 `_xsrf` cookie 时传其值。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/json;charset=UTF-8' \
  -H 'x-requested-with: fetch' \
  -d '{}' \
  'https://www.zhihu.com/api/v3/account/api/login/qrcode'
```

脱敏返回示例：

```json
{
  "expires_at": 1790000000000,
  "link": "https://www.zhihu.com/account/qrcode/scan?token=qr_token",
  "token": "qr_token"
}
```

## 轮询二维码扫码状态

轮询二维码 token，直到手机端确认登录或触发风控。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 否 |
| 是否需要 d_c0 签名 | 否；知乎++轮询时会带 `x-zse-93` |

URL：

```text
https://www.zhihu.com/api/v3/account/api/login/qrcode/{token}/scan_info
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| token | Path | 是 | 创建二维码登录令牌接口返回的 token。 |

zhurl 示例：

```bash
zhurl --web \
  -H 'x-zse-93: 101_3_3.0' \
  'https://www.zhihu.com/api/v3/account/api/login/qrcode/{token}/scan_info'
```

脱敏返回示例：

```json
{
  "status": 1,
  "login_status": "CONFIRMED",
  "cookie": "z_c0=REDACTED; Path=/; Domain=.zhihu.com",
  "user_id": "person_hash_id"
}
```

## 获取 refresh token

认证请求返回 `401` 后，知乎++会先取 refresh token。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 否 |

URL：

```text
https://www.zhihu.com/api/account/prod/token/refresh
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| z_c0 | Cookie | 是 | 当前登录 cookie。 |
| x-requested-with | Header | 是 | 知乎++发送 `fetch`。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/x-www-form-urlencoded;charset=UTF-8' \
  -H 'x-requested-with: fetch' \
  'https://www.zhihu.com/api/account/prod/token/refresh'
```

脱敏返回示例：

```json
{
  "refresh_token": "refresh_token_redacted",
  "expires_in": 2592000
}
```

## 刷新 access token

刷新 web OAuth token。知乎++会把表单 payload 做 zse-v4 加密；不要把明文表单直接发出去。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 否；使用 zse-v4 加密请求体 |

URL：

```text
https://www.zhihu.com/api/v3/oauth/sign_in
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| client_id | 加密表单 Body | 是 | 知乎++使用的 web client id。 |
| grant_type | 加密表单 Body | 是 | `refresh_token`。 |
| timestamp | 加密表单 Body | 是 | 毫秒时间戳。 |
| source | 加密表单 Body | 是 | `com.zhihu.web`。 |
| signature | 加密表单 Body | 是 | refresh payload 的 HMAC-SHA1。 |
| refresh_token | 加密表单 Body | 是 | 上一个接口返回的 refresh token。 |
| x-zse-83 | Header | 是 | `3_3.0`。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/x-www-form-urlencoded;charset=UTF-8' \
  -H 'x-zse-83: 3_3.0' \
  -H 'x-requested-with: fetch' \
  --data-binary 'ENCRYPTED_ZSE_V4_PAYLOAD' \
  'https://www.zhihu.com/api/v3/oauth/sign_in'
```

脱敏返回示例：

```json
{
  "access_token": "access_token_redacted",
  "refresh_token": "new_refresh_token_redacted",
  "token_type": "bearer"
}
```
