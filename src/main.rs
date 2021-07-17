// use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use std::cmp::Ordering;

#[macro_use]
extern crate rust_decimal_macros;

trait ApplyToOrder {
    fn apply_to_order(&self, order: &mut Order);
}

#[derive(Debug)]
struct StockKeepingUnit {
    sku: String,
    shop_price: Decimal,
    market_price: Decimal,
    weight: u32,
}

#[derive(Debug)]
struct OrderItem {
    sku: StockKeepingUnit,
    number: u32,
}

impl StockKeepingUnit {
    fn new(sku: &str, shop_price: &str, market_price: &str, weight: u32) -> Self {
        Self {
            sku: String::from(sku),
            shop_price: Decimal::from_str(shop_price).unwrap(),
            market_price: Decimal::from_str(market_price).unwrap(),
            weight,
        }
    }
}

enum ShippingMethod {
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

struct Address {
    country: Region,
    province: Region,
    city: Region,
    zip_code: String,
}

impl Address {
    fn new() -> Self {
        Self {
            country: Region::new(),
            province: Region::new(),
            city: Region::new(),
            zip_code: String::new(),
        }
    }
}

enum Coupon {
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

#[derive(Debug)]
struct Order {
    items: Vec<OrderItem>,
    // coupons: Vec<Coupon>,
    // shipping_method: ShippingMethod,
    coupon_bonus: Decimal,
    activity_bonus: Decimal,
    shipping_fee: Decimal,
    items_amount: Decimal,
    total_amount: Decimal,

    status_coupon: bool,
    status_activity: bool,
    status_shipping: bool,
    status_total: bool,
}

impl Order {
    fn new() -> Self {
        Self {
            items: Vec::new(),

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

    fn process_items(&mut self) {
        self.items_amount = self
            .items
            .iter()
            .map(|item| item.sku.shop_price * Decimal::from(item.number))
            .sum::<Decimal>()
    }

    fn process_activity(&mut self, activities: &Vec<Activity>) {
        for activity in activities {
            activity.apply_to_order(self);
        }
        self.status_activity = true;
    }

    fn process_shipping_fee(&mut self, shipping_method: &ShippingMethod) {
        let _total_weight = self.items.iter().map(|sku| sku.sku.weight).sum::<u32>();
        shipping_method.apply_to_order(self);
        self.status_shipping = true;
    }

    fn process_coupon(&mut self, coupons: &Vec<Coupon>) {
        for coupon in coupons {
            coupon.apply_to_order(self);
        }
        self.status_coupon = true;
    }

    fn process_summary(&mut self) {
        self.total_amount = self.items_amount + self.shipping_fee - self.coupon_bonus - self.activity_bonus
    }
}

enum Activity {
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
            Activity::FullMinusOne { skus,  .. } => { skus }
            Activity::FixedPrice { skus, .. } => { skus }
        }
    }
}

trait FilterActivityOrderItems<'a> {
    fn filter_activity_order_items(&self, activity: &Activity ) -> Self;
}

impl <'a> FilterActivityOrderItems<'a> for Vec<&'a OrderItem> {
    fn filter_activity_order_items(&self, activity: &Activity) -> Self {
        self.iter()
            .filter(|sku| activity.skus().iter().any(|activity_sku| activity_sku == &sku.sku.sku))
            .map(|item| *item)
            .collect()
    }
}

impl ApplyToOrder for Activity {

