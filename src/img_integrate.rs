use std::env;
use std::fs::{OpenOptions, File};
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use csv::DeserializeRecordsIter;

mod point_pair;
use point_pair::PointPair;

mod file_names;
use file_names::{cop2mcs, cop2molaxis};

type Record = (u128, u16, u16, u8);
type McsRecord= (u128, u8, u16);

/**
 * input   :  {sweep}\t{x}\t{y}\t{brightness}の配列
 * output  :  二次元map
 */
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数の数が異なります");
    }

    let path = &args[1];
    let write_path = cop2molaxis(&path);
    let w = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(write_path)
        .unwrap();
    let mut writer = BufWriter::new(w);

    let points = extract_pair_points(&path);

    for point in &points {
        write!(writer, "{:05}\t{}\t{}\n", point.id, point.theta, point.electron).expect("書き込みエラーです");
    }
    Ok(())
}

// 1ファイルごとの処理
fn extract_pair_points(path: &String) -> Vec<PointPair> {
    let mut points: Vec<PointPair> = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .flexible(true)
        .from_path(Path::new(&path))
        .unwrap();

    let mut mcs_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .flexible(true)
        .from_path(cop2mcs(&path))
        .unwrap();
    
    let mut mcs_itr: DeserializeRecordsIter<'_, File, McsRecord> = mcs_reader.deserialize();

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

            // 電子の数を数える
            loop {
                let mcs_record = mcs_itr.next().unwrap().unwrap();

                // id がmcs のid と一致しており、かつmcs がSTOP の場合
                if mcs_record.0 == pair.id && mcs_record.1 == 1 {
                    pair.electron += 1;
                }

                // mcs のid の方が大きい場合、次の入射粒子に備えて終了
                if mcs_record.0 > pair.id {
                    break;
                }
            }
            if pair.valid() {
                points.push(pair);
            }
        }
        buffer.clear();
        buffer.push(record);
    }
    points
}
