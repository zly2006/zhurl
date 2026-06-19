# 互动、评论、收藏与通知 API

这些接口会改变账号状态或读取与账号强相关的数据。除特别说明外，都应使用已登录账号，并谨慎操作真实内容。

## 回答投票

对回答点赞、取消点赞或切换投票状态。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/answers/{answer_id}/voters
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| answer_id | Path | 是 | 回答 ID。 |
| type | JSON Body | 是 | 知乎++使用投票状态 key，例如赞同或取消状态。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/json' \
  -d '{"type":"up"}' \
  'https://www.zhihu.com/api/v4/answers/{answer_id}/voters'
```

脱敏返回示例：

```json
{
  "voteup_count": 101,
  "voting": 1
}
```

## 文章投票

对文章点赞或取消点赞。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/articles/{article_id}/voters
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| article_id | Path | 是 | 文章 ID。 |
| voting | JSON Body | 是 | `1` 表示赞同，`0` 表示取消。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/json' \
  -d '{"voting":1}' \
  'https://www.zhihu.com/api/v4/articles/{article_id}/voters'
```

脱敏返回示例：

```json
{
  "voteup_count": 101,
  "voting": 1
}
```

## 回答赞同关系提示

获取回答的社交赞同提示文案，例如关注的人是否赞同。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/answers/{answer_id}/relationship?desktop=true
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| answer_id | Path | 是 | 回答 ID。 |
| desktop | Query | 是 | 知乎++使用 `true`。 |

zhurl 示例：

```bash
zhurl --web --jq '{text, relationship}' \
  'https://www.zhihu.com/api/v4/answers/{answer_id}/relationship?desktop=true'
```

脱敏返回示例：

```json
{
  "text": "你关注的人中有人赞同了该回答",
  "relationship": {
    "voting": 1
  }
}
```

## 回答赞同者列表

获取回答赞同者分页列表。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选；部分数据可能需要登录 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/answers/{answer_id}/upvoters
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| answer_id | Path | 是 | 回答 ID。 |
| limit | Query | 否 | 知乎++使用 `10`。 |
| offset | Query | 否 | 起始偏移。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, name, url_token}' \
  'https://www.zhihu.com/api/v4/answers/{answer_id}/upvoters?limit=10&offset=0'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "person_hash_id",
      "name": "赞同者昵称",
      "url_token": "voter-token"
    }
  ],
  "paging": {
    "totals": 100,
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/answers/123/upvoters?offset=10&limit=10"
  }
}
```

## 想法点赞或取消点赞

点赞或取消点赞单条想法。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` / `DELETE` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/pins/{pin_id}/voters/up
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| pin_id | Path | 是 | 想法 ID。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  'https://www.zhihu.com/api/v4/pins/{pin_id}/voters/up'

zhurl --web -X DELETE \
  'https://www.zhihu.com/api/v4/pins/{pin_id}/voters/up'
```

脱敏返回示例：

```json
{
  "liked_count": 11,
  "is_liked": true
}
```

## 想法赞同者列表

获取想法点赞者分页列表。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/pins/{pin_id}/upvoters
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| pin_id | Path | 是 | 想法 ID。 |
| limit | Query | 否 | 知乎++使用 `10`。 |
| offset | Query | 否 | 起始偏移。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, name}' \
  'https://www.zhihu.com/api/v4/pins/{pin_id}/upvoters?limit=10&offset=0'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "person_hash_id",
      "name": "点赞者昵称"
    }
  ],
  "paging": {
    "totals": 10,
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/pins/123/upvoters?offset=10&limit=10"
  }
}
```

## 想法投票

提交想法内投票卡片的选项。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/polls/{poll_id}
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| poll_id | Path | 是 | 投票 ID。 |
| options | JSON Body | 是 | 选项 ID 数组。知乎++一次提交一个选项。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/json' \
  -d '{"options":["{option_id}"]}' \
  'https://www.zhihu.com/api/v4/polls/{poll_id}'
```

脱敏返回示例：

```json
{
  "id": "poll_id",
  "options": [
    {
      "id": "option_id",
      "vote_count": 10,
      "voted": true
    }
  ]
}
```

## 关注或取消关注问题

关注或取消关注某个问题。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` / `DELETE` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/questions/{question_id}/followers
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| question_id | Path | 是 | 问题 ID。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  'https://www.zhihu.com/api/v4/questions/{question_id}/followers'

zhurl --web -X DELETE \
  'https://www.zhihu.com/api/v4/questions/{question_id}/followers'
