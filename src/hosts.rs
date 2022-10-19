use std::collections::HashMap;
use std::collections::HashSet;

pub fn remove_comments(text: &str) -> String {
    for (idx, chr) in text.chars().enumerate() {
        if chr == '#' {
            return text[0..idx].to_string();
        }
    }
    return text.to_string();
}

pub fn hosts_to_hashmap(hosts: &str) -> HashMap<String, HashSet<String>> {
    let mut hash: HashMap<String, HashSet<String>> = HashMap::new();
    for line in hosts.lines() {
        let cleared_line = remove_comments(line).trim().to_string();
        if (cleared_line.is_empty()) || (cleared_line.starts_with('#')) {
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