fn main( )
{
  let a:uint = 0x0F;
  let b:uint = 1 + 2 + 4 + 8;
  assert!( a == b );
  assert_eq!( a, b );
  
  // 次の assert! または assert_eq! は失敗し翻訳は停止する。
  //let c:int = 15;
  //assert!( a == c );
  //assert_eq!( a, c );
  
  let d = box 0b1111u;
  let e = box 0o17u;
  assert!( d == e );
  assert!( a == *d );
  assert!( a == *e );
  assert!( *d == *e );
  assert_eq!( d, e );
  assert_eq!( a, *d );
  assert_eq!( a, *e );
  assert_eq!( *d, *e );
}
