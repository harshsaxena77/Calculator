/*
 * smartcalc v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;
use crate::config::SmartCalcConfig;
use crate::tokinizer::get_currency;
use crate::tokinizer::get_number;
use crate::tokinizer::get_number_or_price;
use crate::tokinizer::get_percent;
use crate::types::NumberType;
use crate::{tokinizer::{TokenInfo, Tokinizer}, types::TokenType};

use crate::tools::do_divition;

pub fn percent_calculator(_: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("p") && fields.contains_key("number") {
        let number = match get_number("number", fields) {
            Some(number) => number,
            _ => return Err("Number information not valid".to_string())
        };

        let percent = match get_percent("p", fields) {
            Some(number) => number,
            _ => return Err("Percent information not valid".to_string())
        };
        return Ok(TokenType::Number(do_divition(percent * number, 100.0), NumberType::Decimal));
    }

    Err("Percent not valid".to_string())
}

pub fn find_numbers_percent(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("part") && fields.contains_key("total") {
        let total = match get_number_or_price(config, "total", fields) {
            Some(number) => number,
            _ => return Err("Total number information not valid".to_string())
        };

        let part = match get_number_or_price(config, "part", fields) {
            Some(number) => number,
            _ => return Err("Part number information not valid".to_string())
        };
        
        return Ok(TokenType::Percent(do_divition(part * 100.0, total)));
    }

    Err("Find percent not valid".to_string())
}

pub fn find_total_from_percent(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("number_part") && fields.contains_key("percent_part") {
        let number_part = match get_number_or_price(config, "number_part", fields) {
            Some(number) => number,
            _ => return Err("Number part information not valid".to_string())
        };

        let percent_part = match get_percent("percent_part", fields) {
            Some(percent) => percent,
            _ => return Err("Percent part information not valid".to_string())
        };

        return Ok(match get_currency(config, "number_part", fields) {
            Some(currency) => TokenType::Money(do_divition(number_part * 100.0, percent_part), currency),
            None => TokenType::Number(do_divition(number_part * 100.0, percent_part), NumberType::Decimal)
        });
    }

    Err("Find percent not valid".to_string())
}


#[cfg(test)]
#[test]
fn find_percent_to_number_1() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;

    let tokens = execute("20 is 10% of what".to_string());
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(200.0, NumberType::Decimal)));

}

#[cfg(test)]
#[test]
fn find_percent_to_number_2() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("180 is 10% of what".to_string());

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(1800.0, NumberType::Decimal)));

}

#[cfg(test)]
#[test]
fn find_numbers_percent_1() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("15 is what % of 100".to_string());

    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Percent(15.00)));
}
