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
