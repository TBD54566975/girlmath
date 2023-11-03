use iso_currency::Currency;
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn major_to_minor(currency_code: &str, amount: &str) -> Result<String, String> {
    let currency = match Currency::from_str(currency_code) {
        Ok(val) => val,
        Err(_) => return Err("currency code not supported".to_string()),
    };

    let currency_symbol = currency.symbol().symbol;
    let amount_formatted = amount
        .replace(&currency_symbol, "")
        .replace(",", "");

    let decimal_amount = match Decimal::from_str(&amount_formatted) {
        Ok(val) => val,
        Err(_) => return Err("amount is not a decimal".to_string()),
    };

    let multiplier = match currency.exponent() {
        Some(significant_digits) => Decimal::from(10u32.pow(significant_digits.into())),
        None => Decimal::from(1), // some currencies don't have significant digits (e.g. JPY)
    };

    let minor_units = decimal_amount * multiplier;

    return Ok(minor_units.floor().to_string());
}

pub fn minor_to_major(currency_code: &str, amount: &str) -> Result<String, String> {
    let currency = match Currency::from_str(currency_code) {
        Ok(val) => val,
        Err(_) => return Err("currency code not supported".to_string()),
    };

    let amount_without_subunit = match currency.symbol().subunit_symbol {
        Some(subunit_symbol) => amount.replace(&subunit_symbol, ""),
        None => amount.to_string(),
    };
    let amount_formatted = amount_without_subunit.replace(",", "");

    let decimal_amount = match Decimal::from_str(&amount_formatted) {
        Ok(val) => val,
        Err(_) => return Err("amount is not a decimal".to_string()),
    };

    let multiplier = match currency.exponent() {
        Some(significant_digits) => Decimal::from(10u32.pow(significant_digits.into())),
        None => Decimal::from(1), // some currencies don't have significant digits (e.g. JPY)
    };

    let major_units = decimal_amount / multiplier;

    return Ok(major_units.to_string());
}

#[cfg(test)]
mod test {
    use super::minor_to_major;
    use super::major_to_minor;

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
        let major_amount = "€12,345.67";
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

    #[test]
    fn cents_with_symbol() {
        let currency_code: &str = "USD";
        let minor_amount = "99¢";
        let major_amount = minor_to_major(currency_code, minor_amount).unwrap();
        assert_eq!(major_amount, "0.99");
    }
}