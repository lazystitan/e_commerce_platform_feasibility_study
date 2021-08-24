use limit::*;

use crate::order::{ApplyToOrder, Order};
use crate::bonus::limit::order_unrelated_limits::CanApplyByOrderUnrelatedInfo;

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

struct Bonus<T>
where T: CanApplyByOrderUnrelatedInfo
{
    country: Country,
    domain: Domain,
    apply_object_info: ApplyObjectLimit,
    order_unrelated_limits: T,
    origin: BonusOrigin,
}

struct BonusApplyError;

impl <T:CanApplyByOrderUnrelatedInfo> ApplyToOrder for Bonus<T> {
    fn apply_to_order(&self, order: &mut Order) {
        // self.order_unrelated_limits.can_apply();

        // if !self.apply_object_info.is_meet_condition(&order.items) {
        //     eprintln!("not meet apply condition");
        //     return;
        // }

        // let bonus = self.apply_object_info.charge(order);


    }
}