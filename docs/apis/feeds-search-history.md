# 信息流、搜索与历史 API

这些接口覆盖首页推荐、关注流、热榜、搜索、浏览历史和最近阅读上报。

## Android 首页推荐

获取 Android 端推荐流，知乎++的 Android 推荐页使用这个接口。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `android` |
| 是否需要登录 | 可选；登录后更个性化 |
| 是否需要 d_c0 签名 | 否 |

URL：

```text
https://api.zhihu.com/topstory/recommend
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| include | Query | 否 | 可请求 `data[*].content,excerpt,headline` 等字段。 |
| paging.next | Query | 否 | 翻页时使用返回里的 `paging.next`。 |

zhurl 示例：

```bash
zhurl --android --jq '.data[0] | {type, action, paging: .paging}' \
  'https://api.zhihu.com/topstory/recommend'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "type": "ComponentCard",
      "action": {
        "parameter": "route_url=https%3A%2F%2Fwww.zhihu.com%2Fquestion%2Fquestion_id%2Fanswer%2Fanswer_id"
      },
      "children": []
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://api.zhihu.com/topstory/recommend?offset=next_offset"
  }
}
```

## Web 首页推荐

获取 web 端首页推荐流。知乎++本地推荐初始化会抓取这个接口。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v3/feed/topstory/recommend
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| desktop | Query | 否 | 知乎++使用 `true`。 |
| limit | Query | 否 | 每页数量，例如 `20`。 |
| offset | Query | 否 | 翻页偏移。 |
| include | Query | 否 | 常用 `data[*].content,excerpt,headline`。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {type, target}' \
  'https://www.zhihu.com/api/v3/feed/topstory/recommend?desktop=true&limit=20'
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
        "question": {
          "id": 987654321,
          "title": "问题标题"
        }
      }
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v3/feed/topstory/recommend?offset=next_offset"
  }
}
```

## 关注赞同推荐

获取带 `action_feed=True` 的推荐流，知乎++本地推荐会把它作为候选来源。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v3/feed/topstory/recommend?action_feed=True&limit={limit}
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| action_feed | Query | 是 | 固定为 `True`。 |
| limit | Query | 否 | 每页数量。 |
| offset | Query | 否 | 翻页偏移。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {type, target}' \
  'https://www.zhihu.com/api/v3/feed/topstory/recommend?action_feed=True&limit=20'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "type": "feed",
      "target": {
        "type": "article",
        "id": 123456,
        "title": "文章标题"
      }
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v3/feed/topstory/recommend?action_feed=True&offset=next_offset"
  }
}
```

## 关注动态

获取当前账号关注用户的动态流。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v3/moments
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| limit | Query | 否 | 知乎++使用 `10`。 |
| desktop | Query | 否 | 知乎++使用 `true`。 |
| offset | Query | 否 | 翻页偏移，来自 `paging.next`。 |
| include | Query | 否 | 可请求内容摘要和作者徽章字段。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {type, target, actors}' \
  'https://www.zhihu.com/api/v3/moments?limit=10&desktop=true'
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
        "excerpt": "动态中的内容摘要..."
      },
      "actors": [
        {
          "id": "person_hash_id",
          "name": "用户昵称"
        }
      ]
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v3/moments?offset=next_offset"
  }
}
```

## 关注推荐动态

获取关注页的推荐动态流。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://api.zhihu.com/moments_v3?feed_type=recommend
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| feed_type | Query | 是 | 固定为 `recommend`。 |
| offset | Query | 否 | 翻页偏移。 |
| include | Query | 否 | 可请求内容摘要和作者字段。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {type, target}' \
  'https://api.zhihu.com/moments_v3?feed_type=recommend'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "type": "feed",
      "target": {
        "type": "pin",
        "id": 123456789,
        "excerpt": "想法摘要..."
      }
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://api.zhihu.com/moments_v3?feed_type=recommend&offset=next_offset"
  }
}
```

## 最近有动态的关注用户

获取最近更新的关注用户列表。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://api.zhihu.com/moments/recent?type=raw
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| type | Query | 是 | 知乎++使用 `raw`。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[] | {actor, unread_count}' \
  'https://api.zhihu.com/moments/recent?type=raw'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "actor": {
        "id": "person_hash_id",
        "url_token": "user-token",
        "name": "用户昵称",
        "avatar_url": "https://picx.zhimg.com/avatar.jpg"
      },
      "unread_count": 2
    }
  ]
}
```

## 热榜

获取知乎热榜总榜。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 否 |
| 是否需要 d_c0 签名 | 是；知乎++仍走统一 web 签名请求 |

URL：

```text
https://www.zhihu.com/api/v3/feed/topstory/hot-lists/total?limit=50&mobile=true
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| limit | Query | 否 | 知乎++使用 `50`。 |
| mobile | Query | 否 | 知乎++使用 `true`。 |
| include | Query | 否 | 可请求内容摘要字段。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {type, target}' \
  'https://www.zhihu.com/api/v3/feed/topstory/hot-lists/total?limit=50&mobile=true'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "type": "feed",
      "target": {
        "type": "question",
        "id": 123456789,
        "title": "热榜问题标题",
        "excerpt": "热榜摘要..."
      }
    }
  ],
  "paging": {
    "is_end": true,
    "next": ""
  }
}
```

