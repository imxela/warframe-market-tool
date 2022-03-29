use crate::serial::Order;

pub fn prices(v: &Vec<&Order>) -> Vec<i32> {
    v.iter().map(|o| o.platinum).collect()
}