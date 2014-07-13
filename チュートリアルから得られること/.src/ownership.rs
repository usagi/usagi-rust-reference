fn main ( )
{
  struct S { x: u8, y: Box< u8 > }

  {
    // a は 構造体 S を所有する
    // a は 構造体 S のフィールドそれぞれも所有する
    let a = S { x: 12, y: box 34 };
    println!( "a: x={},y={}", a.x, a.y );
  } // a がスコープアウトすると
    // 所有していたフィールド a.y もデストラクターが呼ばれる。

  // mut はツリー状に所有するオブジェクト全般へ伝搬する。
  let mut b = S { x: 11, y: box 22 };
  b.x  = 33;
  *b.y = 44;
  println!( "b: x={},y={}", b.x, b.y );
}
