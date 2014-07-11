## ローカル変数とletの性質

### letによるローカル変数の定義

file: [let.rs](.src/let.rs)

```rust
fn main( )
{
  let a = 123i;
  let b = 456i;
  let c = a + b;
  println!( "{}", c );
}
```

result:

> `579`

### let の性質

file: [let_behavior.rs](.src/let_behavior.rs)

```rust
fn main( )
{
  let a     = 123i;
  let b:int = 456;
  
  // 次の行はコンパイルできない
  // a = b - a;

  let a = b - a;
  println!( "{}", a );
  
  let mut c = 987i;
  c -= a + b;
  println!( "{}", c );
  
  let x:f32;
  let y;
  
  // 次の行はコンパイルできない
  //println!( "{}", x );

  x = 1.23e+1;
  y = 4.56e-1f32;
  
  println!( "{}", x * y );
}


```

result:

> ```zsh
333
198
5.6088
```

### 得られること

1. ローカル変数を`let`で定義できるようだ。
    1. `let variable_name = 123i;` リテラルによって型が明示された初期値を与えて型推論を伴って変数を定義できるようだ。
    1. `let variable_name:int = 123;` 型を明示して変数を定義できるようだ。
    1. `let variable_name:int;` 未定義の変数も宣言できるようだ。
        1. 未初期化のローカル変数を参照しようとすると"error: use of possibly uninitialized variable"が発生してコンパイルは停止するので安全なようだ。
        1. 未初期化のローカル変数は①宣言時に型を明示するか、②値の定義時に型が明示的に定義されたリテラルを使って型が明確にならなければ"error: cannot determine a type for this local variable: unconstrained type"が発生してコンパイルは停止するので安全なようだ。
1. `let`により定義された変数は「不変」( immutable )となるようだ。
    1. もし変更するようなコードを書けば"re-assignemnt of immutable variable"が発生してコンパイルできないようだ。安全。
    1. `let mut x`のように`mut`を付けると「可変」となるようだ。
1. `let`によりスコープ内で繰り返し同じシンボル名を使った場合には「再定義」となるようだ。

### おまけ

この文書の参考として後述の参考の1つ目に挙げる
チュートリアル[syntac-basics](http://doc.rust-lang.org/tutorial.html#syntax-basics)
の段階では`let`、あるいはまた`fn main`の`fn`など、
これらは宣言や定義といったプログラミング言語上の構文だと多くのC系言語経験者は思うかも。

でも実は`let`や`fn`、
その他にもこれから先で学ぶ事になる`traits`や`enum`や`types`、
それに`static`などは何れも「式」だったりするようだ。

これについては後述の参考の2つ目に挙げる
チュートリアル[Expressions and semicolons](http://doc.rust-lang.org/tutorial.html#expressions-and-semicolons)
の最後でしれっと触れている。

### この文章と Rust のバージョン

- 執筆: Rust-0.11.0

### 参考

- http://doc.rust-lang.org/tutorial.html#syntax-basics
- http://doc.rust-lang.org/tutorial.html#expressions-and-semicolons
