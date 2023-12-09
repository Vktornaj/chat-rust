use uuid::Uuid;


pub struct TokenMetadata {
    pub token_id: Uuid,
    pub creation_timestamp: i64,
    pub last_use_timestamp: i64,
    pub is_active: bool,
    pub browser: String,
    pub os: String,
}