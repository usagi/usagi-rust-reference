fn main( )
{
  enum Atom
  { Hydrogen = 1
  , Helium
  , Lithium
  }
  
  println!
    ( "Hydrogen: {}\nHelium  : {}\nLithium : {}"
    , Hydrogen as int
    , Helium   as int
    , Lithium  as int
    );
  
  enum Red
  { Red           = 0xFF0000
  , Crimson       = 0xDC143C
  }
  
  println!( "Red  : {}", Red as uint );

  // 次の行は翻訳時エラーとなる
  //println!( "Crimson: {}", Crimson as f32 );
  //println!( "Crimson: {}", Crimson as f64 );
  
  // 次の enum 及びその内部で定義する定数は何れも
  // 翻訳時に警告を受ける。
  enum blue
  { azure      = 0xF0FFFF
  , alice_blue = 0xF0F8FF
  }
  
  println!( "azure: {}", azure as uint );
  
  struct Vec2
  { x: f32
  , y: f32
  }
  
  enum Shape
  { Circle( Vec2, f32 )
  , Rectangle( Vec2, Vec2 )
  }
  
  let o0 = Circle( Vec2 { x: 3.0, y: 4.0 }, 2.0 );
  let o1 = Rectangle( Vec2 { x: 0.5, y: 2.0 }, Vec2 { x: 4.0, y: 5.5 } );
  
  fn area( s: Shape ) -> f32
  {
    match s
    { Circle( _, r )
        => std::f32::consts::PI * r * r
    , Rectangle( Vec2 { x: x0, y: y0 }, Vec2 { x: x1, y: y1 } )
        => ( x1 - x0 ) * ( y1 - y0 )
    }
  }
  
  println!( "area of o0: {}", area( o0 ) );
  println!( "area of o1: {}", area( o1 ) );
}
