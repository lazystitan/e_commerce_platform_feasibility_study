use limit::*;

use crate::order::{ApplyToOrder, Order};

mod limit;

struct Country(String);

struct Domain(String);

enum BonusOrigin {
    PointsSystem,
    ReturnCompensation,
    OutOfStockCompensation,
    CustomerServiceStaff,
    MarketingOperationStaff,
}

struct Bonus {
    country: Country,
    domain: Domain,
    apply_object_info: ApplyObjectLimit,
    time: TimeLimit,
    use_time: UseTimeLimit,
    optional: OptionalLimit,
    visibility: VisibilityLimit,
    user_related: UserRelatedLimit,
    origin: BonusOrigin,
}

struct BonusApplyError;

impl Bonus {}

impl ApplyToOrder for Bonus {
    fn apply_to_order(&self, order: &mut Order) {
        let now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();

        if !self.time.include(now) {
            eprintln!("expired");
            return;
        }

        if self.use_time.is_exceed_max(&order.user) {
            eprintln!("exceed max use time");
            return;
        }

        if !self.apply_object_info.is_meet_condition(&order.items) {
            eprintln!("not meet apply condition");
            return;
        }

        let bonus = self.apply_object_info.charge(order);


    }
}