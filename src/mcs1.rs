use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

/**
 * input   :  hex
 * output  :  {sweep}\t{channel}\t{time}
 */

 // mcs1 #{input file}
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let read_path = Path::new(&args[1]);
    println!("{}", read_path.display());
    let f = match File::open(read_path) {
        Err(why) => panic!("couldn't open: {}", why.to_string()),
        Ok(file) => file,
    };
    let mut reader = BufReader::new(f);
    let mut filecheck = 0i8;
    loop {
        let mut trush = String::new();
        reader
            .read_line(&mut trush)
            .expect("reading from cursor won't fail");

        if trush.find(";bit0..2: channel# 1..6 ( 3 bit)") == Some(0) {
            filecheck = filecheck + 1
        }
        if trush.find(";bit3: edge 0=up / 1=dn ( 1 bit)") == Some(0) {
            filecheck = filecheck + 1
        }
        if trush.find(";bit4 ..31: timedata    (28 bit)") == Some(0) {
            filecheck = filecheck + 1
        }
        if trush.find(";bit32..47: sweeps      (16 bit)") == Some(0) {
            filecheck = filecheck + 1
        }
        if trush.find(";bit48..62: tag0..tag14 (15 bit)") == Some(0) {
            filecheck = filecheck + 1
        }
        if trush.find(";bit63:     data_lost   ( 1 bit)") == Some(0) {
            filecheck = filecheck + 1
        }

        if trush.find("[DATA]") == Some(0) {
            if filecheck == 6 {
                break;
            } else {
                panic!("ファイル保存形式が違います");
            }
        }
    }
    let mut write_path = PathBuf::from(read_path);
    write_path.set_extension("lstdecoded");
    let w = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(write_path)
        .unwrap();
    let mut writer = BufWriter::new(w);

    let mut add_loop: i64 = 0;
    let mut last_add_loop = 0;
    let mut before_raw: i64 = 0; // before : オーバーフロー補正なし
    let mut before: i64 = 0; // before : オーバーフロー補正なし
    for line_ in reader.lines() {
        let line_str = &line_.unwrap();
        let mut line = i64::from_str_radix(line_str, 16).unwrap();

        let channel = line & 7;
        line = line >> 3;

        let edge = line & 1; // 0:up, 1:down
        line = line >> 1;
        if edge == 0 {
            panic!("立ち上がりが検出されています at {} {}", line_str, before)
        }

        let time = line & 268_435_455;
        line = line >> 28;

        let sweeps_raw = line & 65_535;
        line = line >> 16;

        let _ = line & 32_767;
        line = line >> 15;

        let lost = line & 1;
        if lost == 1 {
            panic!("data lost があります at {}, {}", line_str, before)
        }

        let delta = before_raw - sweeps_raw; //

        // 差が2以上の場合
        if delta < -1 {
            panic!("データが飛んでる at {} {} {}", line_str, sweeps_raw, delta)
        }
        // 通常, 100 - 100 or 100 - 101 で、0以下の自然数になるはず. オーバーフローであれば、FFFF - 0 = 65535 - 0 = 65535 となるはずなので、
        // delta < 65535 は書き込み順序がおかしい．
        if 0 < delta && delta < 65535 {
            panic!("ひっくり返ってる at {} {} {} {} {} ", line_str, sweeps_raw, before_raw, delta, before)
        }
        // 通常, 100 - 100 or 100 - 101 で、0以下の自然数になるはずだが、ひっくり返っていたらオーバーフローと判断
        if delta == 65535 {
            // 最後に add_loop を更新してから 65535 (start だけで2^16 イベント来るはず) カウント程度はadd_loop の更新を抑制する
            if before - last_add_loop < 65535 {
                panic!(
                    "オーバーフローと同時にひっくり返ってる at {} {} {} ",
                    line_str,
                    sweeps_raw,
                    sweeps_raw + add_loop - last_add_loop
                )
            }
            add_loop = add_loop + 65536;
            last_add_loop = add_loop + sweeps_raw;
        }

        let sweeps = add_loop + sweeps_raw;

        write!(writer, "{}\t{}\t{}\n", sweeps, channel, time).expect("error while writing file");

        before_raw = sweeps_raw;
        before = sweeps;
    }
    Ok(())
}
