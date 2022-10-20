use std::collections::HashMap;
use std::collections::HashSet;

/// Given a string, returns the string without any comments
/// 
/// # Examples
/// ```
/// hosts::remove_comments("0.0.0.0 127.0.0.1 # Testing Comments");
/// ```
pub fn remove_comments(line: &str) -> String {
    for (idx, chr) in line.chars().enumerate() {
        if chr == '#' {
            return line[0..idx].trim().to_string();
        }
    }
    return line.trim().to_string();
}

/// Converts a HostsFile string to a hashmap
/// 
/// # Theory
/// This is based off the idea that one hosts file might have a lot of different
/// destinations: For example, you may simply be using yours hosts file as a 
/// sink for ads/tracking etc, but you could also be using it to redirect 
/// `ff02::2 ip6-allrouters` and `::1 localhost ip6-localhost ip6-loopback` so
/// this hashmap is used like a Python Dictionary.
/// 
/// # Example
/// ```
/// let hosts_file = "
/// ::1 localhost ip6-localhost ip6-loopback
/// ff02::2 ip6-allrouters
/// 0.0.0.0 metrics.avalara.com # Source: Steven Black's Hosts
/// ";
/// let hashmap = hosts_to_hashmap(hosts_file);
/// ```
/// 
/// This will return [in JSON notation]:
/// ```json
/// {
///     "::1": ["localhost", "ip6-localhost", "ip6-loopback"],
///     "ff02::2": ["ip6-allrouters"],
///     "0.0.0.0": ["metrics.avalara.com"] 
/// }
/// ```
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

/// Converts a hashmap [like one created by hosts_to_hashmap] to a hosts file
/// 
/// # Theory
/// In converting to a hashmap as an intermediate, it allows duplicates to be 
/// removed from huge datasets.
/// 
/// ## What is `compression_level` and why is it needed?
/// Compression level sets the number of columns you want (n + 1) for your 
/// hosts file. For example 3 columns:
/// ```
/// 0.0.0.0 addr1 addr2 addr3
/// ```
/// 
/// And with 2 columns:
/// ```
/// 0.0.0.0 addr1 addr2
/// 0.0.0.0 addr3
/// ```
/// 
/// This is especially useful for Windows 10 systems (I am not sure about other
/// versions of windows but they are likely the same). The DNS Agent in Windows
/// is ancient and takes **a long time** to traverse large hosts files. If you 
/// are using a 4mb file, expect DNS lookups to take tens of seconds. This is 
/// obviously not enough performance in the modern day, so if you increase the
/// number of columns in each file, the DNS agent's algorithm seems to read them
/// much more effectively. I'd recommend setting your compression_level to 9 as 
/// this seems to give pretty consistent speeds. This is the same compression
/// used by [Unified Hosts Auto Update](https://github.com/ScriptTiger/Unified-Hosts-AutoUpdate)
/// 
/// # Example
/// If you had a hosts hashmap like the following [in JSON notation]:
/// ```json
/// {
///     "0.0.0.0": ["192.168.1.1", "182.1.1.0"],
///     "::1": ["localhost"]
/// }
/// ```
/// 
/// If you had a level 2 compression (2-col compression):
/// ```
/// let x = hashmap_to_hosts(example_hosts, 2);
/// ```
/// 
/// `x` would look like so:
/// ```
/// 0.0.0.0 192.168.1.1 182.1.1.0
/// ::1 localhost
/// ```
/// 
/// And when you have just level 1 compression (1-col compression):
/// ```
/// let x = hashmap_to_hosts(example_hosts, 1);
/// ```
/// 
/// `x` would look like so:
/// ```
/// 0.0.0.0 192.168.1.1
/// 0.0.0.0 182.1.1.0
/// ::1 localhost
/// ```
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

