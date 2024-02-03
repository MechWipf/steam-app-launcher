use std::{
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

use color_eyre::{eyre::eyre, Report};
use itertools::Itertools;
use steam_app_launcher::{get_games, get_library, valve_files::appmanifest::AppState};

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let args: Vec<String> = std::env::args().collect();

    let home_folder = std::env::var("HOME")?;

    let library = get_library(&[
        &format!("{home_folder}/.local/share/Steam/steamapps"),
        &format!("{home_folder}/.steam/steam/steamapps"),
    ])?;
    let games = get_games(&library)?;

    match args.len() {
        1 => {
            print_all_games(games.as_slice())?;
        }
        2 => {
            run_game(args.get(1).unwrap(), games.as_slice())?;
        }
        _ => return Err(eyre!("too many arguments")),
    }

    Ok(())
}

fn run_game(game_and_id: &str, games: &[AppState]) -> Result<(), Report> {
    if let Some(game) = games
        .iter()
        .find(|x| format!("{} ({})", x.name, x.app_id).eq(game_and_id))
    {
        Command::new("steam")
            .arg(format!("steam://rungameid/{}", game.app_id))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .exec();
    }

    Ok(())
}

fn print_all_games(games: &[AppState]) -> Result<(), Report> {
    let pretty_print: Vec<(&str, &u64)> = games
        .iter()
        .map(|x| (x.name.as_str(), &x.app_id))
        .sorted_by(|a, b| Ord::cmp(a.0, b.0))
        .collect();

    for g in pretty_print {
        println!("{} ({})", g.0, g.1)
    }

    Ok(())
}
