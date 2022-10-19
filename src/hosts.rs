use std::collections::HashMap;
use std::collections::HashSet;

pub fn remove_comments(line: &str) -> String {
    for (idx, chr) in line.chars().enumerate() {
        if chr == '#' {
            return line[0..idx].trim().to_string();
        }
    }
    return line.trim().to_string();
}

pub fn hosts_to_hashmap(hosts: &str) -> HashMap<String, HashSet<String>> {
    let mut hash: HashMap<String, HashSet<String>> = HashMap::new();
    for line in hosts.lines() {
        let cleared_line = remove_comments(line);
        if cleared_line.is_empty() {
            continue;
        }
        let addresses: Vec<String> = cleared_line.split(' ').map(|s| s.to_string()).collect();
        let destination_host =addresses[0].as_str();
        if !hash.contains_key(destination_host) {
            hash.insert(destination_host.to_string(),  HashSet::from_iter(addresses[1..].to_vec()));
        } else {
            hash.get_mut(destination_host).unwrap().extend(addresses[1..].to_vec());
        }
    }
    hash
}

pub fn hashmap_to_hosts<T: Into<Option<u16>>>(hashmap: &HashMap<String, HashSet<String>>, compression_level: T) -> String {
    let compression: usize = compression_level.into().unwrap_or(9) as usize;

    let mut hosts_text = String::new();

    for (destination, addresses) in hashmap.iter() {
        let addr_vec: Vec<String> = addresses.into_iter().map(|s| s.to_owned()).collect();
        for chunk in addr_vec.chunks(compression) {
            if chunk.len() > 0 {
                hosts_text.push_str(f!("{destination} {sources}\n", sources=chunk.join(" ")).as_str()) 
            }
        }
    }
    hosts_text
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_comments_with_inline_comment() {
        let test_string = "0.0.0.0 127.0.0.1 # loop";
        let expected = "0.0.0.0 127.0.0.1";
        assert_eq!(remove_comments(test_string), expected);
    }

    #[test]
    fn test_remove_comments_with_no_comment_trims() {
        let test_string = "0.0.0.0 127.0.0.1     ";
        let expected = "0.0.0.0 127.0.0.1";
        assert_eq!(remove_comments(test_string), expected);
    }

    #[test]
    fn test_hosts_to_hashmap_with_one_line_returns_one_entry() {
        let test_string = "0.0.0.0 127.0.0.1";
        
        let hashmap = hosts_to_hashmap(test_string);

        let values: Vec<String> = hashmap.get("0.0.0.0").unwrap().into_iter().map(|v| v.to_string()).collect();

        assert_eq!(values, vec!["127.0.0.1"]);
    }

    #[test]
    fn test_hosts_to_hashmap_with_two_lines_returns_two_entries() {
        let test_string = "
        0.0.0.0 127.0.0.1
        0.0.0.0 192.168.1.1
        ";

        let hashmap = hosts_to_hashmap(test_string);

        let mut values: Vec<String> = hashmap.get("0.0.0.0").unwrap().into_iter().map(|v| v.to_string()).collect();

        assert_eq!(values.sort(), vec!["127.0.0.1", "192.168.1.1"].sort());
    }

    #[test]
    fn test_hosts_to_hashmap_with_multiple_destinations() {
        let test_string = "
        0.0.0.0 108.1.1.1
        127.0.0.1 192.168.1.1
        ";

        let hashmap = hosts_to_hashmap(test_string);

        let mut values: Vec<String> = hashmap.keys().map(|v| v.to_string()).collect();

        assert_eq!(values.sort(), vec!["127.0.0.1", "0.0.0.0"].sort());
    }

    
    #[test]
    fn test_hosts_to_hashmap_with_multiple_destinations_and_comments() {
        let test_string = "
        0.0.0.0 108.1.1.1 # Dunno
        127.0.0.1 192.168.1.1 # Wowie
        ";

        let hashmap = hosts_to_hashmap(test_string);

        let mut values: Vec<String> = hashmap.keys().map(|v| v.to_string()).collect();

        assert_eq!(values.sort(), vec!["127.0.0.1", "0.0.0.0"].sort());
    }

}