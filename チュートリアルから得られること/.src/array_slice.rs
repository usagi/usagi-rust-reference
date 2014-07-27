fn main()
{
  let a = [ 1u, 2, 3, 4, 5 ];
  
  // array は C++ でいう std::valarray のように
  // slice によってスライス（slice）を生成できる。
  // スライスの型は &[T] となる。
  let first_index = 1u;
  let last_index  = 4u;
  let s: &[uint] = a.slice( first_index, last_index );
  print!( "s: " );
  for v in s.iter() { print!( "{} ", v ); }
  println!( "" );

  let mut b = [ 2u, 4, 6, 8, 10 ];
  // mut_slice を使うと mut なスライスを生成できる
  // このとき、 let mut t とする必要はないようだ。
  let t = b.mut_slice( first_index, last_index );
  t[1] = 9;
  print!( "t: " );
  for v in t.iter() { print!( "{} ", v ); }
  println!( "" );
}
