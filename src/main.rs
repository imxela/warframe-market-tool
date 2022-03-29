mod serial;
mod statistics;
mod market;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let name = prompt("What item do you want to query?");
        let item_id = name.replace(" ", "_").to_ascii_lowercase();

        let orders = market::query_item(item_id.as_str());
        let mut sell_prices: Vec<i32> = utils::prices(&orders.sell_orders());
        let mut buy_prices: Vec<i32> = utils::prices(&orders.buy_orders());

        sell_prices.sort();
        buy_prices.sort();

        println!("Sell count: {}", sell_prices.len());
        println!("Buy count: {}", buy_prices.len());

        println!("Average sell price: {}p", statistics::average(&mut sell_prices));
        println!("Outlier average sell price: {}p", statistics::outlier_average(&mut sell_prices, 0.1, None, None));
        println!("Deviation average sell price: {}p (std. deviation cutoff ±{}p)", statistics::deviation_average(&mut sell_prices) as i32, statistics::deviation(&sell_prices) as i32);

        println!("Average buy price: {}p", statistics::average(&mut buy_prices));
        println!("Outlier average buy price: {}p", statistics::outlier_average(&mut buy_prices, 0.1, None, None));
        println!("Deviation average buy price: {}p (std. deviation cutoff ±{}p)", statistics::deviation_average(&mut buy_prices) as i32, statistics::deviation(&buy_prices) as i32);

        println!("Lowest sell: {}p", sell_prices[0]);
        println!("Highest buy: {}p", buy_prices[buy_prices.len() - 1]);
        
        prompt("Press <enter> to return");
    }
}

use std::io::{stdin,stdout,Write};

fn input() -> String {
    print!("> ");
    stdout().flush().expect("failed to flush standard output");

    let mut input = String::new();
    let mut result = stdin().read_line(&mut input);

    while result.is_err() {
        println!("Invalid input, please try again!");

        result = stdin().read_line(&mut input);
    }

    String::from(input.trim())
}

fn prompt(question: &str) -> String {
    println!("{}", question);
    input()
}