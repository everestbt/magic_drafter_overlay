use directories::{BaseDirs};
use std::env::consts::OS;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use serde_json::Value;

pub struct ArenaId {
    pub id: i32
}

static PARENT_DIR_1: & str = "Wizards Of The Coast";
static PARENT_DIR_2: & str = "MTGA";
static LOG_FILE_NAME: & str = "Player.log";

pub fn get_default_path() -> PathBuf {
    if let Some(dirs) = BaseDirs::new() {
        let os_dir = match OS {
            "windows" => Path::new("AppData").join("LocalLow"), // This path is untested as not yet run on a windows machine
            "macos" => Path::new("Library").join("Logs"),
            _ => panic!("MTGA reader does not support any other OS than windows and mac")
        };
        dirs.home_dir().join(os_dir).join(PARENT_DIR_1).join(PARENT_DIR_2).join(LOG_FILE_NAME)
    }
    else {
        panic!("Not supported")
    }
}

pub fn check_for_latest_draft_cards(path: &PathBuf) -> Option<Vec<ArenaId>> {
    let line_read = search_file_for_latest_draft_line(path)
        .and_then(|l| l.split_once(' ').map(|s| s.1.to_string()))
        .and_then(|l| Value::from_str(l.as_str()).ok())
        .and_then(|l| l.get("PackCards").cloned())
        .and_then(|l| l.as_str().map(|s| s.to_string()))
        .map(|l| {
            let mut cards = vec![];
            for card in l.split(',') {
                let id = i32::from_str(card).expect("Card id not an integer");
                cards.push(ArenaId {id});
            }
            cards
        });
    line_read
}

fn search_file_for_latest_draft_line(path: &PathBuf) -> Option<String> {
    if let Ok(file) = File::open(path) {
        let lines = BufReader::new(file).lines();
        let mut draft_line = None;
        for l in lines.map_while(Result::ok) {
            if l.contains("Draft.Notify") {
                draft_line = Some(l.clone())
            }
        }
        draft_line
    }
    else {
        None
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_in_file_single_line() {
        let line = search_file_for_latest_draft_line(&PathBuf::from_str("./example.log").unwrap());
        assert!(line.is_some());
        assert!(line.unwrap().contains("\"SelfPick\":13"));
    }

    #[test]
    fn test_read_in_file_multiple_lines() {
        let line = search_file_for_latest_draft_line(&PathBuf::from_str("./example2.log").unwrap());
        assert!(line.is_some());
        assert!(line.unwrap().contains("\"SelfPick\":15"));
    }

    #[test]
    fn test_read_in_player_log() {
        let cards = check_for_latest_draft_cards(&PathBuf::from_str("./PlayerLogExample.log").unwrap());
        assert!(cards.is_some());
        assert_eq!(cards.unwrap().first().unwrap().id, 102708);
    }
}
