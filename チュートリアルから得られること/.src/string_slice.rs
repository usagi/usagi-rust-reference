fn main()
{
  // &str で文字列型、""でリテラルを扱う
  let a: &str = "海苔の佃煮おいしい";
  println!( "{}", a );

  // array 同様に slice できる。
  // しかしこの方法ではインデックスを
  // バイト単位で与える必要があるようだ。
  let b: &str = a.slice( 0, 6 );
  println!( "{}", b );

  // 文字列の array
  let c: [ &str, ..3 ] = [ "abc", "def", "xyz" ];
  // 文字列の array の
  // スライスの0番目の
  // スライス（つまり文字に行き着く）
  let d = c.slice( 1, 2 )[0].slice( 1, 2 );
  println!( "{}", d );
}
