#[derive(Debug)]
pub struct User;

pub type UserName = String;

pub struct BonusUseLog;

impl User {
    pub fn bonus_use_history(&self) -> Vec<BonusUseLog> {
        vec![BonusUseLog{}, BonusUseLog{}]
    }
}