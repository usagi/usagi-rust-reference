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

  // 次の行は翻訳時にエラーとなる
  // fn fy() { 123u }

  fn fy() { }
  
  fy();

  fn fz( a:uint ) -> uint
  {
    return match a
    { a if a < 100 => 123
    , _            => 456
    };
  }

  println!( "{}", fz( 384 ) );
  
  fn fp( ) { println!( "fp: unit?" ); }
  
  match fp() { () => println!( "fp: unit!" ) };
}

// グローバルスコープの後方参照
fn f4( a: int ) -> int { a + 8 }
