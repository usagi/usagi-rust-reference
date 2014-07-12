fn main( )
{
  let mut a = 0u;
  
  while a < 5
  {
    println!( "while: {}", a );
    a += 1;
  }

  loop
  {
    println!( "loop : {}", a );
    a -= 1;
    if a == 0
    { break; }
  }
  
  for n in range(2u, 6)
  { println!( "for  : {}", n ); }

  for c in "string".chars()
  { println!( "for string: {}", c ); }
}
