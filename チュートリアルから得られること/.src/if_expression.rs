fn main( )
{
  let a:int;
  
  if false
  { a = 123; }
  else
  { a = 456; }

  println!( "{}", a );

  let b:int =
    if true
    { 123 }
    else
    { 456 }
    ;
  
  println!( "{}", b );
}
