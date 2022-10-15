use std::collections::HashMap;

fn download_text<'a>(url: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(url)
        .call()?
        .into_string()?;
    Ok(body)
}

fn remove_comments(text: &str) -> String {
    for (idx, chr) in text.chars().enumerate() {
        if chr == '#' {
            return text[0..idx].to_string();
        }
    }
    return text.to_string();
}

fn hosts_to_hashmap(hosts: &str) -> HashMap<String, Vec<String>> {
    let mut hash: HashMap<String, Vec<String>> = HashMap::new();
    for line in hosts.lines() {
        let cleared_line = remove_comments(line).trim().to_string();
        if (cleared_line.is_empty()) || (cleared_line.starts_with('#')) {
            continue;
        }
        let addresses: Vec<String> = cleared_line.split(' ').map(|s| s.to_string()).collect();
        let destination_host =addresses[0].as_str();
        if !hash.contains_key(destination_host) {
            hash.insert(destination_host.to_string(), addresses[1..].to_vec() );
        } else {
            hash.get_mut(destination_host).unwrap().extend(addresses[1..].to_vec());
        }
    }

    hash
}

fn main() -> Result<(), ureq::Error> {
    let mut urls: Vec<String> = Vec::new();
    urls.push(String::from("https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-porn-social/hosts"));

    let mut all_hosts = String::new();

    for url in urls.iter() {
        println!("{}", url);
        let text = download_text(url)?;
        all_hosts.push_str(text.as_str());
    }

    let hosts_hash = hosts_to_hashmap(all_hosts.as_str());
    


    println!("Hello, world!");
    Ok(())
}
