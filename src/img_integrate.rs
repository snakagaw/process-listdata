use std::env;
// use std::fs::File;
// use std::fs::OpenOptions;
// use std::io::prelude::*;
// use std::io::{BufReader, BufWriter};
use std::path::Path; //, PathBuf};

/**
 * input   :  {sweep}\t{x}\t{y}\t{brightness}の配列
 * output  :  二次元map
 */

struct Point {
    x: u16,
    y: u16,
}

struct PointPair {
    id: u128,
    p1: Point,
    p2: Point,
    theta: f32,
    electron: u8,
}

type Record = (u64, u16, u16, u8);
// mcs1 #{input file}
fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    println!("{} 個のフィアルを読み取ります", args.len() - 1);
    for path in args {
        process_file(path);
    }
    Ok(())
}

// 1ファイルごとの処理
fn process_file(path: String) -> std::io::Result<()> {
    let mut points: Vec<PointPair> = Vec::new();
    println!("{}", path);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .flexible(true)
        .from_path(Path::new(&path))?;

    let mut buffer: Vec<Record> = Vec::new();

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
                    points.push(PointPair {
                        id: buffer[0].0 as u128,
                        p1: Point {
                            x: buffer[0].1,
                            y: buffer[0].2,
                        },
                        p2: Point {
                            x: buffer[1].1,
                            y: buffer[1].2,
                        },
                        theta: -1.0,
                        electron: 0,
                    });
                }
                buffer.clear();
                buffer.push(record);

                
            }
        }
    }
    println!("{}", points.len());
    for point in points {
        println!(
            "{} {} {} {} {}",
            point.id, point.p1.x, point.p1.y, point.p2.x, point.p2.y
        );
    }
    Ok(())
}