    fn apply_to_order(&self, order: &mut Order) {
        match self {
            Activity::FullMinusOne { full_number, skus } => {
                let mut in_activity_items = order.items.iter().collect::<Vec<&OrderItem>>().filter_activity_order_items(self);

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

                    let mut available_items_number = (in_activity_items_number / full_number) as u32 * full_number;
                    let mut activity_bonus = dec!(0);
                    for in_activity_item in in_activity_items {
                        if available_items_number >= in_activity_item.number {
                            activity_bonus += Decimal::from(in_activity_item.number) * in_activity_item.sku.shop_price;
                            available_items_number -= in_activity_item.number;
                        } else {
                            activity_bonus += Decimal::from(available_items_number) * in_activity_item.sku.shop_price;
                            break;
                        }
                    }

                    order.activity_bonus += activity_bonus;
                }
            }
            Activity::FixedPrice {
                fixed_number,
                fixed_price,
                skus,
            } => {
                let mut in_activity_items = order.items.iter().collect::<Vec<&OrderItem>>().filter_activity_order_items(self);

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


                    let mut available_items_number = (in_activity_items_number / fixed_number) as u32 * fixed_number;
                    let mut origin_price = dec!(0);
                    for in_activity_item in in_activity_items {
                        if available_items_number >= in_activity_item.number {
                            origin_price += Decimal::from(in_activity_item.number) * in_activity_item.sku.shop_price;
                            available_items_number -= in_activity_item.number;
                        } else {
                            origin_price += Decimal::from(available_items_number) * in_activity_item.sku.shop_price;
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

struct NumberS(i32);

fn main() {
    let skus = [
        ("10001200014432300015987", "5.99", "7.99", 3, 200),
        ("10001200024432300015938", "3.99", "4.99", 2, 200),
        ("10001200026432300015938", "5.99", "8.99", 3, 200),
        ("10001200026432300015939", "12.99", "16.99", 4, 200),
        ("10001200026432300025938", "90.99", "100.99", 5, 200),
        ("10001200076432300025938", "23.99", "26.99", 6, 200),
        ("10001200076432300025934", "56.99", "70.99", 7, 200),
        ("10001200076432300035934", "77.99", "168.99", 8, 200),
        ("10001200076432300045934", "89.99", "100.99", 1, 200),
        ("10001200076412300025984", "16.99", "23.99", 2, 200),
        ("10001200074412300015984", "30.99", "33.99", 3, 200),
        ("10001200074412300010984", "12.99", "15.99", 4, 200),
        ("10001200074412400010984", "14.99", "17.99", 5, 200),
        ("10001200024412300010984", "16.99", "19.99", 1, 200),
        ("10001200024412300010988", "17.99", "22.99", 2, 200),
    ];
    let mut items = Vec::new();

    for sku in skus.iter() {
        items.push(OrderItem {
            sku: StockKeepingUnit::new(sku.0, sku.1, sku.2, sku.4),
            number: sku.3,
        });
    }

    // items.sort_by(|pre_sku, next_sku| {
    //     if pre_sku.sku.shop_price < next_sku.sku.shop_price {
    //         Ordering::Less
    //     } else {
    //         Ordering::Greater
    //     }
    // });
    //
    // println!("{:?}", items);
    //
    // return;

    let mut order = Order::new();
    order.items = items;

    let full_minus_one_activity_skus = vec![
        String::from("10001200024432300015938"),
        String::from("10001200026432300015938"),
        String::from("10001200026432300015939"),
        String::from("10001200074412300015984"),
    ];

    let full_minus_one_activity = Activity::FullMinusOne {
        full_number: 4,
        skus: full_minus_one_activity_skus,
    };

    let fixed_price_activity_skus = vec![
        String::from("10001200076432300025934"),
        String::from("10001200076432300035934"),
        String::from("10001200024412300010988"),
    ];

    let fixed_price = Activity::FixedPrice {
        fixed_number: 3,
        fixed_price: dec!(50.00),
        skus: fixed_price_activity_skus,
    };

    let activities = vec![full_minus_one_activity, fixed_price];

    let product_coupon = Coupon::ProductCoupon(dec!(5));

    let coupons = vec![product_coupon];

    let shipping_method = ShippingMethod::StandardShipping(Address::new());

    order.process_items();
    order.process_activity(&activities);
    order.process_shipping_fee(&shipping_method);
    order.process_coupon(&coupons);
    order.process_summary();

    print!("{:?}", order);
}
