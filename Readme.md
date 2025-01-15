# Beatoraja Play Recommend

Beatoraja のプレイデータを解析してフォーマットする

## 開発準備

libmysqlclient が必要

- MacOS

```sh
brew install mysql
```

## 開発中

## 起動

```sh
docker-compose run --service-ports --build --remove-orphans app bash
```

関連コンテナも起動しつつ、rust image の中に入る
