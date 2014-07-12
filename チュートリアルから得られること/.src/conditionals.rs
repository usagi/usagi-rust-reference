fn main( )
{
  let x = 123u;
  
  if x < 123u
  { println!( "ごーやー" ); }
  else if x > 123u
  { println!( "苦瓜" ); }
  else
  { println!( "蔓茘枝" ); }

  println!
    ( "{}"
    , if      x < 123u { "ごーやー" }
      else if x > 123u { "苦瓜"     }
      else             { "蔓茘枝"   }
    );

  // 次の行はコンパイルエラーとなる
  // if   x  { println!( "にがうり" ); }

  // 次の行はコンパイルエラーとなる
  // if true println!( "つるれいし" );
}
