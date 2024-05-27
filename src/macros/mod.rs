/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

#[macro_export]

macro_rules! gateway_err {
    ($name:ident, $message:expr $(, $arg_name:ident : $val:expr),*) => {
        gateway_err!(_ $name, $message, None $(, $arg_name : $val )* )
    };
    ($name:ident, $message:expr, $e:expr $(, $arg_name:ident : $val:expr),*) => {
        gateway_err!(_ $name, $message, Some(Box::new($e)) $(, $arg_name : $val )* )
    };
    (_ $name:ident, $message:expr, $e:expr $(, $arg_name:ident : $val:expr),*) => {
        GatewayError::$name {
            message: $message.into(),
            module_path: module_path!(),
            line: line!(),
            col: column!(),
            source: $e,
            $($arg_name: $val,)*
        }
    };
}

