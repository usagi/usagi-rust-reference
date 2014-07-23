fn main()
{
  // < T > でジェネリクスを定義した enum
  // C++で言ったら型引数を template< class T > って定義するような具合
  enum ForwardList< T >
  { Nothing
  , Cons( T, Box< ForwardList< T > > )
  }
  
  // ジェネリクスを使う時には型推論されるので明示しなくて良い
  let a = Cons( 123u, box Cons( 234u, box Nothing ) );
  
  // ジェネリクスを定義し使用した関数
  fn push_front< T >( list: ForwardList< T >, value: T )
    -> ForwardList< T >
  { Cons( value, box list ) }
  
  let mut b = Nothing;
  b = push_front( b, "Alice" );
  b = push_front( b, "Bob"   );
  b = push_front( b, "Carol" );
  
  // ジェネリクスに型制約したい場合は
  // < T: 制約0 [+制約1 [+制約2..]] >
  // ここでは Show トレイトを制約としている
  // 制約を付けないと println! で core::fmt::Show 制約を付けないと
  // だめよと翻訳時エラーとなるが、 core:: ではなく std:: な辺り微注意
  fn print_all< T: std::fmt::Show >( list: ForwardList< T > )
  {
    match list
    { Nothing
        => ()
    , Cons( value, box list )
        => { println!( "print_all: {}", value);
             print_all( list )
           }
    }
  }
  
  println!( "[a]" );
  print_all( a );
  
  println!( "[b]" );
  print_all( b );
}
