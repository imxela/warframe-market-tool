use reqwest;

use crate::serial::OrderList;

const URL: &str = "https://api.warframe.market/v1/";

pub fn query_item(item_id: &str) -> OrderList {
    let query: String = format!("items/{}/orders", item_id);

    let orders = reqwest::blocking::get(format!("{}{}", URL, query)).expect("failed to query market API")
        .json::<OrderList>().expect("failed to parse market API response as JSON");

    orders
}