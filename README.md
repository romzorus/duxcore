# Duxcore : embed an ansible-like automation engine right in your Rust code

<div align="center">
<img src="img/dux.png" width="20%">
</div>

# The goal
Instead of having one big automation tool (meaning configuration management or orchestration tool) trying to handle all scenarios (be scalable, performant, handle local and remote hosts through this protocol or this one, be compliant with this security standard and this one...), we prefer to build one flexible automation *engine* (this crate) and make it as easy as possible to embed in a codebase already adapted to one's specific need.

# Documentation
A [*book*](https://www.dux-automate.org/book/) has been opened about the Dux project. Especially, modules list and documentation can be found [here](https://www.dux-automate.org/book/modules.html).

# Principle

Based on Rust's type system, the workflow is as follows :
1. Get a task list : what is the expected state of the managed hosts ? This step produces a `TaskList` struct.
2. Get a hosts list : which hosts are under the scope of this task list ? This step produces a `HostList` struct.
3. Generate `Assignments` : an `Assignment` represents a host and allows to track what happens to this host. It contains everything needed to handle the host and apply the expected state.
4. Dry run : dry run each `Assignment`. This step produces a `ChangeList` struct which contains what needs to be done on the host to reach the expected state.
5. Apply : actually apply the changes on the host to reach the expected state. This step produces a `ResultList` struct.


# Usage
Import the crate

```shell
cargo add duxcore
```
Now let's perform the usual example : **setup a webserver** (but, this time, right from your Rust code !)
```rust
use duxcore::prelude::*;
use std::path::PathBuf;

fn main() {
    // First we define all required components :
    // --> a 'Host'
    let mut target_host = Host::from_string("host-address".into());
    target_host.add_var("package_name", "apache2");

    // --> connection details
    let ssh2_connection_details = Ssh2ConnectionDetails::from(
        target_host.address.clone(),
        Ssh2AuthMode::KeyFile((
            "username".into(),
            PathBuf::from("/path/to/private/key"),
        )),
    );

    // --> a 'HostHandler' based on a Host and its connection details
    let mut host_handler = HostHandler::from(&HostHandlingInfo::from(
        ConnectionMode::Ssh2,
        target_host.address.clone(),
        ConnectionDetails::Ssh2(ssh2_connection_details),
    ))
    .unwrap();

    // --> a 'TaskList' describing the expected state of this host
    let tasklist_content = "
- name: Install apache web server
  steps:
    - name: Package installation
      with_sudo: true
      apt:
        package: \"{{ package_name }}\"
        state: present

    - name: Start and enable the service
      with_sudo: true
      service:
        name: apache2
        state: started
        enabled: true
    
    - name: Finally, enable some website
      with_sudo: true
      command:
        content: a2ensite /path/to/my/site/configuration/file";

    let task_list: TaskList = TaskList::from_str(
        tasklist_content,
        TaskListFileType::Yaml,
        &target_host) // 'Host' is given to take variables into account
        .unwrap();

    // Then we actually use them :
    // --> SSH2 connection needs to be initialized
    host_handler.init();

    // --> Evaluate what needs to be done on the host to meet the expected state
    let mut change_list: ChangeList = task_list.dry_run_tasklist(&mut host_handler).unwrap();

    // --> Apply the required changes and have the host reach the expected state
    // Won't do anything (not even try to connect) if 'ChangeList' is empty (meaning the host is already in the expected state)
    let result_list: ResultList = change_list.apply_changelist(&mut host_handler);
}
```
This is the basic workflow of Dux. It is up to you to parallelize, distribute the work, display the results in some web interface or send them in a RabbitMQ queue... Whatever suits you best ! The whole point is to let you adapt this automation engine to the context of your already-existing infrastructure. Adapt the tool to the job !

# More examples

More complex examples of how the Dux crate can be used are being built as separate projects. These are **proofs of concept** and can be used as a starting point for your own implementation. You can also start from scratch.

## Standard implementation
> One binary doing everything

Dux standard project : [dux-standard](https://gitlab.com/dux-tool/dux-standard)

<div align="center">
<img src="img/standard-illustration.png" width="60%">
</div>

## Agent implementation
> A Dux agent running as a background service, regularly fetching a remote tasklist (http/https, git...) and applying it to itself

Dux agent project : [dux-agent](https://gitlab.com/dux-tool/dux-agent)

<div align="center">
<img src="img/agent-illustration.png" width="60%">
</div>

## Distributed implementation
> Workload split between a controller which generates Assignments and workers which actually run them on targetted hosts

Dux distributed controller project : [dux-distributed-controller](https://gitlab.com/dux-tool/dux-distributed-controller)  
Dux distributed worker project : [dux-distributed-worker](https://gitlab.com/dux-tool/dux-distributed-worker)

<div align="center">
<img src="img/distributed-illustration.png" width="80%">
</div>

## Scalable implementation
> Workload split between a controller and workers nodes, with a message broker in the middle to allow scaling up and down the number of workers

Dux scalable controller project : [dux-scalable-controller](https://gitlab.com/dux-tool/dux-scalable-controller)  
Dux scalable worker project : [dux-scalable-worker](https://gitlab.com/dux-tool/dux-scalable-worker)

<div align="center">
<img src="img/scalable-illustration.png" width="80%">
</div>


# Contribution
Want some help to use this crate for your own situation ? Open to suggestions, feedback, requests and any contribution !
Will gladly exchange ideas and help you build your own implementation right [there](https://discord.com/invite/2gxAW7uzsx) !
