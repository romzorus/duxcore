//! # Duxcore
//! Duxcore is an ansible-like automation engine turned into a Rust crate. As you would traditionally write YAML playbooks, roles, inventories then pass all of these as arguments to a single binary (or a Python script), Dux allows you to handle your "automation flows" right from your Rust code !
//! 
//! What's the point ? You can build the exact automation tool you need and fully benefit from Rust's type system, performances and ecosystem. Send your taskslists or the results through gRPC with [Tonic](https://crates.io/crates/tonic), handle your hosts in parallel with [Rayon](https://crates.io/crates/rayon), build a remote-controlled automation handler with an API built upon [Axum](https://crates.io/crates/axum) and reach it with [Reqwest](https://crates.io/crates/reqwest)... The ability to integrate your hosts, tasklists and results right in your code like regular Rust objects is what allows you to really adapt your automation tool to your situation.
//! 
//! A [*book*](https://www.dux-automate.org/book/) has been opened about the Dux project. Especially, modules list and documentation can be found [here](https://www.dux-automate.org/book/modules.html).
//!
//! # Principle
//!
//! Based on Rust's type system, the workflow is as follows :
//! 1. Get a task list : what is the expected state of the managed hosts ? This step produces a `TaskList` struct.
//! 2. Get a hosts list : which hosts are under the scope of this task list ? This step produces a `HostList` struct.
//! 3. Generate `Assignments` : an `Assignment` represents a host and allows to track what happens to this host. It contains everything needed to handle the host and apply the expected state.
//! 4. Dry run : dry run each `Assignment`. This step produces a `ChangeList` struct which contains what needs to be done on the host to reach the expected state.
//! 5. Apply : actually apply the changes on the host to reach the expected state. This step produces a `ResultList` struct.
//!
//!
//! # Usage
//! Import the crate
//!
//! ```shell
//! cargo add duxcore
//! ```
//! Now let's perform the usual example : **setup a webserver** (but, this time, right from your Rust code !)
//! ```rust
//! use duxcore::prelude::*;
//! use std::path::PathBuf;
//!
//! fn main() {
//!     // First we define all required components :
//!     // --> a 'Host'
//!     let mut target_host = Host::from_string("host-address".into());
//!     target_host.add_var("package_name", "apache2");
//!
//!     // --> connection details
//!     let ssh2_connection_details = Ssh2ConnectionDetails::from(
//!         target_host.address.clone(),
//!         Ssh2AuthMode::KeyFile((
//!             "username".into(),
//!             PathBuf::from("/path/to/private/key"),
//!         )),
//!     );
//!
//!     // --> a 'HostHandler' based on a Host and its connection details
//!     let mut host_handler = HostHandler::from(&HostHandlingInfo::from(
//!         ConnectionMode::Ssh2,
//!         target_host.address.clone(),
//!         ConnectionDetails::Ssh2(ssh2_connection_details),
//!     ))
//!     .unwrap();
//! 
//!     // --> a 'TaskList' describing the expected state of this host
//!     let tasklist_content = "
//! - name: Install apache web server
//!   steps:
//!     - name: Package installation
//!       with_sudo: true
//!       apt:
//!         package: \"{{ package_name }}\"
//!         state: present
//! 
//!     - name: Start and enable the service
//!       with_sudo: true
//!       service:
//!         name: apache2
//!         state: started
//!         enabled: true
//! 
//!     - name: Finally, enable some website
//!       with_sudo: true
//!       command:
//!         content: a2ensite /path/to/my/site/configuration/file";
//! 
//!     let task_list: TaskList = TaskList::from_str(
//!         tasklist_content,
//!         TaskListFileType::Yaml,
//!         &target_host) // 'Host' is given to take variables into account
//!         .unwrap();
//! 
//!     // Then we actually use them :
//!     // --> SSH2 connection needs to be initialized
//!     host_handler.init();
//! 
//!     // --> Evaluate what needs to be done on the host to meet the expected state
//!     let mut change_list: ChangeList = task_list.dry_run_tasklist(&mut host_handler).unwrap();
//! 
//!     // --> Apply the required changes and have the host reach the expected state
//!     // Won't do anything (not even try to connect) if 'ChangeList' is empty (meaning the host is already in the expected state)
//!     let result_list: ResultList = change_list.apply_changelist(&mut host_handler);
//! }
//! ```

pub mod assignment;
pub mod step;
pub mod connection;
pub mod error;
pub mod exitcode;
pub mod host;
pub mod modules;
pub mod output;
pub mod prelude;
pub mod result;
pub mod task;
pub mod workflow;
