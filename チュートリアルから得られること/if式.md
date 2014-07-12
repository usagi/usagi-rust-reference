## if式

file: [if_expression.rs](.src/if_expression.rs)

```rust
fn main( )
{
  let a:int;
  
  if false
  { a = 123; }
  else
  { a = 456; }

  println!( "{}", a );

  let b:int =
    if true
    { 123 }
    else
    { 456 }
    ;
  
  println!( "{}", b );
}
```

result:
> ```zsh
456
123
```

### 得られること

1. `if-else` は「式」( expression )のようだ。
    1. 式のブレースの内部でセミコロンを書かなければ関数と`return`の組み合わせのような挙動を示すようだ。

### おまけ

1. `if ( true )`などと括弧を付けると、"warning: unnecessary parentheses around `if`"と警告を受けてしまう。Rustではif式の条件を括弧で囲む文法は不要のようだ。

### 関連の強い頁

1. [制御構造-条件分岐](制御構造-条件分岐.md)

### この文章と Rust のバージョン

- 執筆: Rust-0.11.0

### 参考

- http://doc.rust-lang.org/tutorial.html#expressions-and-semicolons
