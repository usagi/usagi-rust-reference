// グローバルスコープの前方参照
fn f3( a: int ) -> int { f4( a + 4 ) }

fn main( )
{
  // スコープ内の前方参照
  fn f2( a:int ) -> int { f3( a + 2 ) }

  // f1(+1) --> f2(+2) --> f3(+4) --> f4(+8)
  println!( "{}", f1( 0 ) );

  // スコープ内の後方参照
  fn f1( a:int ) -> int { f2( a + 1 ) }

  // C++なら auto fn() -> void {}; に相当する関数
  fn fx() -> () { }
  
  match fx() { () => println!( "unit!" ) }
}

// グローバルスコープの後方参照
fn f4( a: int ) -> int { a + 8 }
