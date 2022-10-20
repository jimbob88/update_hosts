//! Fast & Simple Hosts Management
//! 
//! A piece of software for downloading and updating hosts files. I am no kind
//! of authority in Rust, I am merely trying to learn the language. A similar
//! version of this software can be found for Python in my GitHub Gists.

#[macro_use]
extern crate fstrings;

mod hosts;
mod download;

use std::fs;

/// Downloads host files into one long host file
/// 
/// # Arguments
/// * `urls` - A list of urls (these can be local urls using `file://`) i.e.
/// <file:///etc/hosts> will open the hosts. And "<https://www.example.com/host>" 
/// will download from the website "example.com".
fn get_hosts(urls: &Vec<String>) -> String {
    let mut all_hosts = String::new();

    for url in urls.iter() {
        let text: String;
        if url.starts_with("file://") {
            let file = url.strip_prefix("file://").unwrap();
            text = fs::read_to_string(file).expect("Unable to open file");
        } else {
            text = download::download_text(url).expect("Error failed to download");
        }
        all_hosts.push_str(text.as_str());
    }

    all_hosts
}

/// This is where the magic happens!
/// 
/// # The Command Line Interface
/// ```
/// A piece of software for updating hosts files
/// 
/// Usage: update_hosts2.exe [OPTIONS] --urls <urls>...
/// 
/// Options:
///   -u, --urls <urls>...                   A list of files to download
///   -o, --out <out>                        Where to write the file to [default: hosts]
///   -c, --compression_level <compression>  The number of columns to use [default: 9]
///   -i, --ignore [<ignore>...]             Select urls/files to use as an ignore list
///   -h, --help                             Print help information
///   -V, --version                          Print version information
/// ```
pub fn main() -> Result<(), ()> {
    let matches = clap::Command::new("HostsManager")
                    .version("0.0.1")
                    .about("A piece of software for updating hosts files")
                    .arg(
                        clap::Arg::new("urls")
                            .short('u')
                            .long("urls")
                            .num_args(1..)
                            .required(true)
                            .help("A list of files to download")
                    )
                    .arg(
                        clap::Arg::new("out")
                            .short('o')
                            .long("out")
                            .help("Where to write the file to")
                            .default_value("hosts")
                    )
                    .arg(
                        clap::Arg::new("compression")
                            .short('c')   
                            .long("compression_level")
                            .default_value("9")
                            .help("The number of columns to use")
                            .allow_negative_numbers(false)
                    )
                    .arg(
                        clap::Arg::new("ignore")
                            .short('i')
                            .long("ignore")
                            .default_value(None)
                            .help("Select urls/files to use as an ignore list")
                            .num_args(0..)
                            .required(false)
                    )
                    .get_matches();


    let urls: Vec<String> = matches.get_many::<String>("urls").unwrap().map(|v| v.to_owned()).collect();
    let out_file = matches.get_one::<String>("out").unwrap();
    let compression_level = matches.get_one::<String>("compression").unwrap().parse::<u16>().unwrap();
    let ignore_list: Vec<String> = {
        let ign = matches.get_many::<String>("ignore");
        if let Some(x) = ign {
            x.map(|v| v.to_owned()).collect()
        } else {
            vec![]
        }
    };

    let all_hosts = get_hosts(&urls);
    let all_ignore = get_hosts(&ignore_list);

    let mut hosts_hash = hosts::hosts_to_hashmap(all_hosts.as_str());
    let ignore_hash = hosts::hosts_to_hashmap(all_ignore.as_str());
    
    hosts::ignore(&mut hosts_hash, &ignore_hash);

    let hosts_text = hosts::hashmap_to_hosts(&hosts_hash, compression_level);

    fs::write(out_file, hosts_text).expect("Expected Hosts file to be writable");


    println!("Cya!");
    Ok(())
}
