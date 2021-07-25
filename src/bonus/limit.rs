use crate::attribute::Attribute;
use crate::order::Order;
use crate::sku::{BoughtSkuSet, Sku, SkuName};
use crate::user::{User, UserName};
use rust_decimal::Decimal;
use std::rc::Rc;
use std::cmp::Ordering;

/// Condition need to meet to apply
pub enum ApplyConditionLimit {
    None,
    Amount(Decimal),
    Number(u32),
    AmountAndNumber(Decimal, u32),
}

pub enum ProductSetLimit {
    None,
    SKUSet(Vec<SkuName>),
    AttributeSet(Vec<Attribute>),
    SKUAndAttributeSet {
        sku_set: Vec<SkuName>,
        attribute_set: Vec<Attribute>,
    },
}

impl ProductSetLimit {
    pub fn filter<'a, 'b>(&'a self, order_items: &'a BoughtSkuSet) -> Vec<(&'a Sku, u32)> {
        match self {
            ProductSetLimit::None
            | ProductSetLimit::AttributeSet(..)
            | ProductSetLimit::SKUAndAttributeSet { .. } => {
                //TODO not consider now
                order_items.filter(|_| true)
            }
            ProductSetLimit::SKUSet(skus) => {
                order_items.intersect_by_sku_name(skus.iter().map(|name| name.as_str()).collect())
            }
        }
    }
}

pub enum BonusFormLimit {
    Percent(Decimal),
    Amount(Decimal),
}

/// Only for bonus applied to product, how many product in
/// products which already meet the product set limit can
/// apply bonus.
pub enum ApplyRangeLimit {
    None,
    Number(u32),
}

pub struct ProductApplyObjectConfig {
    apply_condition_limit: ApplyConditionLimit,
    product_set_limit: ProductSetLimit,
    bonus_form_limit: BonusFormLimit,
    apply_range_limit: ApplyRangeLimit,
}

pub struct ShippingFeeApplyObjectConfig {
    apply_condition_limit: ApplyConditionLimit,
    product_set_limit: ProductSetLimit,
    bonus_form_limit: BonusFormLimit,
}

pub enum ApplyObjectLimit {
    Product(ProductApplyObjectConfig),
    ShippingFee(ShippingFeeApplyObjectConfig),
    ProductAndShippingFee {
        product: ProductApplyObjectConfig,
        shipping_fee: ShippingFeeApplyObjectConfig,
    },
}

impl ApplyObjectLimit {
    pub fn is_meet_condition(&self, order_items: &BoughtSkuSet) -> bool {
        let apply_condition;
        let filtered_skus;

        match self {
            ApplyObjectLimit::Product(c) => {
                filtered_skus = c.product_set_limit.filter(order_items);
                apply_condition = &c.apply_condition_limit;
            }
            ApplyObjectLimit::ShippingFee(c) => {
                filtered_skus = c.product_set_limit.filter(order_items);
                apply_condition = &c.apply_condition_limit;
            }
            ApplyObjectLimit::ProductAndShippingFee { .. } => {
                //TODO not consider now
                return true;
            }
        }

        if filtered_skus.len() == 0 {
            return false;
        }

        return match apply_condition {
            ApplyConditionLimit::None => true,
            ApplyConditionLimit::Amount(limit_amount) => {
                let amount: Decimal = filtered_skus
                    .iter()
                    .map(|&(sku, num)| Decimal::from(num) * sku.price())
                    .sum();

                if amount > *limit_amount {
                    true
                } else {
                    false
                }
            }
            ApplyConditionLimit::Number(limit_number) => {
                let number: u32 = filtered_skus.iter().map(|&(_sku, num)| num).sum();

                if number > *limit_number {
                    true
                } else {
                    false
                }
            }
            ApplyConditionLimit::AmountAndNumber(..) => {
                //TODO not consider now
                true
            }
        };
    }

    pub fn apply(&self, order: &mut Order) {
        match self {
            ApplyObjectLimit::Product(c) => {
                match c.bonus_form_limit {
                    BonusFormLimit::Percent(r) => {
                        match c.apply_range_limit {
                            ApplyRangeLimit::None => {
                                order.activity_bonus += order.items_amount * r
                            }
                            ApplyRangeLimit::Number(mut n) => {
                                let mut filtered_skus = c.product_set_limit.filter(&order.items);
                                filtered_skus.sort_by(|&(s1, _), &(s2, _)| {
                                    return s1.price().cmp(&s2.price())
                                });

                                let mut amount = dec!(0);
                                for (sku, num) in filtered_skus {
                                    if num < n {
                                        amount += sku.price() * Decimal::from(num);
                                    } else {
                                        amount += sku.price() * Decimal::from(n);
                                    }
                                    n -= num;
                                }
                                order.activity_bonus += amount * r
                            }
                        }

                    }
                    BonusFormLimit::Amount(v) => {
                        match c.apply_range_limit {
                            ApplyRangeLimit::None | ApplyRangeLimit::Number(_) => {
                                order.activity_bonus += v
                            }
                        }
                    }
                }
            }
            ApplyObjectLimit::ShippingFee(c) => {
                match c.bonus_form_limit {
                    BonusFormLimit::Percent(r) => {
                        order.activity_bonus += order.shipping_fee * r;
                    }
                    BonusFormLimit::Amount(v) => {
                        order.activity_bonus += v;
                    }
                }
            }
            ApplyObjectLimit::ProductAndShippingFee { .. } => {
                //TODO not consider now
                return;
            }
        }
    }
}

pub enum TimeLimit {
    None,
    Start(chrono::DateTime<chrono::Utc>),
    StartDuration {
        start: chrono::DateTime<chrono::Utc>,
        duration: chrono::Duration,
    },
}

impl TimeLimit {
    pub fn include(&self, time: chrono::DateTime<chrono::Utc>) -> bool {
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

impl UseTimeLimit {
    pub fn is_exceed_max(&self, user: &User) -> bool {
        match self {
            UseTimeLimit::None => false,
            UseTimeLimit::Number(limit) => user.bonus_use_history().len() > *limit as usize,
        }
    }
}

pub enum OptionalLimit {
    No,
    Yes,
}

pub enum VisibilityLimit {
    No,
    Yes,
}

pub enum UserRelatedLimit {
    No,
    Yes(UserName),
}
