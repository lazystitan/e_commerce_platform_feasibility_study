use rust_decimal::prelude::*;
use crate::order::{ApplyToOrder, Order};

pub enum Coupon {
    // ShippingFeeCoupon(Decimal),
    ProductCoupon(Decimal),
    // GeneralCoupon(Decimal)
}

impl ApplyToOrder for Coupon {
    fn apply_to_order(&self, order: &mut Order) {
        match self {
            // Coupon::ShippingFeeCoupon(bonus) => {
            //     order.coupon_bonus += bonus
            // }
            Coupon::ProductCoupon(bonus) => order.coupon_bonus += bonus,
        }
    }
}