```

脱敏返回示例：

```json
{
  "follower_count": 101,
  "is_following": true
}
```

## 根评论列表

获取回答、文章、想法、问题或段评的根评论。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/comment_v5/answers/{answer_id}/root_comment
https://www.zhihu.com/api/v4/comment_v5/articles/{article_id}/root_comment
https://www.zhihu.com/api/v4/comment_v5/pins/{pin_id}/root_comment
https://www.zhihu.com/api/v4/comment_v5/questions/{question_id}/root_comment
https://www.zhihu.com/api/v4/comment_v5/{content_type}s/{content_id}/segment/root_comment?segment_id={segment_id}&limit=20&offset=
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| answer_id / article_id / pin_id / question_id | Path | 视 URL 而定 | 内容 ID。 |
| content_type | Path | 段评接口必填 | `answer` 或 `article` 等去掉复数后再拼回 `{content_type}s`。 |
| content_id | Path | 段评接口必填 | 内容 ID。 |
| segment_id | Query | 段评接口必填 | 段落评论 ID。 |
| order_by | Query | 否 | `score` 或 `ts`。 |
| order | Query | 否 | 导出评论时使用 `score`。 |
| limit | Query | 否 | 每页数量。 |
| offset | Query | 否 | 起始偏移。 |
| include | Query | 否 | 可请求 `data[*].content,excerpt,headline`。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, content, author: .author.name}' \
  'https://www.zhihu.com/api/v4/comment_v5/answers/{answer_id}/root_comment?order_by=score'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "comment_id",
      "content": "<p>评论内容...</p>",
      "created_time": 1790000000,
      "liked": false,
      "like_count": 3,
      "author": {
        "id": "person_hash_id",
        "name": "评论者昵称"
      },
      "child_comments": []
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/comment_v5/answers/123/root_comment?offset=next"
  }
}
```

## 子评论列表

获取某条评论下的子评论。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 可选 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/comment_v5/comment/{comment_id}/child_comment
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| comment_id | Path | 是 | 根评论 ID。 |
| offset | Query | 否 | 翻页偏移。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, content, author: .author.name}' \
  'https://www.zhihu.com/api/v4/comment_v5/comment/{comment_id}/child_comment'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "child_comment_id",
      "content": "<p>子评论内容...</p>",
      "author": {
        "id": "person_hash_id",
        "name": "评论者昵称"
      }
    }
  ],
  "paging": {
    "is_end": true,
    "next": ""
  }
}
```

## 发表评论或回复

向回答、文章、想法、问题或段评提交评论。回复评论时带 `reply_comment_id`。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/comment_v5/answers/{answer_id}/comment
https://www.zhihu.com/api/v4/comment_v5/articles/{article_id}/comment
https://www.zhihu.com/api/v4/comment_v5/pins/{pin_id}/comment
https://www.zhihu.com/api/v4/comment_v5/questions/{question_id}/comment
https://www.zhihu.com/api/v4/comment_v5/{content_type}s/{content_id}/segment/comment?segment_id={segment_id}
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| content | JSON Body | 是 | HTML 片段，知乎++发送 `<p>评论内容</p>`。 |
| reply_comment_id | JSON Body | 否 | 回复目标评论 ID。 |
| 其他 Path / Query 参数 | Path / Query | 视 URL 而定 | 同根评论接口。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/json' \
  -d '{"content":"<p>评论内容</p>","reply_comment_id":"{comment_id}"}' \
  'https://www.zhihu.com/api/v4/comment_v5/answers/{answer_id}/comment'
```

脱敏返回示例：

```json
{
  "id": "new_comment_id",
  "content": "<p>评论内容</p>",
  "author": {
    "id": "person_hash_id",
    "name": "当前用户"
  },
  "created_time": 1790000000
}
```

## 评论点赞或取消点赞

点赞或取消点赞某条评论。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` / `DELETE` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/comments/{comment_id}/like
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| comment_id | Path | 是 | 评论 ID。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  'https://www.zhihu.com/api/v4/comments/{comment_id}/like'

zhurl --web -X DELETE \
  'https://www.zhihu.com/api/v4/comments/{comment_id}/like'
```

脱敏返回示例：

```json
{
  "success": true
}
```

## 段落反应点赞

对正文中的段评高亮进行点赞或取消点赞。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` / `DELETE` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/reaction/{target_type}s/{content_id}/segment_reaction
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| target_type | Path | 是 | 内容类型单数，如 `answer`、`article`。URL 中拼成复数。 |
| content_id | Path | 是 | 内容 ID。 |
| seg_id | JSON Body | 点赞时可选 | 已有段落反应 ID，多个用逗号连接。 |
| seg_ids | JSON Body | 取消点赞时必填 | 段落反应 ID，多个用逗号连接。 |
| content | JSON Body | 点赞时必填 | 被高亮的文本。 |
| position | JSON Body | 点赞时必填 | 起止段落 ID 和 offset。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/json' \
  -d '{"content":"高亮文本","position":{"start":{"paragraph_id":"p1","offset":0},"end":{"paragraph_id":"p1","offset":4}}}' \
  'https://www.zhihu.com/api/v4/reaction/answers/{answer_id}/segment_reaction'
```

脱敏返回示例：

```json
{
  "payload": {
    "segId": "segment_reaction_id"
  }
}
```

## 获取收藏夹详情

获取单个收藏夹元信息。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 视收藏夹权限而定 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/collections/{collection_id}
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| collection_id | Path | 是 | 收藏夹 ID。 |

zhurl 示例：

```bash
zhurl --web --jq '.collection | {id, title, is_public}' \
  'https://www.zhihu.com/api/v4/collections/{collection_id}'
```

脱敏返回示例：

