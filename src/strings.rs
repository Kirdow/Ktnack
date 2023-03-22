
pub struct StrLitMatch<'a> {
    goal: &'a str,
    no_goal: &'a str,
    end: bool,
}

impl StrLitMatch<'_> {
    pub fn new<'a>(value: &String, goal: &'a str, no_goal: &'a str) -> StrLitMatch<'a> {
        let end = value.ends_with(goal) && !value.ends_with(no_goal) && value.len() >= 2;
        StrLitMatch {
            goal,
            no_goal,
            end,
        }
    }

    pub fn ends(&self, value: &String) -> bool {
        return value.ends_with(self.goal) && !value.ends_with(self.no_goal);
    }
}

pub fn format_string_token(text: &String) -> String {
    let text = text
        .replace("\\n", "\n").replace("\\t", "\t")
        .replace("\\\"", "\"").replace("\\\'", "\'")
        .replace("\\\\", "\\");

    if text.starts_with("'") {
        assert!(text.len() == 3, "character typed strings can only have a single character!");
    }

    text
}

pub fn check_string_literal(value: &String) -> Option<StrLitMatch> {
    if !value.starts_with("\"") && !value.starts_with("'") {
        return Option::None;
    }

    let (goal, no_goal) = if value.starts_with("\"") { ("\"", "\\\"") } else { ("'", "\\'") };
    Option::Some(StrLitMatch::new(value, goal, no_goal))
}

pub fn fetch_string(value: &String, code: &mut Vec<String>) -> String {
    if let Some(strlit) = check_string_literal(value) {
        if strlit.end {
            return format_string_token(value);
        }

        let mut list: Vec<String> = vec![value.clone()];
        while let Some(value2) = code.pop() {
            list.push(value2.clone());
            if strlit.ends(&value2) {
                break;
            }
        }

        return format_string_token(&list.join(" "));
    }

    return value.clone();
}