use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use rusty_money::{iso, iso::Currency, ExchangeRate, Money};

fn create_exchange_rate<'a>(
    payin_currency_code: &'a str,
    payout_currency_code: &'a str,
    exchange_rate: f64,
) -> Result<ExchangeRate<'a, Currency>, String> {
    let payin_currency = iso::find(payin_currency_code)
        .ok_or_else(|| "payin currency code not supported".to_string())?;
    let payout_currency = iso::find(payout_currency_code)
        .ok_or_else(|| "payout currency code not supported".to_string())?;
    let exchange_rate_decimal = Decimal::from_f64(exchange_rate)
        .ok_or_else(|| "exchange_rate is not a decimal".to_string())?;

    ExchangeRate::new(payin_currency, payout_currency, exchange_rate_decimal)
        .map_err(|money_err| format!("Failed to create exchange rate with error: {}", money_err))
}

pub fn convert_integer(
    payin_currency_code: &str,
    payout_currency_code: &str,
    payin_amount_subunits: i64,
    exchange_rate: f64,
) -> Result<i64, String> {
    let exchange_rate =
        create_exchange_rate(payin_currency_code, payout_currency_code, exchange_rate)?;

    let payin_amount_money = Money::from_minor(payin_amount_subunits, exchange_rate.from);
    let payout_amount_money = exchange_rate
        .convert(payin_amount_money)
        .map_err(|money_err| format!("Failed to convert money amount with error: {}", money_err))?;

    payout_amount_money
        .amount()
        .to_i64()
        .ok_or("Could not convert payout amount to integer".to_string())
}

pub fn convert_str(
    payin_currency_code: &str,
    payout_currency_code: &str,
    payin_amount: &str,
    exchange_rate: f64,
) -> Result<String, String> {
    let exchange_rate =
        create_exchange_rate(payin_currency_code, payout_currency_code, exchange_rate)?;

    let payin_amount_money = Money::from_str(payin_amount, exchange_rate.from)
        .map_err(|money_err| format!("Failed to format money amount with error: {}", money_err))?;

    let payout_amount_money = exchange_rate
        .convert(payin_amount_money)
        .map_err(|money_err| format!("Failed to convert money amount with error: {}", money_err))?;

    return Ok(payout_amount_money.to_string());
}

// TODO(diehuxx): Use generics to somehow combine convert_str and convert_integer into one function with smart typing