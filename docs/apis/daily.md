# 知乎日报 API

知乎日报接口与普通知乎 web/API 接口不同：知乎++直接请求公开日报域名，不使用账号 cookie，也不使用
`d_c0` 签名。当前 `zhurl` 仍会读取本地账号文件；如果只想无账号请求，可以直接用 curl。

## 最新日报列表

获取知乎日报最新日期的故事列表。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `public` |
| 是否需要登录 | 否 |
| 是否需要 d_c0 签名 | 否 |

URL：

```text
https://news-at.zhihu.com/api/4/stories/latest
```

备用 URL：

```text
https://daily.zhihu.com/api/4/stories/latest
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| 无 | - | - | 知乎++无业务参数。 |

zhurl 示例：

```bash
zhurl --web --jq '{date, stories: [.stories[0] | {id, title}]}' \
  'https://news-at.zhihu.com/api/4/stories/latest'
```

脱敏返回示例：

```json
{
  "date": "20260619",
  "stories": [
    {
      "id": 12345678,
      "title": "日报标题",
      "hint": "作者或来源",
      "url": "https://daily.zhihu.com/story/12345678",
      "images": [
        "https://picx.zhimg.com/daily.jpg"
      ]
    }
  ],
  "top_stories": []
}
```

## 某日前日报列表

获取指定日期之前一天的日报故事列表。知乎日报接口里的 `{date}` 通常表示“获取该日期之前”的内容。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `public` |
| 是否需要登录 | 否 |
| 是否需要 d_c0 签名 | 否 |

URL：

```text
https://news-at.zhihu.com/api/4/stories/before/{date}
```

备用 URL：

```text
https://daily.zhihu.com/api/4/stories/before/{date}
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| date | Path | 是 | `yyyyMMdd`，例如 `20260619`。 |

zhurl 示例：

```bash
zhurl --web --jq '{date, first: .stories[0]}' \
  'https://news-at.zhihu.com/api/4/stories/before/20260619'
```

脱敏返回示例：

```json
{
  "date": "20260618",
  "stories": [
    {
      "id": 12345677,
      "title": "前一日标题",
      "hint": "作者或来源",
      "url": "https://daily.zhihu.com/story/12345677"
    }
  ]
}
```

## 日报故事详情

获取日报故事正文。知乎++用它解析正文里的“查看知乎讨论”链接，再跳转到普通知乎内容。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `public` |
| 是否需要登录 | 否 |
| 是否需要 d_c0 签名 | 否 |

URL：

```text
https://daily.zhihu.com/api/7/story/{story_id}
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| story_id | Path | 是 | 日报故事 ID。 |

zhurl 示例：

```bash
zhurl --web --jq '{id, title, body}' \
  'https://daily.zhihu.com/api/7/story/{story_id}'
```

脱敏返回示例：

```json
{
  "id": 12345678,
  "title": "日报标题",
  "body": "<div class=\"content\">日报正文 HTML...<div class=\"view-more\"><a href=\"https://www.zhihu.com/question/123456789\">查看知乎讨论</a></div></div>",
  "image": "https://picx.zhimg.com/daily-cover.jpg",
  "css": [
    "https://static.daily.zhihu.com/css/share.css"
  ]
}
```
