## Hello-World

file: [hello.rs](.src/hello.rs)

```rust
fn main( )
{
  println!( "hello?" );
}
```

compile: `rustc hello.rs`

run: `./hello`

result:

> `hello?`

### 得られること

1. 関数リテラルは`fn `で初めてシグニチャとボディーを書けばいいようだ。
1. `println!( )`という標準出力する組み込み関数的なものがあるようだ。
    1. それは特にモジュールの使用宣言や何かが無くても使えるようだ。
    1. `println!`のように`!`が末尾に付くリテラルはRustの世界の「マクロ」らしい。
1. `fn main`がエントリーポイントとしてコンパイルされるようだ。

### おまけ

未定義のマクロを使おうとするとコンパイルエラーが発生する。

file: [hoge.rs](.src/hoge.rs)

```rust
fn main( )
{
  hoge!( "error occurred maybe." );
}
```

- `rustc hoge.rs`

> ```
hoge.rs:3:3: 3:7 error: macro undefined: 'hoge'
hoge.rs:3   hoge!( "error occurred maybe." );
            ^~~~
error: aborting due to previous error
```

### この文章と Rust のバージョン

- 執筆: Rust-0.11.0

### 参考

- http://doc.rust-lang.org/tutorial.html#compiling-your-first-program
