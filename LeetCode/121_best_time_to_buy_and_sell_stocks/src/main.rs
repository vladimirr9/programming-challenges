fn main() {}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
        return 0;
    }
    let end = prices.len() - 1;
    let mut highest_encountered_sale_day = prices[end];
    let mut highest_profit = 0;

    for i in (0..end).rev() {
        let buy_day_price = prices[i];
        if highest_encountered_sale_day - buy_day_price > highest_profit {
            highest_profit = highest_encountered_sale_day - buy_day_price;
        }
        if buy_day_price > highest_encountered_sale_day {
            highest_encountered_sale_day = buy_day_price
        }
    }
    return highest_profit;
}


#[test]
fn first_test() {
    let prices = vec![7,1,5,3,6,4];
    assert_eq!(max_profit(prices), 5);
}

#[test]
fn second_test() {
    let prices = vec![7,6,4,3,1];
    assert_eq!(max_profit(prices), 0);
}
