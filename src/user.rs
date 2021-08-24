#[derive(Debug)]
pub struct User;

pub type UserName = String;
pub type UserId = u32;

pub struct BonusUseLog;

impl User {
    pub fn bonus_use_history(&self) -> Vec<BonusUseLog> {
        vec![BonusUseLog{}, BonusUseLog{}]
    }
    pub fn get_user_id(&self) -> UserId {
        return 0
    }
}