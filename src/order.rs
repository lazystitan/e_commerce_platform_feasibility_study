use crate::coupon::Coupon;
use crate::shipping::ShippingMethod;
use rust_decimal::prelude::*;
use crate::user::User;
use crate::sku::{StockKeepingUnit, BoughtSkuSet};

/// A trait to allow other entity to change order's inner status which intend to.
pub trait ApplyToOrder {
    fn apply_to_order(&self, order: &mut Order);
}


/// Order, consist of some skus, each of it can have it's own number,
/// final result of calculation of how much should user pay, and other
/// info needs to calculation of final result.
#[derive(Debug)]
pub struct Order<'a> {
    pub user: User,

    pub items: BoughtSkuSet<'a>,

    pub coupon_bonus: Decimal,
    pub activity_bonus: Decimal,
    pub shipping_fee: Decimal,
    pub items_amount: Decimal,
    pub total_amount: Decimal,

    status_coupon: bool,
    status_activity: bool,
    status_shipping: bool,
    status_total: bool,
}

impl <'a> Order<'a> {
    pub fn new() -> Self {
        Self {
            user: User{},
            items: BoughtSkuSet::new(),

            coupon_bonus: dec!(0),
            activity_bonus: dec!(0),
            shipping_fee: dec!(0),
            items_amount: dec!(0),
            total_amount: dec!(0),

            status_coupon: false,
            status_activity: false,
            status_shipping: false,

            status_total: false,
        }
    }

    /// Should be the first step of calculation, gen the value of
    /// all skus of the order.
    pub fn process_items(&mut self) {
        self.items_amount = self.items.total_amount();
    }

    /// Should be the second step of calculation, since the activities
    /// may have an impact on the value of the skus. Gen the value of
    /// activity's bonus of the order.
    // pub fn process_activity(&mut self, activities: &Vec<Box<dyn ActivityBehavior>>) {
    //     for activity in activities {
    //         activity.apply_to_order(self);
    //     }
    //     self.status_activity = true;
    // }

    /// Should be the third step of calculation, since the coupons
    /// may have an impact on shipping fee. Gen the value of shipping
    /// fee of the order based on the shipping method and the address.
    pub fn process_shipping_fee(&mut self, shipping_method: &ShippingMethod) {
        // let _total_weight = self.items.iter().map(|sku| sku.sku.weight).sum::<u32>();
        shipping_method.apply_to_order(self);
        self.status_shipping = true;
    }

    /// Should be the fourth step of calculation. Gen the value of
    /// coupon bonus of the order.
    pub fn process_coupon(&mut self, coupons: &Vec<Coupon>) {
        for coupon in coupons {
            coupon.apply_to_order(self);
        }
        self.status_coupon = true;
    }

    /// Should be the last step of calculation. Gen the value of how
    /// much should user paying for.
    pub fn process_summary(&mut self) {
        self.total_amount =
            self.items_amount + self.shipping_fee - self.coupon_bonus - self.activity_bonus
    }
}
