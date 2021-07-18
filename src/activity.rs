use crate::order::{ApplyToOrder, Order, OrderItem};
use rust_decimal::prelude::*;
use std::cmp::Ordering;

pub trait ActivityFilter<'a> {
    fn skus(&self) -> &Vec<String>;
    fn filter_activity_order_items(&self, order: &'a Order) -> Vec<&'a OrderItem> {
        order.items.iter()
            .filter(|sku| {
                self.skus()
                    .iter()
                    .any(|activity_sku| activity_sku == &sku.sku.sku)
            })
            .collect()
    }
}

pub trait ActivityBehavior<'a>: ActivityFilter<'a> + ApplyToOrder {}

pub struct FullMinusOneActivity {
    pub full_number: u32,
    pub skus: Vec<String>,
}

impl <'a> ActivityFilter<'a> for FullMinusOneActivity {
    fn skus(&self) -> &Vec<String> {
        &self.skus
    }
}

impl ApplyToOrder for FullMinusOneActivity {
    fn apply_to_order(&self, order: &mut Order) {
        let mut in_activity_items = self.filter_activity_order_items(order);

        let in_activity_items_number = in_activity_items
            .iter()
            .map(|item| item.number)
            .sum::<u32>();

        if in_activity_items_number >= self.full_number {
            in_activity_items.sort_by(|pre_sku, next_sku| {
                pre_sku.sku.shop_price.cmp(&next_sku.sku.shop_price)
            });

            let mut available_items_number =
                (in_activity_items_number / self.full_number) as u32 * self.full_number;
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
}

impl <'a> ActivityBehavior<'a> for FullMinusOneActivity {}

pub struct FixedPriceActivity {
    pub fixed_number: u32,
    pub fixed_price: Decimal,
    pub skus: Vec<String>,
}

impl<'a> ActivityFilter<'a> for FixedPriceActivity {
    fn skus(&self) -> &Vec<String> {
        &self.skus
    }
}

impl ApplyToOrder for FixedPriceActivity {
    fn apply_to_order(&self, order: &mut Order) {
        let mut in_activity_items = self.filter_activity_order_items(order);

        let in_activity_items_number = in_activity_items
            .iter()
            .map(|item| item.number)
            .sum::<u32>();

        if in_activity_items_number >= self.fixed_number {
            let times = in_activity_items_number / self.fixed_number;
            in_activity_items.sort_by(|pre_sku, next_sku| {
                pre_sku.sku.shop_price.cmp(&next_sku.sku.shop_price)
            });

            let mut available_items_number =
                times * self.fixed_number;
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
            let total_price = self.fixed_price * Decimal::from(times);
            let activity_bonus = total_price - origin_price;
            if activity_bonus > Decimal::from(0) {
                order.activity_bonus += activity_bonus
            }
        }
    }
}

impl <'a> ActivityBehavior<'a> for FixedPriceActivity {}