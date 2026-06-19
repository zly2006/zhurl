# 内容详情与媒体 API

这些接口用于获取回答、文章、问题、想法详情，以及视频、表情包和 AI 总结等内容相关资源。

## 获取回答详情

获取单个回答正文、计数、作者、问题主题和交互状态。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选；登录后字段更完整 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/answers/{answer_id}
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| answer_id | Path | 是 | 回答 ID。 |
| include | Query | 否 | 知乎++常用 `content,paid_info,can_comment,excerpt,thanks_count,voteup_count,comment_count,visited_count,attachment,reaction,ip_info,pagination_info,question.topics,reaction.relation.voting,author.badge_v2`。 |

zhurl 示例：

```bash
zhurl --web --jq '{id, type, content, voteup_count, author: .author.name}' \
  'https://www.zhihu.com/api/v4/answers/{answer_id}?include=content,paid_info,can_comment,excerpt,thanks_count,voteup_count,comment_count,visited_count,attachment,reaction,ip_info,pagination_info,question.topics,reaction.relation.voting,author.badge_v2'
```

脱敏返回示例：

```json
{
  "id": 1234567890,
  "type": "answer",
  "content": "<p>回答正文 HTML...</p>",
  "excerpt": "回答摘要...",
  "voteup_count": 100,
  "comment_count": 12,
  "author": {
    "id": "person_hash_id",
    "name": "作者昵称",
    "url_token": "author-token",
    "badge_v2": {
      "title": "认证信息"
    }
  },
  "question": {
    "id": 987654321,
    "title": "问题标题"
  }
}
```

## 获取文章详情

获取单篇文章正文、主题、作者和互动计数。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选；登录后字段更完整 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/articles/{article_id}
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| article_id | Path | 是 | 文章 ID。 |
| include | Query | 否 | 知乎++常用 `content,topics,paid_info,can_comment,excerpt,thanks_count,voteup_count,comment_count,visited_count,relationship,ip_info,relationship.vote,author.badge_v2`。 |

zhurl 示例：

```bash
zhurl --web --jq '{id, title, content, author: .author.name}' \
  'https://www.zhihu.com/api/v4/articles/{article_id}?include=content,topics,paid_info,can_comment,excerpt,thanks_count,voteup_count,comment_count,visited_count,relationship,ip_info,relationship.vote,author.badge_v2'
```

脱敏返回示例：

```json
{
  "id": 123456,
  "type": "article",
  "title": "文章标题",
  "content": "<p>文章正文 HTML...</p>",
  "excerpt": "文章摘要...",
  "voteup_count": 100,
  "comment_count": 12,
  "author": {
    "id": "person_hash_id",
    "name": "作者昵称"
  },
  "topics": [
    {
      "id": "topic_id",
      "name": "话题名"
    }
  ]
}
```

## 获取问题详情

获取问题标题、描述、回答数、关注数、主题和关注状态。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选；关注状态需要登录 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/questions/{question_id}
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| question_id | Path | 是 | 问题 ID。 |
| include | Query | 否 | 知乎++常用 `read_count,visit_count,answer_count,voteup_count,comment_count,follower_count,detail,excerpt,author,relationship.is_following,topics`。 |

zhurl 示例：

```bash
zhurl --web --jq '{id, title, detail, answer_count, follower_count}' \
  'https://www.zhihu.com/api/v4/questions/{question_id}?include=read_count,visit_count,answer_count,voteup_count,comment_count,follower_count,detail,excerpt,author,relationship.is_following,topics'
```

脱敏返回示例：

```json
{
  "id": 123456789,
  "type": "question",
  "title": "问题标题",
  "detail": "<p>问题描述 HTML...</p>",
  "answer_count": 42,
  "follower_count": 1000,
  "relationship": {
    "is_following": false
  },
  "topics": [
    {
      "id": "topic_id",
      "name": "话题名"
    }
  ]
}
```

## 获取问题回答列表

