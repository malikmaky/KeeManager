use serde::{Deserialize, Serialize};

//
// Serializing and deserializing
// the entry structs to be able
// to send them to the front end
//

#[derive(Serialize, Deserialize)]
pub struct LoginEntry {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreditCardEntry {
    pub id: String,
    pub title: String,
    pub card_number: String,
    pub expiry_date: String,
    pub cardholder_name: String,
    pub cvv: String,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize)]
pub struct NoteEntry {
    pub id: String,
    pub title: String,
    pub content: String,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize)]
pub struct IdentityEntry {
    pub id: String,
    pub title: String,
    pub full_name: String,
    pub date_of_birth: String,
    pub nationality: String,
    pub identification_number: String,
    pub issue_date: String,
    pub expiry_date: String,
    pub issuer: String,
    pub notes: String,
    pub last_updated: String,
}
