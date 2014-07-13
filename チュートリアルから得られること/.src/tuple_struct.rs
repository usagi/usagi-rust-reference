fn main( )
{
  // これは構造体 ( struct )
  struct Vec2 { x: f32, y: f32 }
  let a = Vec2 { x: 12.34, y: 56.78 };
  println!( "a: x={},y={}", a.x, a.y );
  
  // これはタプル ( tuple )
  let b = ( 12.34f32, 56.78f32 );
  println!( "{}", b );
  
  // これは列挙体 ( enum )
  enum Pepper { Black, White, Pink, Red }
  println!( "{}", White as uint );

  // これがタプル構造体 ( tuple-struct )
  struct Parameters( u8, u8, u8, u8, u8 );
  let c = Parameters( 3, 1, 4, 5, 1 );
  match c
  { Parameters( str, int, vit, dex, luc )
      => println!
        ( r"c: str={},int={},vit={},dex={},luc={}"
        , str, int, vit, dex, luc
        )
  }
}
