use std::cmp::Ordering;
use std::rc::Rc;

use rust_decimal::prelude::*;

use crate::attribute::Attribute;
use crate::order::Order;
use crate::sku::{BoughtSkuSet, Sku, SkuName};
use crate::user::{User, UserName};

/// Condition of apply
pub enum ApplyConditionLimit {
    None,
    Amount(Decimal),
    Number(u32),
    AmountAndNumber(Decimal, u32),
}

/// Condition of apply
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

/// Bonus form
pub enum BonusFormLimit {
    Percent(Decimal),
    Amount(Decimal),
}

/// Only for bonus which applied to product, including the info of
/// which ones in products that already meet the product set
/// limit can apply bonus.
pub enum ApplyRangeLimit {
    None,
    Number(u32),
}

pub struct ProductApplyObjectConfig {
    apply_condition_limit: ApplyConditionLimit,
    product_set_limit: ProductSetLimit,
    bonus_form_limit: BonusFormLimit,
    apply_range_limit: ApplyRangeLimit,
    superposition: SuperpositionLimit,
}

pub struct ShippingFeeApplyObjectConfig {
    apply_condition_limit: ApplyConditionLimit,
    product_set_limit: ProductSetLimit,
    bonus_form_limit: BonusFormLimit,
    superposition: SuperpositionLimit,
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
                amount > *limit_amount
            }
            ApplyConditionLimit::Number(limit_number) => {
                let number: u32 = filtered_skus.iter().map(|&(_sku, num)| num).sum();
                number > *limit_number
            }
            ApplyConditionLimit::AmountAndNumber(..) => {
                //TODO not consider now
                true
            }
        };
    }

    fn apply_times<'a>(&self, items: &BoughtSkuSet<'a>) -> u32 {
        let s;
        let config;
        match self {
            ApplyObjectLimit::Product(c) => {
                s = &c.superposition;
                config = &c.apply_condition_limit;
            }
            ApplyObjectLimit::ShippingFee(c) => {
                s = &c.superposition;
                config = &c.apply_condition_limit;
            }
            ApplyObjectLimit::ProductAndShippingFee { .. } => {
                unimplemented!()
            }
        };
        s.apply_times(items, config)
    }

    fn product_percent_form_apply(
        &self,
        percent_value: Decimal,
        order: &Order,
    ) -> Result<Decimal, &'static str> {
        if let ApplyObjectLimit::Product(config) = self {
            let times = self.apply_times(&order.items);

            match config.apply_range_limit {
                ApplyRangeLimit::None => {
                    let items_amount =  order.items_amount();
                    let bonus = items_amount * percent_value * Decimal::from(times);
                    if bonus > items_amount {
                        Ok(items_amount)
                    } else {
                        Ok(bonus)
                    }
                }
                ApplyRangeLimit::Number(mut n) => {
                    let mut filtered_skus = config.product_set_limit.filter(&order.items);
                    filtered_skus.sort_by(|&(s1, _), &(s2, _)| {
                        return s1.price().cmp(&s2.price());
                    });

                    n = times * n;
                    let mut amount = dec!(0);
                    for (sku, num) in filtered_skus {
                        if num < n {
                            amount += sku.price() * Decimal::from(num);
                        } else {
                            amount += sku.price() * Decimal::from(n);
                        }
                        n -= num;
                    }
                    Ok(amount * percent_value)
                }
            }
        } else {
            Err("Method not suitable!")
        }
    }

    fn product_amount_form_apply(
        &self,
        amount_value: Decimal,
        order: &Order,
    ) -> Result<Decimal, &'static str> {
        if let ApplyObjectLimit::Product(config) = self {
            match config.apply_range_limit {
                ApplyRangeLimit::None | ApplyRangeLimit::Number(_) => {
                    let items_amount = order.items_amount();
                    let bonus = amount_value * Decimal::from(self.apply_times(&order.items));
                    if bonus > items_amount {
                        Ok(items_amount)
                    } else {
                        Ok(bonus)
                    }
                }
            }
        } else {
            Err("Method not suitable!")
        }
    }

    pub fn charge(&self, order: &Order) -> Decimal {
        let bonus;
        match self {
            ApplyObjectLimit::Product(c) => match c.bonus_form_limit {
                BonusFormLimit::Percent(r) => {
                    bonus = self.product_percent_form_apply(r, order).unwrap()
                }
                BonusFormLimit::Amount(v) => {
                    bonus = self.product_amount_form_apply(v, order).unwrap()
                }
            },
            ApplyObjectLimit::ShippingFee(c) => match c.bonus_form_limit {
                BonusFormLimit::Percent(r) => {
                    bonus =
                        order.shipping_fee * r * Decimal::from(self.apply_times(&order.items));
                }
                BonusFormLimit::Amount(v) => {
                    bonus = v * Decimal::from(self.apply_times(&order.items));
                }
            },
            ApplyObjectLimit::ProductAndShippingFee { .. } => {
                //TODO not consider now
                unimplemented!();
            }
        }
        bonus
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

pub enum SuperpositionLimit {
    Times(u32),
    None,
}

impl SuperpositionLimit {
    fn apply_times<'a>(&self, items: &BoughtSkuSet<'a>, config: &ApplyConditionLimit) -> u32 {
        match self {
            //FIXME
            SuperpositionLimit::Times(times) => times.clone(),
            SuperpositionLimit::None => {
                match config {
                    ApplyConditionLimit::None => 1,
                    ApplyConditionLimit::Amount(amount_condition) => {
                        (items.total_amount() / amount_condition).to_u32().unwrap()
                    }
                    ApplyConditionLimit::Number(number_condition) => {
                        items.total_number() / number_condition
                    }
                    ApplyConditionLimit::AmountAndNumber(_, _) => {
                        //TODO not consider now
                        unimplemented!()
                    }
                }
            }
        }
    }
}
