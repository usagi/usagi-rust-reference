## assert

file: [assert.rs](.src/assert.rs)

```rust
fn main( )
{
  let a:uint = 0x0F;
  let b:uint = 1 + 2 + 4 + 8;
  assert!( a == b );
  assert_eq!( a, b );
  
  // 次の assert! または assert_eq! は失敗し翻訳は停止する。
  //let c:int = 15;
  //assert!( a == c );
  //assert_eq!( a, c );
  
  let d = box 0b1111u;
  let e = box 0o17u;
  assert!( d == e );
  assert!( a == *d );
  assert!( a == *e );
  assert!( *d == *e );
  assert_eq!( d, e );
  assert_eq!( a, *d );
  assert_eq!( a, *e );
  assert_eq!( *d, *e );
}
```

### 得られること

1. `assert!( condition )` で翻訳時のアサーションを実装できるようだ。
1. `assert_eq!( o0, o1 )` で `assert!( o0 == o1 )` となるようだ。

### おまけ

1. ボックス化されたオブジェクト同士の`==`は自動的にボックス解除した値での等値判断となるようだ。

### この文章と Rust のバージョン

- 執筆: Rust-0.11.0

### 参考

- http://doc.rust-lang.org/tutorial.html#operators
- http://doc.rust-lang.org/tutorial.html#boxes
