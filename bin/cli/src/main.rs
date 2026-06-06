use std::{cmp::Reverse, path::PathBuf, str::FromStr};
use clap::Parser;
use player_log_reader::{get_default_path, check_for_latest_draft_cards};
use card_database_reader::get_card_names;
use card_ratings_reader::get_ratings;

// Command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory that the card database and card ratings is held in
    #[arg(long)]
    directory: String,

    /// Ratings filename to use 
    #[arg(long)]
    ratings: String,

    /// Value to use for sorting
    #[arg(long)]
    sort: String,
    
    /// Path to the Player log, if not set will use the standard location
    #[arg(long)]
    player_log: Option<String>,
}

pub fn main() {
    let args = Args::parse();

    let directory = PathBuf::from_str(&args.directory).expect("Directory must be a valid path");
    let player_log_path = if let Some(path) = args.player_log {
        PathBuf::from_str(&path).expect("Player log path must be valid")
    }
    else {
        get_default_path()
    };

    let latest = check_for_latest_draft_cards(player_log_path);
    if let Some(cards) = latest {
        let ids: Vec<i32> = cards.iter().map(|c| c.id).collect();
        let names: Vec<String> = get_card_names(&directory, &ids).expect("Should find all cards")
            .iter()
            .map(|c| c.name.clone())
            .collect();

        let ratings_path = directory.join(args.ratings);
        let mut ratings = get_ratings(ratings_path, &names, &args.sort).expect("Failed to read ratings");
        ratings.sort_by_key(|r| Reverse(r.rating));
        for rating in ratings {
            println!("{} : {}", 
                rating.card_name, 
                rating.rating.map(|d| d.to_string()).unwrap_or("No rating".to_string()))
        }
    }
}