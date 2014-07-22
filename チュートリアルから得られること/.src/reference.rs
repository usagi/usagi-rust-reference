fn main()
{
  let a = box 123u;
  
  // この関数の仮引数の記述ではムーブセマンティクスとなり、
  // 関数の呼び出し元からオブジェクトの所有権を
  // 関数内の変数 fa へ移す事になる
  fn fa( fa: Box<uint> )
  { println!( "fa: {}", fa ); }
  
  fa( a );
  
  // 次の行は、ムーブセマンティクスにより
  // a は既にオブジェクトを束縛していないため翻訳時エラーとなる
  //println!( "a: {}", a );
  
  let b = box 234u;
  
  fn fb( fb: &Box<uint> )
  { println!( "fb: {}", fb ); }
  
  fb( &b );
  println!( "b: {}", b );
  
  // 関数以外でも & 演算子で参照を扱える。
  let c = box 456u;
  let d:&Box<uint> = &c;
  
  // 参照型に対しては * 演算子で間接参照もできるようだ。
  println!( "c, d, *d: {}, {}, {}", c, d, *d );
  
  // 次の行は c の束縛するオブジェクトに対して、
  // d が参照をかけている状態でムーブセマンティクスは
  // 使えない安全設計になっているため翻訳時エラーとなる。
  //fa( c );
  
  enum UintList
  { Nothing
  , Cons( uint, Box<UintList> )
  }
  
  let p = Cons( 123, box Cons( 234, box Cons( 345, box Nothing ) ) );
  let q = Cons( 123, box Cons( 223, box Cons( 334, box Nothing ) ) );
  
  fn eq( p: &UintList, q: &UintList ) -> bool
  {
    match ( p, q )
    { // マッチングするタプルの中の型は &Nothing 型
      ( &Nothing, &Nothing )
        => true
      // タプル構造体の中の参照型の値の取り出しは ref を使う
    , ( &Cons( pv, box ref next_p), &Cons( qv, box ref next_q ) )
      if pv == qv
        => eq( next_p, next_q )
    , _
        => false
    }
  }
  
  println!( "eq p p: {}", eq( &p, &p ) );
  println!( "eq q q: {}", eq( &q, &q ) );
  println!( "eq p q: {}", eq( &p, &q ) );
  
}
