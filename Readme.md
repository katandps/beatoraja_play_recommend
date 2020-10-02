# beatoraja_play_recommend
beatorajaのプレイデータを読み込み、以下の内容を出力する
リコメンド3件(最近更新していない曲)
クリアランプ表
クリアランク表

## 使い方
1. コンフィグファイル(`config.toml`)を編集する
    - 書き換える必要があるのは `score_db_url` と `scorelog_db_url` と `song_db_url` を書き換える
    - それぞれ、beatorajaの `score.db` `score_log.db` `songdata.db` に相当する
3. 難易度表を追加する
    - `TABLE_URL{}`の`{}`の部分を変えて追加する(上限99)
    - 一部の難易度表については現状非対応(現行Overjoy公式など)
4. 起動する
    - 初回起動時は表の読み込みのため、時間がかかる
    - 表のリストと操作キーが表示される
        - 表示されない表については非対応
    - `:` の左にある文字を入力し、改行すると対応した処理が行われる
        - 現在は各難易度ごとのリコメンド3件、クリアランプ表、クリアランク表の出力

## ビルド方法
`sqlite3` が必要
環境によってインストール方法が大きく異なるので各自インストールされたし

## ライブラリとして利用
lib.rsを参照のこと
Repositoryを利用するなどしてデータを用意することで解析結果を出力できる

## やりたいこと
- フォーマットの正しくない表の読み込み(Overjoy難易度表など)
- 表示する/フィルターに使う曲の成績の詳細化
- フィルターのカスタマイズ(日数、BPのみ、スコアのみ など)
  - フロントエンドでフィルタリングできるような情報を返せるようにする
  - リコメンドのフィルターは別途実装
- Webアプリへのスコアのアップロード