```json
{
  "collection": {
    "id": "collection_id",
    "title": "收藏夹标题",
    "description": "收藏夹描述",
    "is_public": false,
    "creator": {
      "id": "person_hash_id",
      "name": "创建者昵称"
    }
  }
}
```

## 获取收藏夹条目

获取收藏夹内的内容列表。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 视收藏夹权限而定 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/collections/{collection_id}/items
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| collection_id | Path | 是 | 收藏夹 ID。 |
| limit | Query | 否 | 每页数量。 |
| offset | Query | 否 | 起始偏移。 |
| include | Query | 否 | 可请求内容摘要字段。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {content}' \
  'https://www.zhihu.com/api/v4/collections/{collection_id}/items'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "content": {
        "type": "answer",
        "id": 1234567890,
        "title": "收藏内容标题",
        "excerpt": "收藏内容摘要..."
      }
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/collections/collection_id/items?offset=20"
  }
}
```

## 查询内容所在收藏夹

查询某个回答或文章与当前账号收藏夹的关系。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://api.zhihu.com/collections/contents/{content_type}/{content_id}
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| content_type | Path | 是 | `answer` 或 `article`。 |
| content_id | Path | 是 | 内容 ID。 |
| limit | Query | 否 | 知乎++使用 `50`。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, title, is_favorited}' \
  'https://api.zhihu.com/collections/contents/answer/{answer_id}?limit=50'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "collection_id",
      "title": "收藏夹标题",
      "is_favorited": true
    }
  ]
}
```

## 添加或移除收藏

把回答或文章加入收藏夹，或从收藏夹移除。

| 字段 | 值 |
| --- | --- |
| 方法 | `PUT` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是；知乎++部分调用路径会显式签名 |

URL：

```text
https://api.zhihu.com/collections/contents/{content_type}/{content_id}
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| content_type | Path | 是 | `answer` 或 `article`。 |
| content_id | Path | 是 | 内容 ID。 |
| add_collections | Form Body | 添加时必填 | 要加入的收藏夹 ID。 |
| remove_collections | Form Body | 移除时必填 | 要移除的收藏夹 ID。 |

zhurl 示例：

```bash
zhurl --web -X PUT \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -d 'add_collections={collection_id}' \
  'https://api.zhihu.com/collections/contents/answer/{answer_id}'
```

脱敏返回示例：

```json
{
  "success": true
}
```

## 创建收藏夹

创建新的收藏夹。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/collections
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| title | JSON Body | 是 | 收藏夹标题。 |
| description | JSON Body | 否 | 收藏夹描述。 |
| is_public | JSON Body | 是 | 是否公开。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  -H 'Content-Type: application/json' \
  -d '{"title":"收藏夹标题","description":"描述","is_public":false}' \
  'https://www.zhihu.com/api/v4/collections'
```

脱敏返回示例：

```json
{
  "collection": {
    "id": "collection_id",
    "title": "收藏夹标题",
    "is_public": false
  }
}
```

## 通知列表

获取通知。知乎++最终会分别请求 `default`、`follow`、`vote_thank` 三个源；
`recent` 是旧的最近通知入口。

| 字段 | 值 |
| --- | --- |
| 方法 | `GET` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/notifications/v2/recent?limit=20
https://www.zhihu.com/api/v4/notifications/v2/default?limit=20
https://www.zhihu.com/api/v4/notifications/v2/follow?limit=20
https://www.zhihu.com/api/v4/notifications/v2/vote_thank?limit=20
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| limit | Query | 否 | 知乎++使用 `20`。 |
| include | Query | 否 | 可请求内容摘要字段。 |
| paging.next | Query | 否 | 翻页 URL。 |

zhurl 示例：

```bash
zhurl --web --jq '.data[0] | {id, content, create_time}' \
  'https://www.zhihu.com/api/v4/notifications/v2/default?limit=20'
```

脱敏返回示例：

```json
{
  "data": [
    {
      "id": "notification_id",
      "create_time": 1790000000,
      "content": {
        "verb": "MEMBER_FOLLOW",
        "text": "通知文案..."
      }
    }
  ],
  "paging": {
    "is_end": false,
    "next": "https://www.zhihu.com/api/v4/notifications/v2/default?offset=next"
  }
}
```

## 标记通知已读

把某类通知全部标记为已读。

| 字段 | 值 |
| --- | --- |
| 方法 | `POST` |
| 模式 | `web` |
| 是否需要登录 | 是 |
| 是否需要 d_c0 签名 | 是 |

URL：

```text
https://www.zhihu.com/api/v4/notifications/v2/default/actions/readall
https://www.zhihu.com/api/v4/notifications/v2/follow/actions/readall
https://www.zhihu.com/api/v4/notifications/v2/vote_thank/actions/readall
```

参数：

| 名称 | 位置 | 必填 | 说明 |
| --- | --- | --- | --- |
| 通知分类 | Path | 是 | `default`、`follow` 或 `vote_thank`。 |

zhurl 示例：

```bash
zhurl --web -X POST \
  'https://www.zhihu.com/api/v4/notifications/v2/default/actions/readall'
```

脱敏返回示例：

```json
{
  "success": true
}
```
