#[derive(Debug)]
pub struct Transaction {
    pub id: i32,
    pub description: String,
    pub amount: f64,
    pub created_at: String,
}