/// Removes all common elements from two hashmaps
pub fn ignore(hashmap: &mut HashMap<String, HashSet<String>>, ignore: &HashMap<String, HashSet<String>>) {
    let empty_hash: HashSet<String> = HashSet::new();

    for (destination, addresses) in hashmap.iter_mut() {
        let ignore_list = ignore.get(destination).unwrap_or(&empty_hash);
        let addr: HashSet<String> = HashSet::from_iter(addresses.difference(&ignore_list).map(|v| v.to_owned()));
        addresses.clone_from(&addr);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

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
        values.sort();

        assert_eq!(values, vec!["127.0.0.1", "192.168.1.1"]);
    }

    #[test]
    fn test_hosts_to_hashmap_with_multiple_destinations() {
        let test_string = "
        0.0.0.0 108.1.1.1
        127.0.0.1 192.168.1.1
        ";

        let hashmap = hosts_to_hashmap(test_string);

        let mut values: Vec<String> = hashmap.keys().map(|v| v.to_string()).collect();
        values.sort();

        assert_eq!(values, vec!["0.0.0.0", "127.0.0.1"]);
    }

    
    #[test]
    fn test_hosts_to_hashmap_with_multiple_destinations_and_comments() {
        let test_string = "
        0.0.0.0 108.1.1.1 # Dunno
        127.0.0.1 192.168.1.1 # Wowie
        ";

        let hashmap = hosts_to_hashmap(test_string);

        let mut values: Vec<String> = hashmap.keys().map(|v| v.to_string()).collect();
        values.sort();

        assert_eq!(values, vec!["0.0.0.0", "127.0.0.1"]);
    }

    #[test]
    fn test_hashmap_to_hosts_with_one_entry_returns_one_line() {
        let mut test_hashmap: HashMap<String, HashSet<String>> = HashMap::new();

        test_hashmap.insert("0.0.0.0".to_string(),HashSet::from_iter(vec!["127.0.0.1".to_string()]) );
        
        assert_eq!(hashmap_to_hosts(&test_hashmap, 1), "0.0.0.0 127.0.0.1\n")
    }

    #[test]
    fn test_hashmap_to_hosts_with_two_entries_returns_two_lines_in_any_order() {
        let mut test_hashmap: HashMap<String, HashSet<String>> = HashMap::new();

        test_hashmap.insert("0.0.0.0".to_string(),
                            HashSet::from_iter(vec!["127.0.0.1".to_string(), "128.0.0.1".to_string()])
                        );
        
        // Order of Hashes are inderterminant
        let hosts = hashmap_to_hosts(&test_hashmap, 1);

        let order1 = hosts == "0.0.0.0 127.0.0.1\n0.0.0.0 128.0.0.1\n";
        let order2 = hosts == "0.0.0.0 128.0.0.1\n0.0.0.0 127.0.0.1\n";

        assert!(order1 || order2);
    }

    #[test]
    fn test_hashmap_to_hosts_with_compression_level_2_makes_two_columns() {
        let mut test_hashmap: HashMap<String, HashSet<String>> = HashMap::new();

        test_hashmap.insert("0.0.0.0".to_string(),
                            HashSet::from_iter(vec!["127.0.0.1".to_string(), "128.0.0.1".to_string()])
                        );
        
        let hosts = hashmap_to_hosts(&test_hashmap, 2);
        // Order of Hashes are inderterminant
        let order1 = hosts == "0.0.0.0 127.0.0.1 128.0.0.1\n";
        let order2 = hosts == "0.0.0.0 128.0.0.1 127.0.0.1\n";

        assert!(order1 || order2);
    }

    #[test]
    fn test_ignore_removes_common_values() {
        let mut test_hashmap: HashMap<String, HashSet<String>> = HashMap::new();

        test_hashmap.insert("0.0.0.0".to_string(),
                            HashSet::from_iter(vec!["127.0.0.1".to_string(), "128.0.0.1".to_string()])
                        );

        let mut ignore_hashmap: HashMap<String, HashSet<String>> = HashMap::new();
        ignore_hashmap.insert("0.0.0.0".to_string(),
            HashSet::from_iter(vec!["128.0.0.1".to_string()])
        );

        ignore(&mut test_hashmap, &ignore_hashmap);

        let mut values: Vec<String> = test_hashmap.get("0.0.0.0").unwrap().into_iter().map(|v| v.to_string()).collect();
        values.sort();


        assert_eq!(values, vec!["127.0.0.1"]);

    }
}