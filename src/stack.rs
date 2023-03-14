use crate::ltypes::*;

pub mod stack_code {
    use super::LValueType;
    
    pub fn pop_one(list: &mut Vec<String>) -> LValueType {
        let value = list.pop();
        if let Option::None = value {
            return LValueType::None;
        }

        let value = value.unwrap();

        if value.starts_with("\"") && value.ends_with("\"") {
            return LValueType::Text((&value[1..value.len() - 1]).to_string());
        } else if let Ok(f) = value.parse::<f32>() {
            return LValueType::Number(f);
        }

        return LValueType::Symbol(value);
    }

    pub fn pop_two(list: &mut Vec<String>) -> (LValueType, LValueType) {
        let first = pop_one(list);
        let second = pop_one(list);

        (first, second)
    }
}

pub mod stack_runtime {
    use super::LValue;

    pub fn pop_one(list: &mut Vec<LValue>) -> LValue {
        list.pop().unwrap_or_else(|| LValue::Number(0.0))
    }


    pub fn pop_two(list: &mut Vec<LValue>) -> (LValue, LValue) {
        let first = pop_one(list);
        let second = pop_one(list);

        (second, first)
    }
}