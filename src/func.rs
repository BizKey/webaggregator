use crate::templates::DvaResult;

pub fn simulate_dva(prices: &[f64], target_increment: f64, commission_rate: f64) -> DvaResult {
    let n = prices.len();
    let mut target_value = 0.0;
    let mut asset_amount = 0.0;
    let mut total_gross_spent = 0.0;
    let mut total_gross_received = 0.0;

    for (i, &price) in prices.iter().enumerate() {
        target_value += target_increment;

        let current_value = asset_amount * price;

        let delta = target_value - current_value;

        if delta > 0.0 {
            let gross_amount = delta;
            let net_amount = gross_amount / (1.0 + commission_rate);
            let bought = net_amount / price;
            asset_amount += bought;
            total_gross_spent += gross_amount;
        } else if delta < 0.0 {
            let gross_amount = -delta;
            let net_amount = gross_amount * (1.0 - commission_rate);
            let sold = gross_amount / price;
            asset_amount -= sold;

            if asset_amount < 0.0 {
                asset_amount = 0.0;
            }
            total_gross_received += net_amount;
        }
    }

    let final_price = *prices.last().unwrap();
    let final_value = asset_amount * final_price;
    let net_invested = total_gross_spent - total_gross_received;
    let profit = final_value - net_invested;
    let roi = if net_invested != 0.0 {
        profit / net_invested
    } else {
        0.0
    };

    DvaResult {
        commission_rate: commission_rate,
        periods: n,
        total_gross_spent: format!("{:.2}", total_gross_spent),
        total_gross_received: format!("{:.2}", total_gross_received),
        net_invested: format!("{:.2}", net_invested),
        final_asset_amount: format!("{:.4}", asset_amount),
        final_price: format!("{}", final_price),
        final_value: format!("{}", final_value),
        profit: format!("{:.2}", profit),
        roi: format!("{:.2}", roi),
    }
}
