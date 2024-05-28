mod r#mod;
use crate::r#mod::*;
use clap::{Parser, Subcommand};
use colored::Colorize;
use serde_json::*;
use std::{io::Read, process::exit};

const PATH: &str = "/Roaming/Gobot/app_userdata/Cruelty Squad";

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Edits save files for you to unlock everything", name = "Ez Cruelty Unlocker",long_about = None)]
struct Args {
    /// Path of the folder where save file are located (PLEASE ENTER IT IN QUOTES)
    #[arg(long, value_name = "FOLDER PATH")]
    folder: Option<String>,
    ///Unlocks all guns
    #[arg(short = 'g')]
    guns_unlock: bool,
    ///Unlock all implants
    #[arg(short = 'i')]
    implants_unlock: bool,
    ///Unlock all levels
    #[arg(short = 'l')]
    level_unlock: bool,
    ///Set money
    #[arg(short = 'm', long, value_name = "MONEY AMOUNT")]
    set_money: Option<String>,
    ///Revive perma dead NPCS (like Fish Fred)
    #[arg(short = 'r')]
    revive_npcs: bool,
    ///Unlock all fish (you can view them at the fish tank in HQ)
    #[arg(short = 'f')]
    fish_unlock: bool,
    ///Toggles death mode (can toggle back to life mode)
    #[arg(short = 'd')]
    death_toggle: bool,
    ///Set the amount owned of a certain stock (PLEASE ENTER IT LIKE THIS 'APPL:100000')
    #[arg(short = 's',long, value_names = &["TICKER:AMOUNT"])]
    stock_owned: Option<String>,
    ///Set the game mode, use numers: 1 -> Divine Light, 2 -> Flesh Automation, 3 -> Power In Misery, 4 -> Hope Eradicated
    #[arg(short = 'o',long, value_names = &["TICKER:AMOUNT"])]
    gamemode: Option<u8>,
}

fn main() {
    let args = Args::parse();
    //panic!();
    if args.folder.is_none() {
        println!("{}", "Folder value empty, retry!!!".red());
        exit(1);
    }
    let mut folder = SaveFolder::new(args.folder.unwrap());
    let mut savegame_buff = String::new();
    let mut stocks_buff = String::new();
    std::fs::File::open(&folder.savegame)
        .expect("Could not open safe file")
        .read_to_string(&mut savegame_buff)
        .unwrap();
    std::fs::File::open(&folder.stocksave)
        .expect("Could not open safe file")
        .read_to_string(&mut stocks_buff)
        .unwrap();
    let mut save_json: Value =
        from_str(&mut savegame_buff).expect("Could not parse savegame.save, possibly corrupted");
    let mut stocks_json: Value =
        from_str(&mut stocks_buff).expect("Could not parse stocks.save, possibly corrupted");

    if args.guns_unlock {
        unlock_all_guns(&mut save_json);
    };
    if args.implants_unlock {
        unlock_all_implants(&mut save_json);
    }
    if args.revive_npcs {
        reset_perma_death_npcs(&mut save_json);
    };
    if args.death_toggle {
        toogle_death(&mut save_json);
    };
    if args.fish_unlock {
        unlock_all_fish(&mut stocks_json);
    };
    if args.level_unlock {
        unlock_all_levels(&mut save_json)
    };
    if args.set_money.is_some() {
        set_money(args.set_money.unwrap(), &mut save_json)
    }
    if args.gamemode.is_some() {
        set_gamemode(&mut save_json, args.gamemode.unwrap())
    }
    if args.stock_owned.is_some() {
        let arg = args.stock_owned.unwrap();
        let split = arg
            .split_once(':')
            .unwrap_or_else(|| panic!("{}", "Wrong format".red()));
        edit_stocks_owned(
            &mut stocks_json,
            split.0.to_string(),
            split.1.to_string(),
            )
    }
    folder.write_to_files(&mut save_json, &mut stocks_json);
    println!("{}", "Success".green())
}
