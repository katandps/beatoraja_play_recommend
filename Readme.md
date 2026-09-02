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
docker compose run --service-ports --build --remove-orphans app bash
```

関連コンテナも起動しつつ、rust image の中に入る

Rust のコンパイル結果は Docker volume に保存されるため、初回ビルド後は `target` と取得済み crate のキャッシュを再利用できる。

キャッシュを作り直したい場合は、関連コンテナを停止してから以下を実行する。

```sh
docker compose down --volumes
```

### Docker network の作成に失敗する場合

`DOCKER-FORWARD` が無いというエラーで起動できない場合は、Docker の iptables chain が壊れている可能性がある。
Docker デーモンを再起動してから、もう一度起動コマンドを実行する。

```sh
sudo systemctl restart docker
docker compose run --service-ports --build --remove-orphans app bash
```

WSL や Docker Desktop を使っている場合は、Docker Desktop か WSL 自体を再起動する。
