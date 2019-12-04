use chrono::NaiveDateTime;

use super::schema::events;

#[derive(Serialize, Deserialize, Debug, Queryable, Identifiable)]
pub struct Event {
    pub id: i32,
    pub uid: String,
    pub content: String,
    pub finished: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "events"]
pub struct NewEvent {
    pub uid: String,
    pub content: String,
    pub finished: bool,
    #[serde(default = "super::naivedate_now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "super::naivedate_now")]
    pub updated_at: NaiveDateTime,
}
