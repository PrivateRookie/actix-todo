use chrono::NaiveDateTime;

use super::schema::events;

#[derive(Serialize, Debug, Queryable)]
pub struct Event {
    pub id: String,
    pub content: String,
    pub finished: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "events"]
pub struct NewEvent {
    pub id: String,
    pub content: String,
    pub finished: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Deserialize)]
#[table_name = "events"]
pub struct UpdateEventStatus {
    pub id: i32,
    pub finished: bool,
    #[serde(default = "super::naivedate_now")]
    pub updated_at: NaiveDateTime,
}
