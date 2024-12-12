use uuid::Uuid;

pub type Balance = u32;
pub type AccountId = Uuid;

pub struct Account {
    pub id: AccountId,
    pub description: String,
    pub balance: Balance,
    pub points: Balance,
}
