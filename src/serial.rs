use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub ingame_name: String,
    pub last_seen: String,
    pub reputation: i32,
    pub region: String,
    pub status: String,
    pub id: String,
    pub avatar: Option<String> // can be 'null'
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub quantity: i32,
    pub creation_date: String,
    pub visible: bool,
    pub user: User,
    pub last_update: String,
    pub platinum: i32,
    pub order_type: String,
    pub region: String,
    pub platform: String,
    pub id: String
}

impl Order {
    pub fn is_buy(&self) -> bool {
        self.order_type == "buy"
    }

    pub fn is_sell(&self) -> bool {
        !self.is_buy()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    orders: Vec<Order>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderList {
    payload: Payload
}

impl OrderList {
    pub fn orders(&self) -> &Vec<Order> {
        &self.payload.orders
    }

    pub fn sell_orders(&self) ->Vec<&Order> {
        self.payload.orders.iter().filter(|o| o.is_sell()).collect()
    }

    pub fn buy_orders(&self) -> Vec<&Order> {
        self.payload.orders.iter().filter(|o| o.is_buy()).collect()
    }
}