#[macro_use]
extern crate fstrings;

mod hosts;
mod download;

use std::fs;

fn main() -> Result<(), ()> {

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
                    .get_matches();


    let urls: Vec<String> = matches.get_many::<String>("urls").unwrap().map(|v| v.to_owned()).collect();
    let out_file = matches.get_one::<String>("out").unwrap();
    let compression_level = matches.get_one::<String>("compression").unwrap().parse::<u16>().unwrap();

    let mut all_hosts = String::new();

    for url in urls.iter() {
        println!("Downloading {}", url);
        let text = download::download_text(url).expect("Error failed to download");
        all_hosts.push_str(text.as_str());
    }

    let hosts_hash = hosts::hosts_to_hashmap(all_hosts.as_str());
    
    let hosts_text = hosts::hashmap_to_hosts(&hosts_hash, compression_level);

    fs::write(out_file, hosts_text).expect("Expected Hosts file to be writable");


    println!("Hello, world!");
    Ok(())
}
