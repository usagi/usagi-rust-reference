fn main( )
{
  let mut x = box 123u;
  
  // 単なる初期値や代入ではなく move semantics により
  // x が束縛していた box 123u で生成したオブジェクトの所有権が
  // y へと完全に移動する。以後、 x は何も束縛していない状態になる。
  let y = x;
  
  // 次の行は x が既に y へ移動している（move semantics）ため、翻訳時にエラーとなる
  //println!( "x: {}", x );
  
  println!( "y: {}", y );
  
  // x に再び他のオブジェクトを束縛して使う事はできる
  x = box 234u;

  println!( "x: {}", x )

  // Clone トレイトを使用
  #[deriving(Clone)]
  struct S { x: Box<uint> }

  let mut p = S { x: box 345u };
  let mut q = p.clone();
  *p.x +=1;
  *q.x -=1;

  println!( "p.x: {}", p.x );
  println!( "q.x: {}", q.x );
}
