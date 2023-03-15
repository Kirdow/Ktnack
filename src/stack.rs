use crate::ltypes::*;

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

    pub fn push_one(list: &mut Vec<LValue>, value: &LValue) {
        list.push(value.to_owned());
    }

    pub fn push_two(list: &mut Vec<LValue>, values: (&LValue, &LValue)) {
        let (first, second) = values;
        push_one(list, first);
        push_one(list, second);
    }
}