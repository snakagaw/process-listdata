use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::fmt;

pub struct PointPair {
    pub id: u128,
    pub p1: Point,
    pub p2: Point,
    pub theta: f64,
    pub electron: u8,
}

impl PointPair {
    pub fn valid(&self) -> bool {
        self.p1.valid() && self.p2.valid() && !self.theta.is_nan()
    }

    const P0: f64 = 1_897.366596101; //(2.0 * 1.8E6 as f64).sqrt();
    const R0: f64 = 2.26; //Si
    const LENGTH: f64 = 1025.0 / 0.08; // px 単位

    ///
    /// 座標から電荷を割り出し、配向軸を計算する
    ///
    pub fn new(id:u128, x1: i16, y1: i16, x2: i16, y2: i16) -> Self{
        let p1 = Point::new(x1, y1);
        let p2 = Point::new(x2, y2);

        let pc: f64 = (p1.q as f64 * p2.q as f64 * 14.4 / Self::R0).sqrt();
        let y1_:f64 = (p1.y + p1.q as i16 * 210).into();
        let y2_:f64 = (p2.y + p2.q as i16 * 210).into();
        let lambda: f64 = ((x1 - x2) as f64 * (x1 - x2) as f64 + (y1_- y2_) * (y1_ - y2_)).sqrt();

        let theta = (Self::P0 / pc * lambda / 2.0 / Self::LENGTH).asin() * 180.0 / std::f64::consts::PI;

        PointPair{id: id, p1: p1, p2: p2, theta: theta, electron: 0}
    }
}

pub struct Point {
    x: i16,
    y: i16,
    q: u8, // q = 0 : 無効判定
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        Point{x:x, y:y, q: CHARGE_RANGES.lock().unwrap()[x as usize][y as usize] }
    }

    pub fn valid (&self) -> bool {
        match self.q {
            0 => {false}
            _ => {true}
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "({x: >3}, {y: >4}) q={q}", x = self.x, y = self.y, q = self.q)
    }
}

static CHARGE_RANGES: Lazy<Mutex<[[u8; 1024]; 400]>> = Lazy::new(|| {
    let charge_points: [Point; 5] = [
        Point {x:180, y:833, q:1},
        Point {x:180, y:623, q:2},
        Point {x:180, y:414, q:3},
        Point {x:180, y:204, q:4},
        Point {x:180, y:-6, q:5},
    ];
    
    let mut charges: [[u8; 1024]; 400] = [[0; 1024]; 400];

    for x in 0..400 {
        for y in 0..1024 {
            for charge in &charge_points {
                if (x as i32 - charge.x as i32) * (x as i32 - charge.x as i32) + (y as i32 - charge.y as i32) * (y as i32 - charge.y as i32) <= 6400 {
                    charges[x as usize][y as usize] = charge.q;
                }
            }
        }
    }
    Mutex::new(charges)
});