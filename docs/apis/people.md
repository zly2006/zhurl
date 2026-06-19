# 用户主页与关系 API

这些接口用于用户资料页、主页 Tab 列表、关注关系和拉黑关系。注意：
`api.zhihu.com/people/{identifier}` 与
`www.zhihu.com/api/v4/members/{url_token}/...` 不是等价接口，字段和鉴权行为可能不同。

## 获取用户资料

获取用户主页基础资料、关注数、回答数、认证徽章等信息。知乎++优先用
`url_token`，没有时退回用户 hash id。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 通常需要 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://api.zhihu.com/people/{identifier}
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| identifier | Path | 是 | 用户 `url_token` 或 hash id。 |
| include | Query | 否 | 知乎++使用资料、关注关系、徽章等字段集合。 |

zhurl 示例：

```bash
zhurl --web --jq '{id, name, url_token, follower_count, badge_v2}' \
  'https://api.zhihu.com/people/{url_token}'
```

脱敏返回示例：

```json
{
  "id": "person_hash_id",
  "url_token": "user-token",
  "name": "用户昵称",
  "headline": "一句话介绍",
  "follower_count": 1000,
  "following_count": 100,
  "answer_count": 42,
  "articles_count": 7,
  "is_following": false,
  "is_blocking": false,
  "badge_v2": {
    "title": "认证信息",
    "detail_badges": []
  }
}
```

## 用户回答列表

获取用户主页“回答”Tab。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/members/{url_token}/answers
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 用户 URL token 或可被 members 接口接受的标识。 |
| sort_by | Query | 否 | `voteups` 或 `created`。知乎++默认 `voteups`。 |
| include | Query | 否 | 知乎++请求正文、评论数、赞同数、作者徽章等字段。 |
| paging.next | Query | 否 | 翻页时使用返回的 `paging.next`。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, question, voteup_count}' \
  'https://www.zhihu.com/api/v4/members/{url_token}/answers?sort_by=voteups'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": 1234567890,
      "type": "answer",
      "excerpt": "回答摘要...",
      "voteup_count": 100,
      "question": {
        "id": 987654321,
        "title": "问题标题"
      }
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/members/user-token/answers?offset=20"
  }
}
```

## 用户文章列表

获取用户主页“文章”Tab。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/members/{url_token}/articles
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 用户 URL token。 |
| sort_by | Query | 否 | `created` 或 `voteups`。知乎++默认 `created`。 |
| include | Query | 否 | 知乎++请求正文、缩略图、赞同数、作者徽章等字段。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, title, voteup_count}' \
  'https://www.zhihu.com/api/v4/members/{url_token}/articles?sort_by=created'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": 123456,
      "type": "article",
      "title": "文章标题",
      "excerpt": "文章摘要...",
      "voteup_count": 100
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/members/user-token/articles?offset=20"
  }
}
```

## 用户动态列表

获取用户主页“动态”Tab。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v3/moments/{url_token}/activities
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 用户 URL token。 |
| include | Query | 否 | 可请求 `data[*].content,excerpt,headline,target.author.badge_v2`。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {type, target}' \
  'https://www.zhihu.com/api/v3/moments/{url_token}/activities'
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
        "excerpt": "动态内容摘要..."
      }
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v3/moments/user-token/activities?offset=next_offset"
  }
}
```

## 用户粉丝列表

获取用户粉丝。知乎++当前使用 `api.zhihu.com/people/{person_id}/followers` 作为粉丝列表来源。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://api.zhihu.com/people/{person_id}/followers
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| person_id | Path | 是 | 用户 hash id。 |
| include | Query | 否 | 知乎++请求回答数、文章数、关注状态、徽章等字段。 |
| paging.next | Query | 否 | 翻页 URL。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, name, follower_count}' \
  'https://api.zhihu.com/people/{person_id}/followers'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "follower_hash_id",
      "name": "粉丝昵称",
      "url_token": "follower-token",
      "follower_count": 100,
      "is_following": false
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://api.zhihu.com/people/person_hash_id/followers?offset=20"
  }
}
```

## 用户关注列表

获取用户关注的人。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/members/{url_token}/followees
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 用户 URL token。 |
| include | Query | 否 | 知乎++请求回答数、文章数、关注状态、徽章等字段。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, name, is_following}' \
  'https://www.zhihu.com/api/v4/members/{url_token}/followees'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "followee_hash_id",
      "name": "关注用户昵称",
      "url_token": "followee-token",
      "is_following": true
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/members/user-token/followees?offset=20"
  }
}
```

## 用户收藏夹列表

获取用户主页“收藏”Tab 的收藏夹列表。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选；私密收藏夹需要登录且有权限 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/members/{url_token}/favlists
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 用户 URL token。 |
| include | Query | 否 | 知乎++请求更新时间、回答数、关注数、创建者等字段。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, title, follower_count}' \
  'https://www.zhihu.com/api/v4/members/{url_token}/favlists'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "collection_id",
      "title": "收藏夹标题",
      "description": "收藏夹描述",
      "follower_count": 10,
      "answer_count": 20,
      "is_public": true
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/members/user-token/favlists?offset=20"
  }
}
```

## 用户收藏夹列表（people 路径）

