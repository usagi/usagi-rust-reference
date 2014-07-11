fn main( )
{
  // 整数型
  let a:int  = 0i   // 10進
             + 0x0i // 16進
             + 0o0i //  8進
             + 0b0i //  2進
             ;
  let a:i8   = 0i8;
  let a:i16  = 0i16;
  let a:i32  = 0i32;
  let a:i64  = 0i64;
  
  // 非負整数型
  let a:uint = 0u;
  let a:u8   = 0u8;
  let a:u16  = 0u16;
  let a:u32  = 0u32;
  let a:u64  = 0u64;

  // 浮動小数点数型
  let a:f32  = 0.0e+0f32;
  let a:f64  = 0.0e+0f64;

  // 真偽値型
  let a:bool = true && false;
  
  // 文字型
  let a:char = 'a';
  let a:char = '\t'; // Tab

  // 文字列型
  let a:&str  = "醤油はヒゲタ本膳";
  let a:&str  = r#"サドン
	デス
		ソース"#
    ;

  // ユニット型
  let a:()   = ();
}
