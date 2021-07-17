use crate::order::{ApplyToOrder, Order, OrderItem};
use rust_decimal::prelude::*;
use std::cmp::Ordering;

pub enum Activity {
    FullMinusOne {
        full_number: u32,
        skus: Vec<String>,
    },
    FixedPrice {
        fixed_number: u32,
        fixed_price: Decimal,
        skus: Vec<String>,
    },
}

impl Activity {
    fn skus(&self) -> &Vec<String> {
        match self {
            Activity::FullMinusOne { skus, .. } => skus,
            Activity::FixedPrice { skus, .. } => skus,
        }
    }
}

trait FilterActivityOrderItems<'a> {
    fn filter_activity_order_items(&self, activity: &Activity) -> Self;
}

impl<'a> FilterActivityOrderItems<'a> for Vec<&'a OrderItem> {
    fn filter_activity_order_items(&self, activity: &Activity) -> Self {
        self.iter()
            .filter(|sku| {
                activity
                    .skus()
                    .iter()
                    .any(|activity_sku| activity_sku == &sku.sku.sku)
            })
            .map(|item| *item)
            .collect()
    }
}

impl ApplyToOrder for Activity {
    fn apply_to_order(&self, order: &mut Order) {
        match self {
            Activity::FullMinusOne {
                full_number,
                skus: _,
            } => {
                let mut in_activity_items = order
                    .items
                    .iter()
                    .collect::<Vec<&OrderItem>>()
                    .filter_activity_order_items(self);

                let in_activity_items_number = in_activity_items
                    .iter()
                    .map(|item| item.number)
                    .sum::<u32>();

                if in_activity_items_number >= *full_number {
                    in_activity_items.sort_by(|pre_sku, next_sku| {
                        if pre_sku.sku.shop_price < next_sku.sku.shop_price {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    });

                    let mut available_items_number =
                        (in_activity_items_number / full_number) as u32 * full_number;
                    let mut activity_bonus = dec!(0);
                    for in_activity_item in in_activity_items {
                        if available_items_number >= in_activity_item.number {
                            activity_bonus += Decimal::from(in_activity_item.number)
                                * in_activity_item.sku.shop_price;
                            available_items_number -= in_activity_item.number;
                        } else {
                            activity_bonus += Decimal::from(available_items_number)
                                * in_activity_item.sku.shop_price;
                            break;
                        }
                    }

                    order.activity_bonus += activity_bonus;
                }
            }
            Activity::FixedPrice {
                fixed_number,
                fixed_price,
                skus: _,
            } => {
                let mut in_activity_items = order
                    .items
                    .iter()
                    .collect::<Vec<&OrderItem>>()
                    .filter_activity_order_items(self);

                let in_activity_items_number = in_activity_items
                    .iter()
                    .map(|item| item.number)
                    .sum::<u32>();

                if in_activity_items_number >= *fixed_number {
                    let times = in_activity_items.len() as u32 / fixed_number;
                    in_activity_items.sort_by(|pre_sku, next_sku| {
                        if pre_sku.sku.shop_price < next_sku.sku.shop_price {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    });

                    let mut available_items_number =
                        (in_activity_items_number / fixed_number) as u32 * fixed_number;
                    let mut origin_price = dec!(0);
                    for in_activity_item in in_activity_items {
                        if available_items_number >= in_activity_item.number {
                            origin_price += Decimal::from(in_activity_item.number)
                                * in_activity_item.sku.shop_price;
                            available_items_number -= in_activity_item.number;
                        } else {
                            origin_price += Decimal::from(available_items_number)
                                * in_activity_item.sku.shop_price;
                            break;
                        }
                    }
                    let total_price = fixed_price * Decimal::from(times);
                    let activity_bonus = total_price - origin_price;
                    if activity_bonus > Decimal::from(0) {
                        order.activity_bonus += activity_bonus
                    }
                }
            }
        }
    }
}
