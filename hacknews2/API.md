# Hacker News API

## Get Top Stories

### Request

```curl

GET https://hacker-news.firebaseio.com/v0/topstories.json
```

### Response

```json
[
    1,
    2,
    3
]
```

## Get Story

* request

```curl
GET https://hacker-news.firebaseio.com/v0/item/{{id}}.json
```

* response

```json
{
  "by": "emmorts",
  "descendants": 55,
  "id": 41428705,
  "kids": [
    41429127,
    41429414,
    41428997,
    41429266,
    41429192,
    41428857,
    41430034,
    41430223,
    41429175,
    41429447,
    41430257,
    41430071,
    41428875,
    41429938,
    41429729,
    41430120,
    41430005,
    41429544,
    41429601,
    41428907,
    41428882,
    41429292,
    41429407,
    41430219,
    41429583,
    41429290,
    41429649,
    41429255,
    41430080,
    41429258,
    41429679,
    41429937,
    41429084,
    41429499,
    41429002,
    41428925,
    41429554
  ],
  "score": 232,
  "time": 1725310260,
  "title": "The Art of Finishing",
  "type": "story",
  "url": "https://www.bytedrum.com/posts/art-of-finishing/"
}
```

## Get Comments

* request

```curl
GET https://hacker-news.firebaseio.com/v0/item/{{id}}.json
```

* response

```json

  "by": "solomonb",
  "id": 41429127,
  "kids": [
    41430355,
    41430275,
    41429671,
    41429946,
    41429591
  ],
  "parent": 41428705,
  "text": "I relate heavily to the author&#x27;s dilemma. ...",
  "time": 1725314395,
  "type": "comment"
}
```

## Get User

* request

```curl
GET https://hacker-news.firebaseio.com/v0/user/{{user_id}}.json
```

* response

```json

{
  "about": "...",
  "created": 1575317518,
  "id": "xxx",
  "karma": 545,
  "submitted": [
  ]
}
```
