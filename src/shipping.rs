use crate::order::{ApplyToOrder, Order};

struct Region {
    region_id: i32,
    region_code: String,
    region_name: String,
}

impl Region {
    fn new() -> Self {
        Self {
            region_id: 0,
            region_code: String::new(),
            region_name: String::new(),
        }
    }
}

pub struct Address {
    country: Region,
    province: Region,
    city: Region,
    zip_code: String,
}

impl Address {
    pub fn new() -> Self {
        Self {
            country: Region::new(),
            province: Region::new(),
            city: Region::new(),
            zip_code: String::new(),
        }
    }
}

pub enum ShippingMethod {
    StandardShipping(Address),
    ExpeditedShipping(Address),
}

impl ShippingMethod {
    fn id(&self) -> i32 {
        match self {
            ShippingMethod::StandardShipping(_) => 1,
            ShippingMethod::ExpeditedShipping(_) => 2,
        }
    }
}

impl ApplyToOrder for ShippingMethod {
    fn apply_to_order(&self, order: &mut Order) {
        match self {
            ShippingMethod::StandardShipping(_) => {
                order.shipping_fee = dec!(10.87);
            }
            ShippingMethod::ExpeditedShipping(_) => order.shipping_fee = dec!(21.77),
        }
    }
}