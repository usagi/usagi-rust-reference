fn main( )
{
  let a = format!( "hello, {}!", 12345u );
  println!( "{}", a );
  println!( "{}", a );
  println!( "{}", a );
  
  print!( "{}", a );
  print!( "{}", a );
  print!( "{}", a );
}
