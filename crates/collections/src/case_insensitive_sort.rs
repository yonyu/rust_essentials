// Sort username case insensitively

fn sort_usernames<T: AsRef<str> + Ord>(usernames: &mut Vec<T>) {
    
    usernames.sort_by_cached_key(|name| name.as_ref().to_lowercase());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_usernames() {
        let mut usernames = vec!["Zach", "bob", "Alice", "alice", "Zach", "zach"];
        sort_usernames(&mut usernames);
        assert_eq!(usernames, ["Alice", "alice", "bob", "Zach", "Zach", "zach"]);
    }
}