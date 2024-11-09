pub fn get_date_filter(start_date: Option<String>, end_date: Option<String>) -> String {
    match (start_date, end_date) {
        (Some(start), Some(end)) => format!("created:{}..{}", start, end),
        (Some(start), None) => format!("created:>={}", start),
        (None, Some(end)) => format!("created:<={}", end),
        (None, None) => String::from(""),
    }
}

pub fn get_status_filter(status: Option<String>) -> String {
    match status {
        Some(status) => format!("is:{}", status),
        None => String::from(""),
    }
}

pub fn get_author_filter(author: Option<String>) -> String {
    match author {
        Some(author) => format!("author:{}", author),
        None => String::from(""),
    }
}
