use std::cmp::Ordering;
use std::rc::Rc;

use rust_decimal::prelude::*;

use crate::attribute::Attribute;
use crate::order::Order;
use crate::sku::{BoughtSkuSet, Sku, SkuName};
use crate::user::{User, UserName};

pub enum ApplyObjectLimit {
    Product,
    ShippingFee,
}

/// Condition of apply
pub enum ApplyConditionLimit {
    None,
    Amount(Decimal),
    Number(u32),
    AmountAndNumber(Decimal, u32),
    AmountOrNumber(Decimal, u32),
}

pub trait CanApplyByProducts {
    fn can_apply(&self, order_items: &BoughtSkuSet) -> bool;
}

/// Condition of apply
pub enum ProductSetLimit {
    None,
    SKUSet(Vec<SkuName>),
    AttributeSet(Vec<Attribute>),
    SKUAndAttributeSet(Vec<SkuName>, Vec<Attribute>),
    SKUOrAttributeSet(Vec<SkuName>, Vec<Attribute>),
}

pub trait ProductSetFilter {
    fn filter<'a>(&'a self, order_items: &'a BoughtSkuSet) -> BoughtSkuSet;
}

/// Bonus form
pub enum BonusFormLimit {
    Percent(Decimal),
    Amount(Decimal),
    FixedPrice(Decimal),
}

/// Only for bonus which applied to product, including the info of
/// which ones in products that already meet the product set
/// limit can apply bonus.
pub enum ApplyRangeLimit {
    None,
    Number(u32),
}

pub trait ApplyRangeFilter {
    fn filter<'a>(&self, order_items: &'a BoughtSkuSet) -> Vec<(&'a Sku)>;
}

pub enum SuperpositionLimit {
    Times(u32),
    None,
}

pub trait CalSuperposition {
    fn apply_times<'a>(&self, items: &BoughtSkuSet<'a>, config: &ApplyConditionLimit) -> u32;
}

pub mod order_unrelated_limits {
    use crate::user::{User, UserId, UserName};

    pub struct OrderUnrelatedLimits<T, U, V>
        where T: CanApplyByTime,
              U: CanApplyByUseTime,
              V: CanApplyByUser
    {
        time_limit: T,
        use_time_limit: U,
        user_related_limit: V,
    }

    pub trait CanApplyByOrderUnrelatedInfo {
        fn can_apply(&self, user: &User) -> bool;
    }

    pub trait CanApplyByTime {
        fn include(&self, time: chrono::DateTime<chrono::Utc>) -> bool;
    }

    pub trait CanApplyByUseTime {
        fn is_exceed_max(&self, user: &User) -> bool;
    }

    pub trait CanApplyByUser {
        fn check_user(&self, user: &User) -> bool;
    }


    pub enum TimeLimit {
        None,
        Start(chrono::DateTime<chrono::Utc>),
        StartDuration {
            start: chrono::DateTime<chrono::Utc>,
            duration: chrono::Duration,
        },
    }

    impl CanApplyByTime for TimeLimit {
        fn include(&self, time: chrono::DateTime<chrono::Utc>) -> bool {
            match self {
                TimeLimit::None => true,
                TimeLimit::Start(start) => *start < time,
                TimeLimit::StartDuration { start, duration } => {
                    *start < time && (*start + *duration) > time
                }
            }
        }
    }

    pub enum UseTimeLimit {
        None,
        Number(u32),
    }

    impl CanApplyByUseTime for UseTimeLimit {
        fn is_exceed_max(&self, user: &User) -> bool {
            match self {
                UseTimeLimit::None => false,
                UseTimeLimit::Number(limit) => user.bonus_use_history().len() > *limit as usize,
            }
        }
    }


    pub enum UserRelatedLimit {
        No,
        Yes(UserId),
    }

    impl CanApplyByUser for UserRelatedLimit {
        fn check_user(&self, user: &User) -> bool {
            match self {
                UserRelatedLimit::No => {
                    true
                }
                UserRelatedLimit::Yes(id) => {
                    *id == user.get_user_id()
                }
            }
        }
    }


    pub enum OptionalLimit {
        Optional,
        TryCompulsory,
        Compulsory,
    }

    pub enum VisibilityLimit {
        No,
        Yes,
    }
}