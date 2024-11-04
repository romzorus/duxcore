//! # Duxcore
//! Duxcore is an ansible-like automation engine turned into a Rust crate. As you would traditionally write YAML playbooks, roles, inventories then pass all of these as arguments to a single binary (or a Python script), Dux allows you to handle your "automation flows" right from your Rust code !
//!
//! What's the point ? You can build the exact automation tool you need and fully benefit from Rust's type system, performances and ecosystem. Send your taskslists or the results through gRPC with [Tonic](https://crates.io/crates/tonic), handle your hosts in parallel with [Rayon](https://crates.io/crates/rayon), build a remote-controlled automation handler with an API built upon [Axum](https://crates.io/crates/axum) and reach it with [Reqwest](https://crates.io/crates/reqwest)... The ability to integrate your hosts, tasklists and results right in your code like regular Rust objects is what allows you to really adapt your automation tool to your situation.
//!
//! A [*book*](https://www.dux-automate.org/book/) has been opened about the Dux project. Especially, modules list and documentation can be found [here](https://www.dux-automate.org/book/modules.html).
//!
//! # Most basic example : install a web server
//! ```rust
//!use duxcore::prelude::*;
//!
//!fn main() {
//!
//!    // First we need to define what the expected state of the target host is.
//!    let my_tasklist = "---
//!- name: Let's install a web server !
//!  steps:
//!    - name: First, we test the connectivity and authentication with the host.
//!      ping:
//!      
//!    - name: Then we can install the package...
//!      with_sudo: true
//!      apt:
//!        package: '{{ package_name }}'
//!        state: present
//!        
//!    - name: ... and start & enable the service.
//!      with_sudo: true
//!      service:
//!        name: '{{ service_name }}'
//!        state: started
//!        enabled: true
//!        ";
//!
//!    // Then we create a 'Job'.
//!    let mut my_job = Job::new();
//!
//!    // We set who the target host of this Job is, and how to connect to it.
//!    my_job
//!        .set_address("10.20.0.203").unwrap()
//!        .set_connection(HostConnectionInfo::ssh2_with_key_file("dux", "controller_key")).unwrap();
//!    
//!    // We give it some context and the task list.
//!    my_job
//!        .set_var("package_name", "apache2")
//!        .set_var("service_name", "apache2")
//!        .set_tasklist_from_str(my_tasklist, TaskListFileType::Yaml).unwrap()
//!    ;
//!    // We can finally apply the task list to this host.
//!    my_job.apply();
//!
//!    // Let's see the result.
//!    println!("{}", my_job.display_pretty());
//!}
//! ```

pub mod connection;
pub mod error;
pub mod exitcode;
pub mod host;
pub mod job;
pub mod modules;
pub mod output;
pub mod prelude;
pub mod result;
pub mod step;
pub mod task;
pub mod workflow;
