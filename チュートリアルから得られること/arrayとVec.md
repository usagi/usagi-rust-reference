## arrayとVec

### 注意

- 現行版チュートリアルで "vector" と呼ばれ、この文書でもそれに従っているものは最新版では "array" と称するように変更されたらしい。
    - https://twitter.com/omasanori/status/493133918342610945

### source

file: [array_and_Vec.rs](.src/array_and_Vec.rs)

```rust
fn main()
{
  // ベクター（vector）型 は C++ の std::array<T, N> に近い型
  // （std::vector<T>とはあまり近くないので注意が必要）
  
  // [ ... ] 自体が形名と同じ
  let a = [ 1u, 3, 5 ];
  // [ X , .. N ] で N 個の X を含むベクター
  let b = [ 1u, ..3 ];
  
  // ベクター型は operator[] でインデックスアクセスを直接できる
  println!( "a: {} {} {}", a[0], a[1], a[2] );

  // for-in に掛ける場合には .iter() する必要がある
  print!( "b: " );
  for e in b.iter() { print!( "{} ", e ); }
  println!( "" );
  
  // ベクター型と大変紛らわしい別の型として、
  // 可変長の Vec<T> 型もある
  // vec!{ } マクロで Vec<T> 型のオブジェクトを生成できる
  let c: Vec<uint> = vec!( 2u, 4, 6 );
  
  // 次の行は、可変長と言っても mut していなければ
  // 変更できないので翻訳時エラーとなる
  //c.push( 8 );
  
  // Vec<T> も for-in に放り込むには .iter() する必要がある
  print!( "c: " );
  for e in c.iter() { print!( "{} ", e ); }
  println!( "" );
  
  // マクロの引数リストの囲みは ( ) でも { } でも構わないようだ。
  let mut d: Vec<uint> = vec!{ 1u, 2 };
  
  // mut な Vec<T> には push で要素を1つずつ末尾に追加できる
  d.push( 3 );
  d.push( 4 );
  d.push( 5 );
  
  // pop で末尾の要素を1つずつ除去できる
  println!( "d.pop: {}", d.pop() );
  println!( "d.pop: {}", d.pop() );

  print!( "d: " );
  for e in d.iter() { print!( "{} ", e ); }
  println!( "" );
  
  // clear で全ての要素を除去できる
  d.clear();
  
  // pop の return は
  // enum ?<T> { Some( T ), None }
  // みたいなことになっているようだ。
  println!( "d.pop: {}", d.pop() );
}
```

result:
```zsh
a: 1 3 5
b: 1 1 1 
c: 2 4 6 
d.pop: Some(5)
d.pop: Some(4)
d: 1 2 3 
d.pop: None
```

### 得られること

1. ベクター（vector）という C++ でいえば `std::array<T,N>` に近い型があるようだ。
    1. `[ 1u, 2, 3 ]` を vector 型と言い、型の表記とリテラルが同じようだ。
    1. `[ 2u, ..3 ]` とすると `2u` を 3個 含んだ vector となるようだ。
1. `Vec<T>`型という C++ でいえば `std::vector<T>` に近い型があるようだ。
    1. `mut` ならば可変長で `push` や `pop` や `clear` できるようだ。
        1. `pop` の return は `enum` で `Some(x:T)` または `None` となり安全なようだ。
1. ベクターも `Vec<T>` も `.iter()` しないと for-in に放り込めないようだ。

### 関連の強い頁

1. [制御構造-ループ](制御構造-ループ.md)

### この文章と Rust のバージョン

- 執筆: Rust-0.11.0

### 参考

- http://doc.rust-lang.org/tutorial.html#vectors-and-strings
- https://github.com/rust-lang/rust/wiki/Rust-for-CXX-programmers#vectors--arrays

