use std::env;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;

use getopts::Options;

mod data;
mod matcher;

use data::Data;
use matcher::{anywhere_re, consecutive_re, match_dist};
use std::io::{Seek, SeekFrom};

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
        let dbp = db_path();
        let cur_dir = env::current_dir().unwrap().to_string_lossy().to_string();
        let mut fp = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(dbp)
            .unwrap();
        let mut db = Data::deserialize_from(&mut fp).unwrap_or_else(|_| Data::new());
        if opt_match.opt_present("a") {
            let path = opt_match.opt_str("a").unwrap();
            db.increase(path);
        } else if opt_match.opt_present("i") {
            db.increase(cur_dir);
        } else if opt_match.opt_present("d") {
            db.decrease(cur_dir);
        } else {
            let num = db.purge();
            println!("Purged {} entries.", num);
        }
        fp.seek(SeekFrom::Start(0)).unwrap();
        db.serialize_into(&mut fp).unwrap();
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

fn db_path() -> PathBuf {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".config");
    if !config_path.exists() {
        fs::create_dir(config_path.as_path()).unwrap();
    }
    config_path.push("rzc.db");
    config_path
}

fn jump(needles: Vec<String>) {
    let db = if let Ok(fp) = File::open(db_path()) {
        if let Ok(db) = Data::deserialize_from(fp) {
            db
        } else {
            print!(".");
            return;
        }
    } else {
        print!(".");
        return;
    };

    let a_re = anywhere_re(&needles, true);
    let c_re = consecutive_re(&needles, true);
    let cur_dir = env::current_dir().unwrap();
    let sorted = db.sorted();
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
    if let Ok(fp) = File::open(db_path()) {
        if let Ok(db) = Data::deserialize_from(fp) {
            print!("{}", db.to_string());
            return;
        }
    }
    eprintln!("There is no database to show.")
}
