## Vim

Rust本体のリポジトリーにVim用のプラグインが含まれています。

- https://github.com/rust-lang/rust/tree/master/src/etc/vim

NeoBundleを使っている場合には次のようにして使います。

file: .vimrc

```vim
NeoBundle 'vim', { 'type' : 'nosync', 'base' : '~/repos/rust/src/etc' }
```

この設定の書き方では、`~/repos/rust`にRustのGitHubリポジトリーをcloneしてある想定で、
その中の`src/etc`をbaseに、その中の`vim`をNeoBundleにプラグインとしてロードさせ、
`:NeoBundleUpdate`等で同期しないよう`'type' : 'nosync'`もセットしています。

しかし、これだけだと、`*.rs`ファイルをVimで開いた際に、
ファイルタイプが上手く扱われないようで期待した対応が得られないかもしれません。

より具体的にはVimが標準でサポートする`hercules.vim`が`*.rs`も
対象のファイルタイプとして定義しているために、`*.rs`を開いても
filetypeがrustではなくherculesとして認識されてしまい期待通りとなりません。

簡単な対応として、`.vimrc`に、次の対応を追記します。

```vim
au BufNewFile,BufRead,BufEnter *.rs setf rust
```

`BufNewFile`と`BufRead`に加えて`BufEnter`もハンドルする事で、
`*.rs`への`setf rust`を行います。

以上、

1. Rust本体に含まれるVimプラグインのロード
1. `BufEnter`をハンドルして`*.rs`に対して`setf rust`を行うこと

この2点でVimで快適なRust生活を送る基礎が整います。

### おまけ

1.[thinca/vim-quickrun](https://github.com/thinca/vim-quickrun)もRust対応済み。
    - [Merge pull request #22 from mattn/mozilla-rust](https://github.com/thinca/vim-quickrun/commit/0cebb7fb41447b3f909c2a48035340b31d8c4322)

### この文章と Rust のバージョン

- 執筆: Rust-0.11.0

### 参考

- https://github.com/rust-lang/rust/tree/master/src/etc/vim
