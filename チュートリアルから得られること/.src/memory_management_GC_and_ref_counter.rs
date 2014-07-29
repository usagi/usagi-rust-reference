fn main()
{

  // std::rc::Rc::new というのが参照カウンター方式のメモリー管理オブジェクトで、
  // C++ における std::shared_ptr を std::make_shared する相当のようだ。
  let a = std::rc::Rc::new( 123u );
  println!( "a   : {}", a );
  // 参照カウントが +1 される
  let b = a.clone();
  println!( "a, b: {}, {}", a, b );
  // オブジェクトの所有権の移動
  let c = b;
  println!( "c   : {}", c );

  // box の代わりに box(std::gc::GC) を使うと
  // ガベージコレクションを利用できるようだ。
  let x = box( std::gc::GC ) 456u;
  println!( "x   : {}", x );
  // GCを利用するオブジェクトはムーブとならないようだ。
  let y = x;
  println!( "x, y: {} {}", x, y );

  // Rc と GC は何れも格納した内部の値については不変で、
  // たとえ次の行のように mut を用いても内容を変更できず、
  // 翻訳時エラーとなるようだ。
  //let mut p = a;
  //*p += 1;
  //let mut q = x;
  //*q += 1;
}
