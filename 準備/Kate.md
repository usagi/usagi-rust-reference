## Kate

KDEのKate向けの設定ファイルがRust本体のリポジトリーに含まれています。

- https://github.com/rust-lang/rust/tree/master/src/etc/kate

Kateは単にテキストエディターというだけではなく、
IDE KDevelopのエディターコンポーネントでもあるので、
この設定ファイルをKateに導入する事で
KDevelopでもRustソースコードの編集性が向上します。

Kateのユーザーごとの設定ファイル置き場に、
`rust.xml`を配置すればシンタックスハイライトが動作するようになります。

```zsh
cd ~/.kde/share/apps/katepart/syntax
ln -s ~/repos/rust/src/etc/kate/rust.xml
```

導入後に、Kateで`*.rs`ファイルを扱ってもシンタックスハイライトされない場合は、
Kateのメニューから「ツール」/「モード」と「ツール」/「強調表示」を確認し、
rustを設定します。

より詳細なカスタマイズは、Kateのメニューから「設定」/「Kateの設定」を開き、
「エディターコンポーネント」の「開く/保存」の「モードとファイルタイプ」タブから
ファイルタイプ rust を選び調整します。

### この文章と Rust のバージョン

- 執筆: Rust-0.11.0

### 参考

- https://github.com/rust-lang/rust/tree/master/src/etc/kate
