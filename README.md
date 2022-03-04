The Rust Programming Language 日本語版 写経
===

https://doc.rust-jp.rs/book-ja/


- [x] 1. 事始め
- [x] 1.1. インストール
- [x] 1.2. Hello, World!
- [x] 1.3. Hello, Cargo!
- [x] 2. 数当てゲームをプログラムする
- [ ] 3. 一般的なプログラミングの概念
- [x] 3.1. 変数と可変性
- [x] 3.2. データ型
- [x] 3.3. 関数
- [x] 3.4. コメント
- [ ] 3.5. フロー制御
- [ ] 4. 所有権を理解する
- [ ] 4.1. 所有権とは？
- [ ] 4.2. 参照と借用
- [ ] 4.3. スライス型
- [ ] 5. 構造体を使用して関係のあるデータを構造化する
- [ ] 5.1. 構造体を定義し、インスタンス化する
- [ ] 5.2. 構造体を使ったプログラム例
- [ ] 5.3. メソッド記法
- [ ] 6. Enumとパターンマッチング
- [ ] 6.1. Enumを定義する
- [ ] 6.2. matchフロー制御演算子
- [ ] 6.3. if letで簡潔なフロー制御
- [ ] 7. 肥大化していくプロジェクトをパッケージ、クレート、モジュールを利用して管理する
- [ ] 7.1. パッケージとクレート
- [ ] 7.2. モジュールを定義して、スコープとプライバシーを制御する
- [ ] 7.3. モジュールツリーの要素を示すためのパス
- [ ] 7.4. useキーワードでパスをスコープに持ち込む
- [ ] 7.5. モジュールを複数のファイルに分割する
- [ ] 8. 一般的なコレクション
- [ ] 8.1. ベクターで一連の値を保持する
- [ ] 8.2. 文字列で `UTF-8` でエンコードされたテキストを保持する
- [ ] 8.3. キーとそれに紐づいた値をハッシュマップに格納する
- [ ] 9. エラー処理
- [ ] 9.1. panic!で回復不能なエラー
- [ ] 9.2. Resultで回復可能なエラー
- [ ] 9.3. panic!すべきかするまいか
- [ ] 10. ジェネリック型、トレイト、ライフタイム
- [ ] 10.1. ジェネリックなデータ型
- [ ] 10.2. トレイト：共通の振る舞いを定義する
- [ ] 10.3. ライフタイムで参照を検証する
- [ ] 11. 自動テストを書く
- [ ] 11.1. テストの記述法
- [ ] 11.2. テストの実行のされ方を制御する
- [ ] 11.3. テストの体系化
- [ ] 12. 入出力プロジェクト：コマンドラインプログラムを構築する
- [ ] 12.1. コマンドライン引数を受け付ける
- [ ] 12.2. ファイルを読み込む
- [ ] 12.3. リファクタリングしてモジュール性とエラー処理を向上させる
- [ ] 12.4. テスト駆動開発でライブラリの機能を開発する
- [ ] 12.5. 環境変数を取り扱う
- [ ] 12.6. 標準出力ではなく標準エラーにエラーメッセージを書き込む
- [ ] 13. 関数型言語の機能：イテレータとクロージャ
- [ ] 13.1. クロージャ：環境をキャプチャできる匿名関数
- [ ] 13.2. 一連の要素をイテレータで処理する
- [ ] 13.3. 入出力プロジェクトを改善する
- [ ] 13.4. パフォーマンス比較：ループVSイテレータ
- [ ] 14. CargoとCrates.ioについてより詳しく
- [ ] 14.1. リリースプロファイルでビルドをカスタマイズする
- [ ] 14.2. Crates.ioにクレートを公開する
- [ ] 14.3. Cargoのワークスペース
- [ ] 14.4. cargo installでCrates.ioからバイナリをインストールする
- [ ] 14.5. 独自のコマンドでCargoを拡張する
- [ ] 15. スマートポインター
- [ ] 15.1. ヒープのデータを指すBox<T>を使用する
- [ ] 15.2. Derefトレイトでスマートポインターを普通の参照のように扱う
- [ ] 15.3. Dropトレイトで片付け時にコードを走らせる
- [ ] 15.4. Rc<T>は、参照カウント方式のスマートポインター
- [ ] 15.5. RefCell<T>と内部可変性パターン
- [ ] 15.6. 循環参照は、メモリをリークすることもある
- [ ] 16. 恐れるな！並行性
- [ ] 16.1. スレッドを使用してコードを同時に走らせる
- [ ] 16.2. メッセージ受け渡しを使ってスレッド間でデータを転送する
- [ ] 16.3. 状態共有並行性
- [ ] 16.4. SyncとSendトレイトで拡張可能な並行性
- [ ] 17. Rustのオブジェクト指向プログラミング機能
- [ ] 17.1. オブジェクト指向言語の特徴
- [ ] 17.2. トレイトオブジェクトで異なる型の値を許容する
- [ ] 17.3. オブジェクト指向デザインパターンを実装する
- [ ] 18. パターンとマッチング
- [ ] 18.1. パターンが使用されることのある箇所全部
- [ ] 18.2. 論駁可能性：パターンが合致しないかどうか
- [ ] 18.3. パターン記法
- [ ] 19. 高度な機能
- [ ] 19.1. Unsafe Rust
- [ ] 19.2. 高度なトレイト
- [ ] 19.3. 高度な型
- [ ] 19.4. 高度な関数とクロージャ
- [ ] 19.5. マクロ
- [ ] 20. 最後のプロジェクト：マルチスレッドのWebサーバを構築する
- [ ] 20.1. シングルスレッドのWebサーバを構築する
- [ ] 20.2. シングルスレッドサーバをマルチスレッド化する
- [ ] 20.3. 正常なシャットダウンと片付け
- [ ] 21. 付録
- [ ] 21.1. 付録A：キーワード
- [ ] 21.2. 付録B：演算子と記号
- [ ] 21.3. 付録C：導出可能なトレイト
- [ ] 21.4. 付録D：便利な開発ツール
- [ ] 21.5. 付録E：Edition
- [ ] 21.6. 付録F：本の翻訳
- [ ] 21.7. 付録G：Rustの作られ方と“Nightly Rust”
