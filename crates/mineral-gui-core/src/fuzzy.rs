use pinyin::ToPinyin;

pub fn fuzzy_match(query: &str, text: &str) -> bool {
    let needle = query.trim().to_lowercase();
    if needle.is_empty() {
        return true;
    }
    let mut haystack = text.to_lowercase();
    let mut full = String::new();
    let mut initials = String::new();
    for py in text.to_pinyin() {
        if let Some(py) = py {
            full.push_str(py.plain());
            initials.push_str(py.first_letter());
        }
    }
    if !full.is_empty() {
        haystack.push(' ');
        haystack.push_str(&full);
        haystack.push(' ');
        haystack.push_str(&initials);
    }
    if haystack.contains(&needle) {
        return true;
    }
    let mut index = 0;
    let chars: Vec<char> = needle.chars().collect();
    for ch in haystack.chars() {
        if ch == chars[index] {
            index += 1;
            if index == chars.len() {
                return true;
            }
        }
    }
    false
}
