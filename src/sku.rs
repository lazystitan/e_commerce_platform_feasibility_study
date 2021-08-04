use crate::attribute::Attr;
use rust_decimal::prelude::*;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

pub type SkuName = String;

/// The most basic unit of product.
#[derive(Debug)]
pub struct StockKeepingUnit<'a> {
    sku: SkuName,
    shop_price: Decimal,
    // pub market_price: Decimal,
    weight: u32,
    attributes: HashSet<&'a Attr>,
}

/// Sku is considered unique enough for it's name(or id)
impl<'a> Hash for StockKeepingUnit<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sku.hash(state)
    }
}

impl<'a> PartialEq for StockKeepingUnit<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.name() == other.name() {
            true
        } else {
            false
        }
    }
}

impl<'a> Eq for StockKeepingUnit<'a> {}

pub type Sku<'a> = StockKeepingUnit<'a>;

/// Implement some interface instead of access attribute directly
impl<'a> StockKeepingUnit<'a> {
    pub fn new(
        sku: &str,
        shop_price: &str,
        market_price: &str,
        weight: u32,
        attr: HashSet<&'a Attr>,
    ) -> Self {
        Self {
            sku: String::from(sku),
            shop_price: Decimal::from_str(shop_price).unwrap(),
            // market_price: Decimal::from_str(market_price).unwrap(),
            weight,
            attributes: attr,
        }
    }

    pub fn name(&self) -> &str {
        &self.sku
    }

    pub fn price(&self) -> Decimal {
        self.shop_price
    }

    pub fn weight(&self) -> u32 {
        self.weight
    }

    pub fn attrs(&self) -> &HashSet<&'a Attr> {
        &self.attributes
    }
}

/// A map, it's key is a sku with a correspond number,
/// to represent a batch of sku each of them can added
/// to bag multiple times.
#[derive(Debug)]
pub struct BoughtSkuSet<'a> {
    skus: HashMap<Sku<'a>, u32>,
}

impl<'a> BoughtSkuSet<'a> {
    pub fn new() -> Self {
        Self {
            skus: HashMap::new(),
        }
    }

    /// With a conditioner function return bool filter out skus and numbers.
    pub fn filter<T>(&self, conditioner: T) -> Vec<(&Sku, u32)>
    where
        T: Fn(&Sku) -> bool,
    {
        self.skus
            .iter()
            .filter(|&(sku, _)| conditioner(sku))
            .map(|(sku, &number)| (sku, number))
            .collect()
    }

    /// Intersect operation with another.
    pub fn intersect(&self, other: &Self) -> Vec<(&Sku, u32)> {
        self
            .skus
            .iter()
            .filter(|&(sku, _)| other.skus.contains_key(sku))
            .map(|(sku, &number)| (sku, number))
            .collect()
    }

    /// Intersect operation with a set of sku name.
    pub fn intersect_by_sku_name(&self, other_keys: Vec<&str>) -> Vec<(&Sku, u32)> {
        self.skus
            .iter()
            .filter(|&(sku, _)| other_keys.contains(&sku.sku.as_str()))
            .map(|(sku, &number)| (sku, number))
            .collect()
    }

    /// As name.
    pub fn total_amount(&self) -> Decimal {
        self.skus.iter().map(|(sku, &number)| {
            sku.shop_price * Decimal::from(number)
        }).sum()
    }

    pub fn total_number(&self) -> u32 {
        self.skus.iter().map(|(_sku, &number)| {
            number
        }).sum()
    }
}
