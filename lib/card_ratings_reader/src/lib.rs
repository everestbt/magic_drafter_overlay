use std::io::Error;
use std::path::PathBuf;
use std::str::FromStr;
use csv::ReaderBuilder;
use rust_decimal::Decimal;

pub struct CardRatings{
    pub card_name: String,
    pub rating: Option<Decimal>
}

pub fn get_ratings(path: PathBuf, cards: &[String], sort_val: &str) -> Result<Vec<CardRatings>, Error> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;

    // Find the column for the rating
    let headers = rdr.headers()?;
    let index = headers.iter().position(|s| s.eq(sort_val)).expect("Must provide a sort_val that the file contains");
    let mut ratings = vec![];
    for result in rdr.records() {
        let record = result?;
        let name = record.get(0).expect("Card name must be in the first column");
        if cards.contains(&name.to_string()) {
            let rating = record.get(index).and_then(|r| map_rating(r));
            ratings.push(CardRatings { card_name: name.to_string(), rating });
        }
    }
    Ok(ratings)
}

fn map_rating(rating: &str) -> Option<Decimal> {
    // Trim common string
    let s = rating.replace(&['%', 'p'][..], "");
    if s.is_empty() {
        None
    }
    else {
        Some(Decimal::from_str(s.as_str()).expect("Should only be numbers"))
    }
}