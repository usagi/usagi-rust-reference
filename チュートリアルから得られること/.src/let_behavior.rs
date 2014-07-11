fn main( )
{
  let a     = 123i;
  let b:int = 456;
  
  // 次の行はコンパイルできない
  // a = b - a;

  let a = b - a;
  println!( "{}", a );
  
  let mut c = 987i;
  c -= a + b;
  println!( "{}", c );
  
  let x:f32;
  let y;
  
  // 次の行はコンパイルできない
  //println!( "{}", x );

  x = 1.23e+1;
  y = 4.56e-1f32;
  
  println!( "{}", x * y );
}

