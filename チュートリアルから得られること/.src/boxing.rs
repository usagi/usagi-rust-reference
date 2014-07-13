fn main( )
{
  let a: u8        = 123u8;
  let b: Box< u8 > = box 123u8;
  let c: Box< u8 > = box a;
  
  assert!( a == *b );
  assert!( b ==  c );
  
  println!( "size_of<uint>     : {}", std::mem::size_of::< uint >() );
  println!( "size_of<Box<uint>>: {}", std::mem::size_of::< Box < uint > >() );

  println!( "size_of<(u64,u64,u64)>: {}", std::mem::size_of::< ( u64, u64, u64 )>() );
  println!( "size_of<(u64,u64,u64)>: {}", std::mem::size_of::< Box<( u64, u64, u64 )>>() );

  #[allow(dead_code)]
  struct S0
  { a: u8, b: u16, c: u32, d: u64, e: f32, f: f64 }
  
  #[allow(dead_code)]
  struct S1
  { a: Box< u8  >, b: Box< u16 >, c: Box< u32 >
  , d: Box< u64 >, e: Box< f32 >, f: Box< f64 >
  }

  println!( "size_of<S0>: {}", std::mem::size_of::< S0 >() );
  println!( "size_of<S0>: {}", std::mem::size_of::< S1 >() );
  println!( "size_of<Box<S0>>: {}", std::mem::size_of::< Box< S0 > >() );
  println!( "size_of<Box<S1>>: {}", std::mem::size_of::< Box< S1 > >() );
}
