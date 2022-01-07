use std::env;
use once_cell::sync::Lazy;
use std::sync::Mutex;
// use std::fs::File;
// use std::fs::OpenOptions;
// use std::io::prelude::*;
// use std::io::{BufReader, BufWriter};
use std::path::Path; //, PathBuf};


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


struct Point {
    x: i16,
    y: i16,
    q: u8, // q = 0 : 無効判定
}

struct PointPair {
    id: u128,
    p1: Point,
    p2: Point,
    theta: f64,
    electron: u8,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        Point{x:x, y:y, q: CHARGE_RANGES.lock().unwrap()[x as usize][y as usize] }
    }

    fn valid (&self) -> bool {
        match self.q {
            0 => {false}
            _ => {true}
        }
    }
}


impl PointPair {
    fn valid(&self) -> bool {
        self.p1.valid() && self.p2.valid()
    }

    const p0: f64 = 1_897.366596101; //(2.0 * 1.8E6 as f64).sqrt();
    const r0: f64 = 2.26; //Si
    const length: f64 = 1025.0 / 0.08;

    fn new(id:u128, x1: i16, y1: i16, x2: i16, y2: i16) -> Self{
        let p1 = Point::new(x1, y1);
        let p2 = Point::new(x2, y2);

        let pc: f64 = (p1.q as f64 * p2.q as f64 * 14.4 / Self::r0).sqrt();
        let y1_:f64 = (p1.y + p1.q as i16 * 210).into();
        let y2_:f64 = (p2.y + p2.q as i16 * 210).into();
        let lambda: f64 = ((x1 - x2) as f64 * (x1 - x2) as f64 + (y1_- y2_) * (y1_ - y2_)).sqrt();

        let theta = (Self::p0 / pc * lambda / 2.0 / Self::length).asin() * 180.0 / std::f64::consts::PI;

        PointPair{id: id, p1: p1, p2: p2, theta: theta, electron: 0}
    }
}


type Record = (u64, u16, u16, u8);


/**
 * input   :  {sweep}\t{x}\t{y}\t{brightness}の配列
 * output  :  二次元map
 */
fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    // println!("{:?}", CHARGE_RANGES.lock().unwrap());
    println!("{}", CHARGE_RANGES.lock().unwrap()[0][3]);



    println!("{} 個のフィアルを読み取ります", args.len());
    for path in args {
        let points = extract_pair_points(path);
        println!("{}",points.len());
    }
    Ok(())
}

// 1ファイルごとの処理
fn extract_pair_points(path: String) -> Vec<PointPair> {
    let mut points: Vec<PointPair> = Vec::new();
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .flexible(true)
        .from_path(Path::new(&path));

    match reader {
        Err(err) => {
            panic!("reader 作成でエラー {}", err)
        }
        Ok(reader_) => {
            let mut buffer: Vec<Record> = Vec::new();
            let mut reader = reader_;
            for line in reader.deserialize() {
                match line {
                    Err(_) => {
                        panic!("行の処理でエラー")
                    }
                    Ok(line) => {
                        let record: Record = line;
                        if buffer.len() == 0 || record.0 == buffer[0].0 {
                            buffer.push(record);
                            continue;
                        }
                        // record のid が、buffer にたまっていたものと違った場合

                        if buffer.len() == 2 {
                            let pair = PointPair::new(
                                buffer[0].0 as u128,
                                buffer[0].1 as i16, buffer[0].2 as i16,
                                buffer[1].1 as i16, buffer[1].2 as i16
                            );
                            // if pair.valid() {
                                points.push(pair);
                            // }
                        }
                        buffer.clear();
                        buffer.push(record);
                    }
                }
            }
            for point in &points {
                // if point.theta.is_nan() {
                if point.id == 7363 {
                    println!("{} {} {} {} {} {} {} {}", point.id, point.p1.q, point.p2.q, point.theta, point.p1.x, point.p1.y, point.p2.x, point.p2.y);
                }
            }
            points
        }
    }
}


