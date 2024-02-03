use std::path::{Path, PathBuf};

use error::SteamAppLauncherError;
use valve_files::{
    appmanifest::{self, AppState},
    libraryfolders::{self, LibraryFolders},
};

pub mod error;
pub mod valve_files;

pub fn get_library(dirs: &[&str]) -> Result<LibraryFolders, SteamAppLauncherError> {
    let library_path = get_library_path(dirs)?;
    let lib_path_display = library_path.display().to_string();

    let library_text =
        &std::fs::read_to_string(library_path).map_err(SteamAppLauncherError::Read)?;
    let library_data = libraryfolders::from_str(library_text)
        .map_err(|_| SteamAppLauncherError::Parse(lib_path_display))?;

    Ok(library_data)
}

pub fn get_games(library: &LibraryFolders) -> Result<Vec<AppState>, SteamAppLauncherError> {
    let mut games = Vec::new();

    let appmanifests = get_games_path(library);

    for app_path in appmanifests {
        let file = std::fs::read_to_string(app_path)
            .map_err(|x| SteamAppLauncherError::Parse(x.to_string()))?;
        let manifest = appmanifest::from_str(&file)
            .map_err(|x| SteamAppLauncherError::Parse(x.to_string()))?;
        games.push(manifest);
    }

    Ok(games)
}

fn get_games_path(library: &LibraryFolders) -> Vec<PathBuf> {
    library
        .libraries
        .iter()
        .flat_map(|x| {
            x.apps
                .iter()
                .map(|y| (x.path.as_str(), y.0))
                .collect::<Vec<(&str, &u64)>>()
        })
        .map(|(lib_path, id)| {
            PathBuf::from(lib_path)
                .join("steamapps")
                .join(format!("appmanifest_{}.acf", id))
        })
        .filter(|x| x.is_file())
        .collect()
}

fn get_library_path(dirs: &[&str]) -> Result<PathBuf, SteamAppLauncherError> {
    let library_path = {
        let dirs = dirs
            .iter()
            .map(|x| Path::new(x).join("libraryfolders.vdf"))
            .filter(|x| x.is_absolute() && x.is_file())
            .take(1)
            .map(|x| x.to_path_buf())
            .collect::<Vec<PathBuf>>();

        if dirs.is_empty() {
            return Err(SteamAppLauncherError::LibraryNotFound);
        }

        dirs.first()
            .ok_or(SteamAppLauncherError::LibraryNotFound)?
            .to_owned()
    };
    Ok(library_path)
}