获取用户收藏夹列表。知乎++在收藏夹分页页和“在浏览器中打开”辅助流程里使用
这个路径，后者会用 `limit=50` 查找当前账号下的专用私密收藏夹。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选；查询私密收藏夹或当前账号辅助收藏夹时需要登录 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/people/{url_token}/collections
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 用户 URL token。 |
| limit | Query | 否 | 每页数量；知乎++辅助流程使用 `50`。 |
| offset | Query | 否 | 分页偏移。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, title, is_public}' \
  'https://www.zhihu.com/api/v4/people/{url_token}/collections?limit=50'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "collection_id",
      "title": "收藏夹标题",
      "description": "收藏夹描述",
      "is_public": false,
      "item_count": 3,
      "follower_count": 0,
      "creator": {
        "url_token": "user-token",
        "name": "用户昵称"
      }
    }
  ],
  "paging": {
    "is_end": true,
    "next": ""
  }
}
```

## 用户提问列表

获取用户主页“提问”Tab。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/members/{url_token}/questions
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 用户 URL token。 |
| include | Query | 否 | 知乎++请求创建时间、回答数、关注数、作者、访问数、评论数、详情、关系、话题、赞同数。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, title, answer_count}' \
  'https://www.zhihu.com/api/v4/members/{url_token}/questions'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": 123456789,
      "type": "question",
      "title": "问题标题",
      "answer_count": 42,
      "follower_count": 100
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/members/user-token/questions?offset=20"
  }
}
```

## 用户想法列表

获取用户主页“想法”Tab。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/v2/pins/{url_token}/moments
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 用户 URL token。 |
| include | Query | 否 | 知乎++请求点赞数、评论数、创建/更新时间和内容。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, content, like_count}' \
  'https://www.zhihu.com/api/v4/v2/pins/{url_token}/moments'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": 123456789,
      "type": "pin",
      "content": [
        {
          "type": "text",
          "content": "想法内容..."
        }
      ],
      "like_count": 10,
      "comment_count": 2
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/v2/pins/user-token/moments?offset=20"
  }
}
```

## 用户专栏投稿列表

获取用户主页“专栏”Tab。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/members/{url_token}/column-contributions
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 用户 URL token。 |
| include | Query | 否 | 知乎++请求文章数、关注者和作者字段。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, title, articles_count}' \
  'https://www.zhihu.com/api/v4/members/{url_token}/column-contributions'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "column_id",
      "title": "专栏标题",
      "articles_count": 10,
      "followers": 100
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/members/user-token/column-contributions?offset=20"
  }
}
```

## 用户关注的收藏夹

获取用户关注订阅页里的收藏夹列表。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/members/{url_token}/following-favlists
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 用户 URL token。 |
| include | Query | 否 | 知乎++请求更新时间、回答数、关注数、创建者。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, title, creator}' \
  'https://www.zhihu.com/api/v4/members/{url_token}/following-favlists'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "collection_id",
      "title": "收藏夹标题",
      "creator": {
        "id": "person_hash_id",
        "name": "创建者昵称"
      }
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/members/user-token/following-favlists?offset=20"
  }
}
```

## 用户关注的问题

获取用户关注订阅页里的问题列表。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/members/{url_token}/following-questions
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 用户 URL token。 |
| paging.next | Query | 否 | 翻页 URL。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, title, type}' \
  'https://www.zhihu.com/api/v4/members/{url_token}/following-questions'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "question_id",
      "type": "question",
      "title": "关注的问题标题"
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/members/user-token/following-questions?offset=20"
  }
}
```

## 用户关注的话题

获取用户关注订阅页里的话题列表。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/members/{url_token}/following-topic-contributions
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 用户 URL token。 |
| paging.next | Query | 否 | 翻页 URL。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, name, topic}' \
  'https://www.zhihu.com/api/v4/members/{url_token}/following-topic-contributions'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "topic_id",
      "type": "topic",
      "name": "话题名",
      "avatar_url": "https://picx.zhimg.com/topic.jpg"
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/members/user-token/following-topic-contributions?offset=20"
  }
}
```

## 用户关注的专栏

获取用户关注订阅页里的专栏列表。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/members/{url_token}/following-columns
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 用户 URL token。 |
| include | Query | 否 | 知乎++请求文章数、关注者和作者字段。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, title, followers}' \
  'https://www.zhihu.com/api/v4/members/{url_token}/following-columns'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "column_id",
      "title": "专栏标题",
      "followers": 100,
      "articles_count": 10
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/members/user-token/following-columns?offset=20"
  }
}
```

## 关注或取消关注用户

关注或取消关注某个用户。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` / `DELETE` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/members/{url_token}/followers
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 要关注或取消关注的用户 URL token。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  'https://www.zhihu.com/api/v4/members/{url_token}/followers'

zhurl --web -X DELETE \
  'https://www.zhihu.com/api/v4/members/{url_token}/followers'
```

脱敏返回示例：

```json
{
  "follower_count": 1001
}
```

## 拉黑或取消拉黑用户

把用户加入或移出知乎拉黑列表。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` / `DELETE` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/members/{url_token}/actions/block
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| url_token | Path | 是 | 要拉黑或取消拉黑的用户 URL token。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  'https://www.zhihu.com/api/v4/members/{url_token}/actions/block'

zhurl --web -X DELETE \
  'https://www.zhihu.com/api/v4/members/{url_token}/actions/block'
```

脱敏返回示例：

```json
{
  "success": true
}
```
