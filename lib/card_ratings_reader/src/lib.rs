use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;
use std::str::FromStr;
use csv::ReaderBuilder;
use rust_decimal::Decimal;

pub struct CardRatings{
    pub card_name: String,
    pub rating: Option<Decimal>
}

pub fn get_ratings(path: &PathBuf, cards: &[String], sort_val: &str) -> Result<Vec<CardRatings>, Error> {
    let rating_map = load_ratings_map(path, sort_val)?;
    let mut ratings = Vec::with_capacity(cards.len());
    for card in cards {
        if let Some(rating) = rating_map.get(card) { // Most will meet this condition
            ratings.push(CardRatings { card_name: card.clone(), rating: Some(rating.clone()) });
        }
        else {
            if let Some(partial_match) =  rating_map.keys().find(|s| card_name_match(&s, &card)) {
                ratings.push(CardRatings { card_name: card.clone(), rating: Some(rating_map.get(partial_match).unwrap().clone()) });
            }
            else {
                ratings.push(CardRatings { card_name: card.clone(), rating: None});
            }
        }
    }
    Ok(ratings)
}

fn card_name_match(rating_name: &str, card_name: &str) -> bool {
    // First lowercase them
    let rating_name_lower = rating_name.to_lowercase();
    let card_name_lower = card_name.to_lowercase();
    // Check if the card_name contains the rating name and the vice-versa
    card_name_lower.contains(&rating_name_lower) || rating_name_lower.contains(&card_name_lower)
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

fn load_ratings_map(path: &PathBuf, sort_val: &str) -> Result<HashMap<String, Decimal>, Error> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;
    // Find the column for the rating
    let headers = rdr.headers().expect("Rating file must have headers");
    let index = headers.iter().position(|s| s.eq(sort_val)).expect("Must provide a sort_val that the file contains");
    let mut rating_map = HashMap::new();
    for result in rdr.records() {
        let record = result?;
        let name = record.get(0).expect("Card name must be in the first column");
        if let Some(rating) = record.get(index).and_then(|r| map_rating(r)) {
            rating_map.insert(name.to_string(), rating);
        }
    }
    Ok(rating_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_name_match_double_sided() {
        assert!(card_name_match("Fable of the Mirror-Breaker", "Fable of the Mirror-Breaker // Reflection of Kiki-Jiki"));
    }
}