use std::{cmp::Reverse, path::PathBuf, str::FromStr, thread::sleep, time::Duration};
use clap::Parser;
use player_log_reader::{get_default_path, check_for_latest_draft_cards};
use card_database_reader::get_card_names;
use card_ratings_reader::{get_ratings, CardRatings};

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

    /// Whether to poll or only run once
    #[arg(long)]
    poll: bool,
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

    if args.poll {
        let mut latest_ratings = vec![];
        loop {
            let ratings = check_for_ratings(&player_log_path, &directory, &args.ratings, &args.sort);
            if &ratings.len() != &latest_ratings.len() { // Use the fact that the number of cards will change when there is a new draft 
                for rating in &ratings {
                    println!("{} : {}", 
                        rating.card_name, 
                        rating.rating.map(|d| d.to_string()).unwrap_or("No rating".to_string()))
                }
                latest_ratings = ratings;
            }
            sleep(Duration::from_secs(1));
        }
    }
    else {
        let ratings = check_for_ratings(&player_log_path, &directory, &args.ratings, &args.sort);
        for rating in ratings {
            println!("{} : {}", 
                rating.card_name, 
                rating.rating.map(|d| d.to_string()).unwrap_or("No rating".to_string()))
        }
    }
}

fn check_for_ratings(player_log_path : &PathBuf, directory: &PathBuf, ratings_file: &str, sort_name: &str) -> Vec<CardRatings> {
    let latest = check_for_latest_draft_cards(player_log_path);
    if let Some(cards) = latest {
        let ids: Vec<i32> = cards.iter().map(|c| c.id).collect();
        let names: Vec<String> = get_card_names(&directory, &ids).expect("Should find all cards")
            .iter()
            .map(|c| c.name.clone())
            .collect();

        let ratings_path = directory.join(ratings_file);
        let mut ratings = get_ratings(ratings_path, &names, sort_name).expect("Failed to read ratings");
        ratings.sort_by_key(|r| Reverse(r.rating));
        ratings
    }
    else {
        vec![]
    }
}