获取某个问题下的回答 feed，用于问题页和回答切换导航。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/questions/{question_id}/feeds
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| question_id | Path | 是 | 问题 ID。 |
| limit | Query | 否 | 知乎++问题页使用 `20`，回答切换预取使用 `6`。 |
| order | Query | 否 | `default` 或 `updated`。 |
| offset | Query | 否 | 翻页偏移。 |
| include | Query | 否 | 可请求 `data[*].content,excerpt,headline,target.author.badge_v2`。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {type, target}' \
  'https://www.zhihu.com/api/v4/questions/{question_id}/feeds?limit=20&order=default'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "type": "feed",
      "target": {
        "type": "answer",
        "id": 1234567890,
        "excerpt": "回答摘要...",
        "author": {
          "id": "person_hash_id",
          "name": "作者昵称"
        }
      }
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/questions/123/feeds?offset=20"
  }
}
```

## 获取想法详情

获取单条想法内容、作者、主题和互动状态。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选；登录后字段更完整 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/pins/{pin_id}?include=topics
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| pin_id | Path | 是 | 想法 ID。 |
| include | Query | 否 | 知乎++使用 `topics`。 |

zhurl 示例：

```bash
zhurl --web --jq '{id, content, author: .author.name, like_count}' \
  'https://www.zhihu.com/api/v4/pins/{pin_id}?include=topics'
```

脱敏返回示例：

```json
{
  "id": 123456789,
  "type": "pin",
  "content": [
    {
      "type": "text",
      "content": "想法正文..."
    }
  ],
  "like_count": 10,
  "comment_count": 2,
  "author": {
    "id": "person_hash_id",
    "name": "作者昵称"
  },
  "topics": [
    {
      "id": "topic_id",
      "name": "话题名"
    }
  ]
}
```

## 获取视频播放信息

根据视频 ID 和内容 ID 获取可播放的清晰度列表。知乎++会从 `video_play.playlist.mp4`
里选择码率最高的 URL。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 否；知乎++没有在此处使用 `signZhihuFetchRequest` |

URL：

```text
https://www.zhihu.com/api/v4/video/play_info?r={video_id}
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| r | Query | 是 | 视频 ID。 |
| content_id | JSON Body | 是 | 所属内容 ID。 |
| content_type_str | JSON Body | 是 | 内容类型，如 `answer`。 |
| video_id | JSON Body | 是 | 视频 ID。 |
| scene_code | JSON Body | 是 | 知乎++使用 `answer_detail_web`。 |
| is_only_video | JSON Body | 是 | 知乎++传 `true`。 |
| x-app-za | Header | 是 | 知乎++传 `OS=webplayer`。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/json' \
  -H 'x-app-za: OS=webplayer' \
  -d '{"content_id":"{answer_id}","content_type_str":"answer","video_id":"{video_id}","scene_code":"answer_detail_web","is_only_video":true}' \
  'https://www.zhihu.com/api/v4/video/play_info?r={video_id}'
```

脱敏返回示例：

```json
{
  "video_play": {
    "playlist": {
      "mp4": [
        {
          "bitrate": 1200,
          "url": [
            "https://video.zhihu.com/path/to/video.mp4"
          ]
        }
      ]
    }
  }
}
```

## 获取表情包贴纸组

下载知乎表情占位符到图片 URL 的映射。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 否 |
| 是否需要 d_c0 签名 | 否；知乎++Android 实现使用普通 `HttpURLConnection` |

URL：

```text
https://www.zhihu.com/api/v4/sticker-groups/1114161698310770688
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| 无 | - | - | 固定贴纸组 ID。 |

zhurl 示例：

```bash
zhurl --web --jq '.data.stickers[0] | {id, placeholder, static_image_url}' \
  'https://www.zhihu.com/api/v4/sticker-groups/1114161698310770688'
```

脱敏返回示例：

```json
{
  "data": {
    "stickers": [
      {
        "id": "sticker_id",
        "placeholder": "[表情]",
        "static_image_url": "https://picx.zhimg.com/sticker.png"
      }
    ]
  }
}
```

## 请求知乎直答总结

对回答或文章发起知乎 AI 总结流式请求。返回是 SSE，不是普通 JSON。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/ai_ingress/stream/completion
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| content_id | JSON Body | 是 | 回答或文章 ID。 |
| content_type | JSON Body | 是 | `answer` 或 `article`。 |
| title | JSON Body | 是 | 内容标题。 |
| Accept | Header | 是 | `text/event-stream`。 |
| x-xsrftoken | Header | 通常需要 | 当前 `_xsrf` cookie。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Accept: text/event-stream' \
  -H 'Content-Type: application/json' \
  -d '{"content_id":"{answer_id}","content_type":"answer","title":"内容标题"}' \
  'https://www.zhihu.com/ai_ingress/stream/completion'
```

脱敏返回示例：

```text
event: answer
data: {"event":"answer","data":{"summary":"总结片段...","delta":true}}

event: end
data: {"event":"end","data":{}}
```
