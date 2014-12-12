// News Junkie
// ---
// Features:
// a) Aggressively block unproductive websites 
// b) Load a server with a reminder message about not wasting 
//    time on these sites.
// ---
// By Dan McGrady (dmix) 2014

use std::io::BufferedReader;
use std::io::File;

static HOSTS_PATH: &'static str  = "/etc/hosts";
static CONF_PATH: &'static str  = "/home/snake/.config/news_junkie.conf";

fn main() {
    let hosts: Vec<String> = parse_hosts();
    let conf: Vec<String> = parse_conf();
    let blacklist: Vec<uint> = parse_blacklist(hosts);
    update_blacklist(hosts, conf, blacklist);
}

fn update_blacklist(hosts: Vec<String>, 
                    conf: Vec<String>, 
                    blacklist: Vec<uint>) {
    // Append blacklisted hosts not currently activated
    println!("{}", hosts);
    println!("{}", conf);
    println!("{}", blacklist);
}

fn parse_conf() -> Vec<String> {
    let path = Path::new(CONF_PATH);
    let mut file = BufferedReader::new(File::open(&path));
    let mut conf: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
    return conf;
}

fn parse_hosts() -> Vec<String> {
    let path = Path::new(HOSTS_PATH);
    let mut file = BufferedReader::new(File::open(&path));
    let hosts: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
    return hosts;
}

fn parse_blacklist(hosts: Vec<String>) -> Vec<uint> {
    let mut blacklist = Vec::new();
    let mut list_begin = false;

    for i in range(0u, hosts.len()) {
        if list_begin {
            blacklist.push(i);
        }
        if hosts[i].contains("BEGIN news_junkie") {
            list_begin = true;
        }
        if hosts[i].contains("END news_junkie") {
            break;
        }
    }
    return blacklist;
}
