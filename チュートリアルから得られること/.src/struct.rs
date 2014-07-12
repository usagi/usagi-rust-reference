fn main( )
{
  struct Vec3
  { x: f32
  , y: f32
  , z: f32
  }

  let p0 = Vec3 { x: 1.23, y: 2.34, z: 3.45 };
  let p1 = Vec3 { x: 4.56, y: 5.67, z: 6.78 };

  let v1 = Vec3 { x: p1.x - p0.x, y: p1.y - p0.y, z: p1.z - p0.z };

  println!( "v1: x={}, y={}, z ={}", v1.x, v1.y, v1.z );

  println!
   ( "{}"
   , match v1
     { Vec3 { x: 1.11, y: 2.22, z: 3.33 } => "オレガノ"
     , Vec3 { x: 3.33, y:    _, z: 1.11 } => "バジル"
     , Vec3 { y: 1.0..3.0, z: 2.22, ..  } => "クミン"
     , Vec3 { z: zz, .. } if zz < 5.00    => "ピンクペッパー"
     , Vec3 { ..                        } => "ホワイトペッパー"
     }
   );
  
  let Vec3 { x: a, y: b, z: c } = v1;
  println!( "a:{}, b:{}, c:{}", a, b, c );

  // 次の行は翻訳時に警告を受ける
  struct snake_case { snake_field: uint }
  
  println!( "snake: {}", snake_case { snake_field: 123 }.snake_field );
}
