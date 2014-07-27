## arrayとスライス

file: [array_slice.rs](.src/array_slice.rs)

```rust
fn main()
{
  let a = [ 1u, 2, 3, 4, 5 ];
  
  // array は C++ でいう std::valarray のように
  // slice によってスライス（slice）を生成できる。
  // スライスの型は &[T] となる。
  let first_index = 1u;
  let last_index  = 4u;
  let s: &[uint] = a.slice( first_index, last_index );
  print!( "s: " );
  for v in s.iter() { print!( "{} ", v ); }
  println!( "" );

  let mut b = [ 2u, 4, 6, 8, 10 ];
  // mut_slice を使うと mut なスライスを生成できる
  // このとき、 let mut t とする必要はないようだ。
  let t = b.mut_slice( first_index, last_index );
  t[1] = 9;
  print!( "t: " );
  for v in t.iter() { print!( "{} ", v ); }
  println!( "" );
}
```

result:
```zsh
s: 2 3 4 
t: 4 9 8 
```

### 得られること

1. `[ T, ..N ]`（但し Tは型、 N 要素数）な array （以前はvectorと呼ばれていたそれ）に対して `.slice( F, L )`（但し、FとLはスライスの始端と終端とするスライス元のインデックス）を適用する事で `&[T]` スライス（slice）型を生成できるようだ。
    1. C++でいうと `std::valarray` とその `operator[]` により生成できる `std::slice_array` のような関係。
1. `mut_slice`を使うと mut なスライスを生成できるようだ。
    1. この時、`let mut t = ` などとする必要はないようだ。

### 関連の強い頁

1. [arrayとVec](arrayとVec.md)

### この文章と Rust のバージョン

- 執筆: Rust-0.11.0

### 参考

- http://doc.rust-lang.org/tutorial.html#vectors-and-strings
