use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Seek;
use std::path::PathBuf;

use getopts::Options;

mod data;
mod matcher;

use data::Data;
use matcher::{anywhere_re, consecutive_re, match_dist};
use once_cell::sync::Lazy;

static DB: Lazy<File> = Lazy::new(|| {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".config");
    if !config_path.exists() {
        fs::create_dir(&config_path).unwrap();
    }
    config_path.push("rzc.db");
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&config_path)
        .expect("Fail to open db.")
});

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt(
        "a",
        "add",
        "add new path to database with default weight",
        "PATH",
    );
    opts.optflag("i", "increase", "increase current directory");
    opts.optflag("d", "decrease", "decrease current directory");
    opts.optflag("s", "stat", "show database, which contain path and weight");
    opts.optflag("", "purge", "remove any non-existent paths from database");
    opts.optflag("h", "help", "print this help");
    opts.optflag("v", "version", "show version information");

    let opt_match = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!("{}", e.to_string()),
    };
    if opt_match.opt_present("h") {
        print_usage(opts);
        return;
    }
    if opt_match.opt_present("v") {
        eprintln!("zc v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if !opt_match.free.is_empty() {
        jump(opt_match.free);
    } else if opt_match.opts_present(&[
        "a".to_string(),
        "i".to_string(),
        "d".to_string(),
        "purge".to_string(),
    ]) {
        let mut data = Data::deserialize_from(&*DB).unwrap_or_else(|_| Data::new());
        let cur_dir = env::current_dir().unwrap().display().to_string();
        if opt_match.opt_present("a") {
            let path = opt_match.opt_str("a").unwrap();
            data.increase(path);
        } else if opt_match.opt_present("i") {
            data.increase(cur_dir);
        } else if opt_match.opt_present("d") {
            data.decrease(cur_dir);
        } else {
            let num = data.purge();
            println!("Purged {} entries.", num);
        }
        (&*DB).seek(std::io::SeekFrom::Start(0)).unwrap();
        DB.set_len(0).unwrap();
        data.serialize_into(&*DB).unwrap();
    } else if opt_match.opt_present("s") {
        show_db();
    } else {
        print_usage(opts);
    }
}

fn print_usage(opts: Options) {
    let brief = format!(
        "{} {}\nUsage: {} [OPTIONS] [PATH...]",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME")
    );
    eprintln!("{}", opts.usage(&brief));
}

fn jump(needles: Vec<String>) {
    let data = if let Ok(data) = Data::deserialize_from(&*DB) {
        data
    } else {
        print!(".");
        return;
    };

    let a_re = anywhere_re(&needles, true);
    let c_re = consecutive_re(&needles, true);
    let cur_dir = env::current_dir().unwrap();
    let sorted = data.sorted();
    let sorted_main: Vec<_> = sorted
        .iter()
        .filter(|(p, _)| {
            let path = PathBuf::from(p);
            path.exists() && path != cur_dir
        })
        .collect();
    let sorted_a = sorted_main.clone();
    let sorted_f = sorted_main.clone();
    let path = sorted_main
        .iter()
        .filter(|(p, _)| c_re.is_match(p))
        .chain(
            sorted_f
                .iter()
                .filter(|(p, _)| match_dist(needles.last().unwrap(), p, true, 0.6)),
        )
        .chain(sorted_a.iter().filter(|(p, _)| a_re.is_match(p)))
        .next();

    if let Some((path, _)) = path {
        print!("{}", path);
    } else {
        print!(".");
    }
}

fn show_db() {
    if let Ok(data) = Data::deserialize_from(&*DB) {
        print!("{}", data.to_string());
    }
}