## 热搜词

获取搜索页热搜词。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 否 |
| 是否需要 d_c0 签名 | 是；知乎++仍走统一 web 签名请求 |

URL：

```text
https://www.zhihu.com/api/v4/search/hot_search
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| 无 | - | - | 知乎++不传业务参数。 |

zhurl 示例：

```bash
zhurl --web --jq '.top_search.words[0]' \
  'https://www.zhihu.com/api/v4/search/hot_search'
```

脱敏返回示例：

```json
{
  "top_search": {
    "words": [
      {
        "query": "热搜关键词",
        "display_query": "热搜关键词",
        "heat_score": 123456
      }
    ]
  }
}
```

## 搜索

按关键词搜索回答、文章、视频等内容。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/search_v3
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| q | Query | 是 | 搜索关键词。 |
| t | Query | 是 | 知乎++使用 `general`。 |
| gk_version | Query | 否 | 知乎++使用 `gz-gaokao`。 |
| correction | Query | 否 | 知乎++使用 `1`。 |
| offset | Query | 否 | 起始偏移，默认 `0`。 |
| limit | Query | 否 | 每页数量，知乎++使用 `20`。 |
| search_source | Query | 否 | `Normal` 或 `Filter`。 |
| vertical | Query | 否 | 内容类型：`answer`、`article`、`zvideo`。 |
| sort | Query | 否 | 排序：`created_time`、`upvoted_count`。 |
| time_interval | Query | 否 | 时间范围：`a_day`、`a_week`、`a_month`、`three_months`、`half_a_year`、`a_year`。 |
| restricted_scene / restricted_field / restricted_value | Query | 否 | 用户主页内搜索时限制到指定 `member_hash_id`。 |
| include | Query | 否 | 知乎++使用 `data[*].highlight,object,type`。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {type, object}' \
  'https://www.zhihu.com/api/v4/search_v3?gk_version=gz-gaokao&t=general&q=%E6%90%9C%E7%B4%A2%E8%AF%8D&correction=1&offset=0&limit=20&search_source=Normal&show_all_topics=0'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "type": "search_result",
      "highlight": {
        "title": ["命中的标题片段"]
      },
      "object": {
        "type": "answer",
        "id": 1234567890,
        "url": "https://api.zhihu.com/answers/1234567890"
      }
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/search_v3?offset=20&q=..."
  }
}
```

## 在线浏览历史

获取账号的云端浏览历史。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://api.zhihu.com/unify-consumption/read_history
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| offset | Query | 否 | 起始偏移，知乎++使用 `0`。 |
| limit | Query | 否 | 每页数量，知乎++使用 `10`。 |
| include | Query | 否 | 可请求内容摘要字段。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {data}' \
  'https://api.zhihu.com/unify-consumption/read_history?offset=0&limit=10'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "data": {
        "header": {
          "title": "浏览过的内容标题"
        },
        "action": {
          "url": "https://www.zhihu.com/question/123/answer/456"
        },
        "content": {
          "summary": "内容摘要..."
        }
      }
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://api.zhihu.com/unify-consumption/read_history?offset=10&limit=10"
  }
}
```

## 添加浏览历史

把内容写入知乎云端浏览历史。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/read_history/add
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| content_token | JSON Body | 是 | 内容 ID 或用户 ID。 |
| content_type | JSON Body | 是 | 内容类型，如 `answer`、`article`、`question`、`pin`、`profile`。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/json' \
  -d '{"content_token":"{answer_id}","content_type":"answer"}' \
  'https://www.zhihu.com/api/v4/read_history/add'
```

脱敏返回示例：

```json
{
  "success": true
}
```

## 清空浏览历史

清空知乎云端浏览历史。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://api.zhihu.com/read_history/batch_del
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| pairs | JSON Body | 是 | 要删除的历史项；知乎++清空时传空数组。 |
| clear | JSON Body | 是 | 清空时传 `true`。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/json' \
  -d '{"pairs":[],"clear":true}' \
  'https://api.zhihu.com/read_history/batch_del'
```

脱敏返回示例：

```json
{
  "success": true
}
```

## 上报最近阅读触达

上报 feed 中内容的 `touch` 或 `read` 状态。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/lastread/touch
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| items | multipart/form-data | 是 | JSON 字符串数组，如 `[["answer","{answer_id}","read"]]`。 |
| x-requested-with | Header | 是 | 知乎++发送 `fetch`。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'x-requested-with: fetch' \
  -H 'Content-Type: multipart/form-data; boundary=----zhurl' \
  --data-binary $'------zhurl\r\nContent-Disposition: form-data; name="items"\r\n\r\n[["answer","{answer_id}","read"]]\r\n------zhurl--\r\n' \
  'https://www.zhihu.com/lastread/touch'
```

脱敏返回示例：

```json
{
  "success": true
}
```
