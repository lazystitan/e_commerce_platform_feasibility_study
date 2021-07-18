use crate::shipping::{ShippingMethod, Address};
use crate::activity::{FullMinusOneActivity, FixedPriceActivity, ActivityBehavior};
use crate::coupon::Coupon;
use crate::order::{OrderItem, StockKeepingUnit, Order};

#[macro_use]
extern crate rust_decimal_macros;

mod activity;
mod coupon;
mod order;
mod shipping;

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

    let mut order = Order::new();
    order.items = items;

    let full_minus_one_activity_skus = vec![
        String::from("10001200024432300015938"),
        String::from("10001200026432300015938"),
        String::from("10001200026432300015939"),
        String::from("10001200074412300015984"),
    ];

    let full_minus_one_activity = FullMinusOneActivity {
        full_number: 4,
        skus: full_minus_one_activity_skus,
    };

    let fixed_price_activity_skus = vec![
        String::from("10001200076432300025934"),
        String::from("10001200076432300035934"),
        String::from("10001200024412300010988"),
    ];

    let fixed_price = FixedPriceActivity {
        fixed_number: 3,
        fixed_price: dec!(50.00),
        skus: fixed_price_activity_skus,
    };

    let activities: Vec<Box<dyn ActivityBehavior>> = vec![Box::new(full_minus_one_activity), Box::new(fixed_price)];

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
