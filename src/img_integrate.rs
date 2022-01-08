use std::env;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

mod point_pair;
use point_pair::PointPair;

mod file_names;
use file_names::{cop2mcs, cop2molaxis};

type Record = (u64, u16, u16, u8);

/**
 * input   :  {sweep}\t{x}\t{y}\t{brightness}の配列
 * output  :  二次元map
 */
fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    println!("{} 個のフィアルを読み取ります", args.len());
    for path in args {
        let write_path = cop2molaxis(&path);
        let w = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(write_path)
            .unwrap();
        let mut writer = BufWriter::new(w);

        let points = extract_pair_points(path);

        for point in &points {
            write!(writer, "{}\t{}\n", point.id, point.theta).expect("書き込みエラーです");
        }
    }
    Ok(())
}

// 1ファイルごとの処理
fn extract_pair_points(path: String) -> Vec<PointPair> {
    let mut points: Vec<PointPair> = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .flexible(true)
        .from_path(Path::new(&path))
        .unwrap();

    let mut buffer: Vec<Record> = Vec::new();
    for line in reader.deserialize() {
        let record: Record = line.unwrap();
        if buffer.len() == 0 || record.0 == buffer[0].0 {
            buffer.push(record);
            continue;
        }
        // record のid が、buffer にたまっていたものと違った場合

        if buffer.len() == 2 {
            // PoitPair 構造体を生成すると同時に、電荷、分子軸配向を計算
            let mut pair = PointPair::new(
                buffer[0].0 as u128,
                buffer[0].1 as i16,
                buffer[0].2 as i16,
                buffer[1].1 as i16,
                buffer[1].2 as i16,
            );
            pair.electron = 3;
            if pair.valid() {
                points.push(pair);
            }
        }
        buffer.clear();
        buffer.push(record);
    }
    points
}
