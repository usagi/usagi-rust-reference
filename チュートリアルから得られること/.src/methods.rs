fn main()
{
  // 2次元の点を表す構造体
  struct Point
  { x: f64
  , y: f64
  }

  // 二次元の形状を表す列挙体
  enum Shape
  { Circle( Point, f64 )      // 中心点と半径
  , Rectangle( Point, Point ) // 左上点と右下点
  }

  // enum Shape へのメソッドの定義
  impl Shape
  {
    // メソッド
    fn show_area( &self )
    { match *self
      { Circle( _, r )      => println!( "{}", r * r * 3.14159265358979 )
      , Rectangle( p1, p2 ) => println!( "{}", ( p2.x - p1.x ) * (p1.y - p2.y) )
      }
    }
  }
  
  // Circle と Rectangle を定義
  let c = Circle( Point{ x: 2.5, y: 2.5 }, 2.0 );
  let r = Rectangle( Point{ x: 1.23, y: 3.33 }, Point{ x: 5.20, y: -1.10 } );
  
  // Shape のメソッド show_area を呼び出す
  c.show_area();
  r.show_area();
}

