/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::str::FromStr;

pub fn string_to_number<T>(value: String) -> Result<T, <T as FromStr>::Err> 
where T: FromStr
{
    value.parse::<T>()
}
fn is_number(s: &str) -> bool {
    s.chars().all(char::is_numeric)
}