fn main( )
{
  {
    // box: ヒープからメモリー確保
    let a = box 123u8;
    println!( "{}", a );
  } // ここで a は解放される。
}
