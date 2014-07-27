fn main( )
{
  let a = 123u;

  match a
  {   0       => println!( "ししとうがらし" )
  , 123 | 456 => println!( "ぴーまん" )
  , 500..2500 => println!( "ぱぷりか" )
  , _         => println!( "はらぺーにょ" )
  }

  println!
    ( "{}"
    , match ( 123u, 456u )
      { ( 123,   t ) if t < 123 => "三鷹"
      , ( 456,   _ )            => "能鷹"
      , (   s, 123 ) if s > 456 => "黄辛"
      , (   _,   _ )            => "万願寺"
      }
    )
  
  let ( x, _, z ) =
    match a
    { s @ 0..100 => ( s + 1,  1, ( s + 1 ) * ( s + 1 ) )
    , s @ _      => ( s - 1, -1, ( s - 1 ) * ( s - 1 ) )
    };

  println!( "x: {} y: {}", x, z );

  let b = &[ 1u, 2, 3, 4, 5 ];
  
  fn print_from_head( a: &[uint] )
  {
    match a
    { [ head, ..tails ]
      => { print!( "{} ", head ); print_from_head( tails ) }
    , []
      => ()
    }
  }

  print_from_head( b );
}
