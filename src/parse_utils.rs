use rust_lisp::model::Value;

pub fn parse_value(val: &Value, first_call: bool) -> String {
    let mut to_return = String::new();

    match val {
        Value::Symbol(symbol) => {
            let mut symbol_str = String::new();

            if symbol.len() == 1 {
                symbol_str.push('\'');
                symbol_str.push_str(symbol);
                symbol_str.push('\'');
            } else {
                symbol_str.push('"');
                symbol_str.push_str(symbol);
                symbol_str.push('"');
            }

            to_return.push_str(&symbol_str);
        }

        Value::True => to_return.push_str("true"),
        Value::False => to_return.push_str("false"),
        Value::Nil => to_return.push_str("None"),

        Value::List(list) => {
            to_return.push('(');
            for val in list.into_iter() {
                to_return.push_str(&parse_value(val, true));
            }
            to_return.push(')');
        }

        Value::String(_) | Value::Int(_) | Value::Float(_) => to_return.push_str(&val.to_string()),

        v => {
            let v = v.type_name();
            let mut iter_v = v.chars();
            let mut type_name = iter_v.next().unwrap().to_ascii_uppercase().to_string();
            type_name.push_str(&iter_v.collect::<String>());

            panic!("Unsupported return type: {}", type_name)
        }
    }

    if first_call {
        to_return.push(',');
    }

    to_return
}

