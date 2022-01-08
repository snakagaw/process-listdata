use std::env;
use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

mod point_pair;
use point_pair::PointPair;

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
        let mut write_path = PathBuf::from(&path);
        
        
        let filename = String::from(write_path.file_name().unwrap().to_str().unwrap());
        write_path.pop(); write_path.pop();
        write_path.push("2_molaxis");
        write_path.push(filename);

        println!("{:?}", write_path);
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
                            if pair.valid() {
                                points.push(pair);
                            }
                        }
                        buffer.clear();
                        buffer.push(record);
                    }
                }
            }
            // for point in &points {
            //     println!("{} {} {} {}", point.id, point.p1, point.p2, point.theta);
            // }
            points
        }
    }
}


