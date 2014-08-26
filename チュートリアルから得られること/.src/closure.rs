fn main( )
{
  let x = 13;
  
  // 参考: 名前付きの関数の似たコード
  // 次の行は関数では x をキャプチャーする仕様が無いため、
  // コンパイルエラーとなる。
  // fn f( a: int ) -> int { println!( "function {}", x + a ) }
  
  // クロージャー
  // しれっと x を自動的にキャプチャーしている
  let c = | a: int | -> () { println!( "closure {}", x + a ) };
  
  // クロージャーはスコープ外の値をキャプチャーできる
  c( 17 );
  
  // クロージャーは () 以外の return 型も推論できる
  let increment = | a: int | { a + 1 };

  // increment により実引数 1 は 2 となり、
  // c により 2 + 13 が出力される。
  c( increment( 1 ) );
  
  // クロージャーでは本体の実行条件を定義できる例
  let mut max = 0;
  // スコーピングしないと max は
  // クロージャーにキャプチャーされている間 mutable borrowed 
  // となるので、処理に必要なスコープだけスコープ付けしておく。
  {
    // クロージャーの定義で body の前に条件句を定義できる
    let f = | a: int | if a > max { max = a };
    for x in [ 1, 2, 3, 8, 6, 4].iter()
    {
      f(*x);
    }
  }
  println!("max = {}", max);
}

