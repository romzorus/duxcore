use crate::host::hosts::Host;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct HostList {
    pub hosts: Option<Vec<Host>>,
}

pub fn hostlist_get_all_hosts(hostlist: &HostList) -> Option<Vec<String>> {
    match &hostlist.hosts {
        Some(host_list) => {
            let mut all_hosts_addresses: Vec<String> = Vec::new();
            for host in host_list {
                all_hosts_addresses.push(host.address.clone());
            }
            Some(all_hosts_addresses)
        }
        None => None,
    }
}

pub fn hostlist_get_from_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap() // Placeholder : error handling required here
}

pub fn hostlist_get_from_interactive_mode() -> String {
    // Placeholder : we might want a mode where the TaskList is already set and we can add
    // manually / pipe from some other source in real time some more hosts to run the TaskList
    // on and, as soon as the hosts are entered, the TaskList is run on them. Interest ?
    String::new()
}

// If the host is already in the list, the index is returned. Otherwise, None is returned.
pub fn find_host_in_list(hosts_list: &Vec<Host>, host_name: &String) -> Option<usize> {
    for (index, host) in hosts_list.iter().enumerate() {
        if host.address.eq(host_name) {
            return Some(index);
        }
    }
    None
}
