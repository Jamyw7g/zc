use std::{env, path::Path};
use std::path::PathBuf;
use std::fs::{self, File};

use getopts::{HasArg, Occur, Options};

mod data;
mod matcher;

use data::DataList;
use matcher::{match_anywhere, match_consecutive, match_fuzzy};


fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    let mut opts = Options::new();

    opts.optopt("a", "add", "add path", "DIR");

    opts.opt("i", "increase", "increase current directory weight", "WEIGHT", 
             HasArg::Maybe, Occur::Optional);
    opts.opt("d", "decrease", "decrease current directory weight", "WEIGHT", 
             HasArg::Maybe, Occur::Optional);

    opts.optflag("s", "stat", "show database entries and their key weights");
    opts.optflag("", "purge", "remove non-existent paths from database");

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "show version information");

    let opt_match = match opts.parse(&args) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string())
    };
    if opt_match.opt_present("h") {
        print_uage(opts);
        return;
    }
    if opt_match.opt_present("v") {
        eprintln!("zc v{}", option_env!("CARGO_PKG_VERSION").unwrap_or("0.1.0"));
        return;
    }

    if !opt_match.free.is_empty() {
        jump(opt_match.free);
    } else {
        if let Some(val) = opt_match.opt_str("a") {
            let mut data = load_data(library_path().as_path());
            data.add(&val);
            data.serialize_into(File::create(library_path().as_path()).unwrap());
        } else 
        if opt_match.opt_present("i") {
            let weight = if let Some(val) = opt_match.opt_str("a") {
                val.parse().unwrap_or(10.0)
            } else {
                10.0
            };
            let mut data = load_data(library_path().as_path());
            let cur_dir = env::current_dir().unwrap();
            data.increase(cur_dir.to_str().unwrap(), weight);
            data.serialize_into(File::create(library_path().as_path()).unwrap());
        } else
        if opt_match.opt_present("d") {
            let weight = if let Some(val) = opt_match.opt_str("a") {
                val.parse().unwrap_or(10.0)
            } else {
                10.0
            };
            let mut data = load_data(library_path().as_path());
            let cur_dir = env::current_dir().unwrap();
            data.decrease(cur_dir.to_str().unwrap(), weight);
            data.serialize_into(File::create(library_path().as_path()).unwrap());
        } else 
        if opt_match.opt_present("s") {
            show_stat();
        } else 
        if opt_match.opt_present("purge") {
            purge();
        }
    }
}

fn print_uage(opts: Options) {
    let brief = "usage: zc [-h] [-a DIR] [-i [WEIGHT]] [-d [WEIGHT]] [-p] [-s] [-v] [DIR...]";
    eprintln!("{}", opts.usage(&brief));
}

fn library_path() -> PathBuf {
    let mut library = dirs::home_dir().unwrap();
    library.push(".config");
    if !library.exists() {
        fs::create_dir(library.as_path()).unwrap();
    }
    library.push("r_zc.db");
    library
}

fn load_data<P: AsRef<Path>>(path: P) -> DataList {
    if path.as_ref().exists() {
        DataList::deserialize_from(File::open(path.as_ref()).unwrap())
    } else {
        DataList::new()
    }
}

fn jump(needles: Vec<String>) {
    let data = load_data(library_path().as_path());

    let one_path = match_consecutive(&needles, &data, true).iter()
        .chain(match_fuzzy(&needles, &data, true, None).iter())
        .chain(match_anywhere(&needles, &data, true).iter())
        .filter(|(p, _)| Path::new(p).to_path_buf() != env::current_dir().unwrap())
        .take(1)
        .cloned()
        .next();

    if let Some(path) = one_path {
        print!("{}", path.0);
    } else {
        print!(".");
    }
}

fn show_stat() {
    let path = library_path();
    if path.exists() {
        let fp = File::open(path.as_path()).unwrap();
        let data = DataList::deserialize_from(fp);
        print!("{}", data.to_string());
    } else {
        eprintln!("can't found database file, please check");
    }
}

fn purge() {
    let path = library_path();
    if path.exists() {
        let fp = File::open(path.as_path()).unwrap();
        let mut data = DataList::deserialize_from(fp);
        let num = data.clean();
        println!("cleanup {} item", num);
        let fp = File::create(path.as_path()).unwrap();
        data.serialize_into(fp);
    } else {
        eprintln!("nothing to cleanup.");
    }
}
