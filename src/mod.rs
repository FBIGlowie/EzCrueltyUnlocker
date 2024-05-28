use std::{io::Write, process::exit};

use colored::Colorize;
use serde_json::*;

const FISH_TICKERS: [&str; 73] = [
    "FLSH", "CIV", "BRNY", "POOL", "BRST", "DRMP", "GLOM", "SMLL", "BIG", "FISH", "NGHT", "ZOOP",
    "CARB", "MOON", "EEL", "FLND", "GFLD", "BLRP", "HDRA", "PSYK", "BNCY", "UNI", "NOCT", "HUMN",
    "BUBL", "FRAG", "FLIP", "SKIP", "GRAB", "DLPH", "OCTO", "HEXA", "SLRP", "PIPE", "BOG", "SBOG",
    "SWMP", "SPKE", "GEEL", "LUCK", "BNBO", "WLTH", "COIN", "WOF", "PAIN", "ZIPP", "AGON", "BRRN",
    "CREP", "DLTA", "ETRN", "DOS", "MSTK", "SCCS", "SUN", "CUBE", "GLRM", "DFSH", "DSLR", "DSCK",
    "ICE", "ICBE", "BLSM", "CHTH", "HEAD", "POND", "HELI", "ENGN", "DEAD", "TRNC", "FUZZ", "SOUL",
    "BLLS",
];

const ALL_IMPLANTS: [&str; 49] = [
    "Nightmare Vision Goggles",
    "First Aid Kit",
    "Zoom N Go Bionic Eyes",
    "Zomy X-200 Portable Cassette Player",
    "Vertical Entry Device",
    "CSIJ Level IV Body Armor",
    "CSIJ Level III Body Armor",
    "Speed Enhancer Gland",
    "CSIJ Level II Body Armor",
    "Life Sensor",
    "Grappendix",
    "Speed Enhancer Total Organ Package",
    "Biothruster",
    "Load Bearing Vest",
    "Tactical Blast Shield",
    "Composite Helmet",
    "Icaros Machine",
    "Flechette Grenade",
    "Night Vision Goggles",
    "Stealth Suit",
    "Cursed Torch",
    "HE Grenade",
    "Cortical Scaledown+",
    "Hazmat Suit",
    "Tattered Rain Hat",
    "Holy Scope",
    "Abominator",
    "Eyes of Corporate Insight",
    "Military Camouflage",
    "Extravagant Suit",
    "ZZzzz Special Sedative Grenade",
    "Augmented arms",
    "Pneumatic Legs",
    "Alien Leg Wetware",
    "Ammunition Gland",
    "Angular Advantage Tactical Munitions",
    "Skullgun",
    "House",
    "CSIJ Level VI Golem Exosystem",
    "Goo Overdrive",
    "Gunkboosters",
    "CSIJ Level V Biosuit",
    "CSIJ Level IIB Body Armor",
    "Funkgrunters",
    "Microbial Oil Secretion Glands",
    "Biojet",
    "Speed Enhancer Node Cluster",
    "Bouncy Suit",
    "Flowerchute",
];

pub struct SaveFolder {
    pub folder_path: std::path::PathBuf,
    pub savegame: std::path::PathBuf,
    pub stocksave: std::path::PathBuf,
}

pub fn unlock_all_guns(save_json: &mut Value) {
    save_json["weapons_unlocked"] = json!(vec![true; 31]);
}

pub fn unlock_all_implants(save_json: &mut Value) {
    save_json["implants_unlocked"] = json!(ALL_IMPLANTS.to_vec());
}

pub fn unlock_all_levels(save_json: &mut Value) {
    save_json["levels_unlocked"] = json!(19);
}

pub fn set_money(amount: String, save_json: &mut Value) {
    save_json["money"] = json!(amount);
}

pub fn reset_perma_death_npcs(save_json: &mut Value) {
    save_json["dead_npcs"] = json!(vec![0; 0]);
}

pub fn toogle_death(save_json: &mut Value) {
    if let json!(true) = save_json["death"] {
        save_json["death"] = json!(false);
    } else {
        save_json["death"] = json!(true)
    }
}

pub fn edit_stocks_owned(stocks_json: &mut Value, ticker: String, amount: String) {
    if stocks_json[&ticker].is_null() {
        println!("{}", "Ticker not found, try again".red());
        exit(1);
    } else {
        stocks_json[&ticker]["owned"] = json!(amount);
    }
}

pub fn unlock_all_fish(stocks_json: &mut Value) {
    stocks_json["fish_found"] = json!(FISH_TICKERS.to_vec());
}

pub fn set_gamemode(save_json: &mut Value, v: u8) {
    match v {
        1 => {
            save_json["soul"] = json!(true);
            save_json["hell_discovered"] = json!(false);
            save_json["husk"] = json!(false);
            save_json["hope"] = json!(false);
        }
        2 => {
            save_json["soul"] = json!(false);
            save_json["hell_discovered"] = json!(true);
            save_json["husk"] = json!(false);
            save_json["hope"] = json!(false);
        }
        3 => {
            save_json["soul"] = json!(false);
            save_json["hell_discovered"] = json!(false);
            save_json["husk"] = json!(true);
            save_json["hope"] = json!(false);
        }
        4 => {
            save_json["soul"] = json!(false);
            save_json["hell_discovered"] = json!(false);
            save_json["husk"] = json!(false);
            save_json["hope"] = json!(true);
        }
        _ => {
            println!("{}", "Please choose 1-4".red());
            exit(1)
        }
    }
}

impl SaveFolder {
    pub fn new(folder: String) -> SaveFolder {
        #[cfg(target_os = "linux")]
        let folder: SaveFolder = {
            let mut folder_path: std::path::PathBuf;
            let mut savegame: Option<std::path::PathBuf> = None;
            let mut stocksave: Option<std::path::PathBuf> = None;
            let mut buff = folder;
            if std::path::PathBuf::from(buff.trim()).exists()
                & std::path::PathBuf::from(buff.trim()).is_dir()
            {
                folder_path = std::path::PathBuf::from(buff.trim());
                for entry in std::fs::read_dir(buff.trim()).unwrap() {
                    let entry = entry.unwrap();
                    if entry.path().to_string_lossy().contains("savegame") {
                        savegame = Some(entry.path());
                    }
                    if entry.path().to_string_lossy().contains("stocks") {
                        stocksave = Some(entry.path());
                    }
                }
                if savegame.is_none() {
                    println!("'savegame.save' not found, retry!!!");
                    exit(1);
                }
                if stocksave.is_none() {
                    println!("'stocks.save' not found, retry!!!");
                    exit(1);
                }
            } else {
                println!("Folder doesn't contain save files.");
                exit(1);
            }
            SaveFolder {
                folder_path,
                savegame: savegame.unwrap(),
                stocksave: stocksave.unwrap(),
            }
        };
        folder
    }
    pub fn write_to_files(&self, save_json: &mut Value, stocks_json: &mut Value) {
        let mut savegame = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .open(&self.savegame)
            .expect("Could not write to save file.");
        let mut stocksave = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .open(&self.stocksave)
            .expect("Could not write to stocks file.");
        savegame
            .write_all(save_json.to_string().as_bytes())
            .unwrap();
        stocksave
            .write_all(stocks_json.to_string().as_bytes())
            .unwrap();
    }
}
