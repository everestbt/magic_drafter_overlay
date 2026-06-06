use std::path::{PathBuf};
use rusqlite::{Connection, Result};

static DATABASE_NAME: & str = "AllPrintings.sqlite";

#[derive(Clone, Debug)]
pub struct Card {
    pub arena_id: i32,
    pub name: String,
}

#[derive(Clone, Debug)]
struct IdentifierShim {
    uuid: String,
}

#[derive(Clone, Debug)]
struct CardShim {
    name: String,
}

pub fn get_card_names(path: &PathBuf, card_ids: &[i32]) -> Result<Vec<Card>> {
    let conn = get_connection(path);
    let mut cards = vec![];
    let mut err = None;
    for id in card_ids {
        let mut stmt_identifier = conn.prepare("SELECT uuid FROM cardIdentifiers WHERE mtgArenaId = ?1 LIMIT 1")?;
        let mut iter_identifier = stmt_identifier.query_map([id.to_string()], |row| {
            Ok( IdentifierShim {
                uuid: row.get(0)?,
            })
        })?;
        if let Some(row_id) = iter_identifier.next() {
            if let Ok(card_id) = row_id {
                let mut stmt_cards = conn.prepare("SELECT name FROM cards WHERE uuid = ?1 LIMIT 1")?;
                let mut iter_card = stmt_cards.query_map([card_id.uuid], |row| {
                    Ok( CardShim {
                        name: row.get(0)?,
                    })
                })?;
                if let Some(row_card) = iter_card.next() {
                    if let Ok(card) = row_card {
                        cards.push(Card { arena_id: id.clone(), name: card.name });
                    }
                    else {
                        err = Some(row_card.unwrap_err());
                        break 
                    }
                }
            }
            else {
                err = Some(row_id.unwrap_err());
                break 
            }
        }
    }
    if let Some(error) = err {
        Err(error)
    }
    else {
        Ok(cards)
    }
}

fn get_connection(path: &PathBuf) -> Connection {
    let path = path.join(DATABASE_NAME);
    let conn: Connection = Connection::open(path).expect("Failed to open a connection");
    conn
}