# 知乎 API 参考

这是一份面向 `zhurl` 的非官方知乎 API 索引，来源是当前本机
`/Users/zhaoliyan/IdeaProjects/Zhihu` 的知乎++源码调用点。它用于调试、
复现和探索，不是稳定 SDK 契约。

本索引只整理知乎与知乎日报的一方接口。知乎++自己的更新检查、匿名统计等
外部服务接口不属于知乎 API，未纳入正文。

整理时间：2026-06-19

## 字段说明

| 字段 | 含义 |
| --- | --- |
| 模式 | `web` 表示使用 `zhurl --web`；`android` 表示使用 `zhurl --android`；`public` 表示接口本身公开，但当前 `zhurl` 仍会读取本地账号文件。 |
| 是否需要登录 | 该接口是否通常需要有效知乎登录态。 |
| 是否需要 d_c0 签名 | 知乎++是否按 web 请求方式使用 `d_c0` 生成 `x-zse-96`。对 `zhurl` 来说通常对应 `--web`。 |

所有返回示例都已脱敏。示例中的 `{answer_id}`、`{url_token}`、`{question_id}`
等占位符需要替换后再运行。

## 目录

- [账号与登录](apis/account.md)
- [信息流、搜索与历史](apis/feeds-search-history.md)
- [内容详情与媒体](apis/content.md)
- [用户主页与关系](apis/people.md)
- [互动、评论、收藏与通知](apis/interactions.md)
- [知乎日报](apis/daily.md)

## 常用 zhurl 写法

Web 签名 GET：

```bash
zhurl --web --jq '{id, type}' 'https://www.zhihu.com/api/v4/me'
```

Android API GET：

```bash
zhurl --android --jq '.data[0]' 'https://api.zhihu.com/topstory/recommend'
```

Web 签名 JSON POST：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/json' \
  -d '{"key":"value"}' \
  'https://www.zhihu.com/api/v4/example'
```

Android/API 域名表单 PUT：

```bash
zhurl --android -X PUT \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -d 'field=value' \
  'https://api.zhihu.com/example'
```
