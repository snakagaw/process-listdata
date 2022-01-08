use regex::Regex;
use std::env;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

mod mylib;

type Record = (u128, f64, u16);

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("引数の数が異なります");
    }

    args.remove(0);

    let mut parallels = Spectrum::new();
    let mut perpendiculars = Spectrum::new();

    let se_regex = Regex::new(r"se(\d{3})").unwrap();
    for path in &args {
        let read_path = PathBuf::from(&path);
        let filename = read_path.file_name().unwrap().to_str().unwrap();
        let se: u16 = se_regex.captures(filename).unwrap()[1].parse().unwrap();

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .flexible(true)
            .from_path(read_path)
            .unwrap();
        for event_ in reader.deserialize() {
            let event: Record = event_.unwrap();
            match event.1 {
                x if 0.0 <= x && x <= 30.0 => parallels.check(se as u64, event.2 as u64),
                x if 60.0 <= x && x <= 90.0 => perpendiculars.check(se as u64, event.2 as u64),
                _ => {}
            }
        }
    }

    let analyzer_folder = mylib::file_names::analyzed_folder(&args[0]);
    let mut perpendicular_path = analyzer_folder.clone();
    perpendicular_path.push("perpendicular");
    let mut parallel_path = analyzer_folder.clone();
    parallel_path.push("parallel");


    let mut perpendicular_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(perpendicular_path)
        .unwrap();
    write!(perpendicular_file, "{}", perpendiculars).unwrap();

    let mut parallel_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(parallel_path)
        .unwrap();
    write!(parallel_file, "{}", parallels).unwrap();
    Ok(())
}

struct Spectrum {
    starts: [u64; 31],
    stops: [u64; 31], // TODO: 行数を自動で読みたい
}

impl Spectrum {
    pub fn new() -> Self {
        Spectrum {
            starts: [0; 31],
            stops: [0; 31],
        }
    }

    pub fn check(&mut self, energy: u64, stop_count: u64) {
        self.starts[energy as usize] += 1;
        self.stops[energy as usize] += stop_count;
    }
}

impl fmt::Display for Spectrum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut a: String = "".to_string();
        for i in 0..=30 {
            let oneline = &format!("{}\t{}\t{}\n", i, self.starts[i], self.stops[i]);
            a = a + oneline;
        }
        a.pop();
        write!(f, "{}", a)
    }
}
