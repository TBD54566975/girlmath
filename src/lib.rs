use rusty_money::{Money, iso};
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn major_to_minor(currency_code: &str, amount: &str) -> Result<String, String> {
    let currency = match iso::find(currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };

    let multiplier = Decimal::from(10u32.pow(currency.exponent));

    let amount_without_currency_symbol = amount.replace(&currency.symbol, "");
    let amount_money = match Money::from_str(&amount_without_currency_symbol, currency) {
        Ok(val) => val,
        Err(_) => return Err("amount is not a decimal".to_string()),
    };
    let amount_decimal = amount_money.amount();

    let minor_amount = amount_decimal * multiplier;

    return Ok(minor_amount.floor().to_string());
}

pub fn minor_to_major(currency_code: &str, amount: &str) -> Result<String, String> {
    let currency = match iso::find(currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };

    let decimal_amount = match Decimal::from_str(amount) {
        Ok(val) => val,
        Err(_) => return Err("amount is not a decimal".to_string()),
    };

    let multiplier = Decimal::from(10u32.pow(currency.exponent));

    let major_units = decimal_amount / multiplier;

    return Ok(major_units.to_string());
}


#[cfg(test)]
mod test {
    use super::{major_to_minor, minor_to_major};

    #[test]
    fn dollar_less_than_one() {
        let currency_code = "USD";
        let major_amount = "0.99";

        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "99");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, major_amount);
    }

    #[test]
    fn dollar_more_than_one() {
        let currency_code = "USD";
        let major_amount = "1.50";

        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "150");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, major_amount);
    }

    #[test]
    fn dollar_thousands() {
        let currency_code = "USD";
        let major_amount = "12345.67";
        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "1234567");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, major_amount);
    }

    #[test]
    fn dollar_with_dollar_sign() {
        let currency_code = "USD";
        let major_amount = "$12345.67";
        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "1234567");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, "12345.67");
    }

    #[test]
    fn dollar_with_dollar_sign_and_thousands_comma() {
        let currency_code = "USD";
        let major_amount = "$12,345.67";
        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "1234567");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, "12345.67");
    }

    #[test]
    fn euro_with_euro_symbol_and_thousands_comma() {
        let currency_code = "EUR";
        let major_amount = "€12.345,67";
        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "1234567");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, "12345.67");
    }

    #[test]
    fn yen_with_symbol() {
        let currency_code = "JPY";
        let major_amount = "¥12345";
        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "12345");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, "12345");
    }
}