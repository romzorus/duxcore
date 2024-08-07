# Duxcore : embed an ansible-like automation engine right in your Rust code

<div align="center">
<img src="img/dux.png" width="20%">
</div>

# The goal
Instead of having one big automation tool (meaning configuration management or orchestration tool) trying to handle all scenarios (be scalable, performant, handle local and remote hosts through this protocol or this one, be compliant with this security standard and this one...), we prefer to build one flexible automation *engine* (this crate) and make it as easy as possible to embed in a codebase already adapted to one's specific need.

# Documentation
A *book* has been open [here](https://www.dux-automate.org/) about the dux project.

# Examples
So far, 3 versions are being built based on this crate (as proofs of concept):
- [**standard**](https://gitlab.com/dux-tool/dux-standard) : one executable taking a list of tasks and a list of hosts as input (plus extra such as username, key...) and applying these tasks to these *controlled* hosts

<div align="center">
<img src="img/standard-illustration.png" width="60%">
</div>

- **scalable** ([controller node](https://gitlab.com/dux-tool/dux-scalable-controller), [worker node](https://gitlab.com/dux-tool/dux-scalable-worker)): turned into a microservice architecture, the tasks are created by controller nodes, sent to a message broker (RabbitMQ), fetched by worker nodes (these are the one actually doing something on the *controlled* hosts) and the results are sent back to the controllers through the message broker. This allows to multiply the number of *controlled* hosts handled simultaneously by increasing the number of worker nodes

<div align="center">
<img src="img/scalable-illustration.png" width="80%">
</div>

- [**agent**](https://gitlab.com/dux-tool/dux-agent) : one executable running as a service on a host. This service regularly applies a task list on its own host. This task list can be a local file or fetched via multiple methods (https, ftp, git...). It allows a *pull mode* : the *controlled* hosts are actively looking for tasks to apply to themselves. All the operator has to do is to update the task list and the rest will take place on its own.

<div align="center">
<img src="img/agent-illustration.png" width="60%">
</div>


# Contribution / help / discussion
Want some help to use this crate for your own situation ? Open to suggestions, feedback, requests and any contribution !
Will gladly exchange ideas with you right [there](https://discord.com/invite/2gxAW7uzsx) !


# Modules available
*(alphabetized)*
| Module | Description |
| ---      | ---      |
| `apt` | Manage packages on Debian-like distributions |
| `command` | Run a single shell command on the controlled host |
| `dnf` | Manage packages on Fedora-like distributions (no difference with `yum`) |
| `lineinfile` | Manipulate lines in a file (add, delete) |
| `ping` | Test SSH connectivity with remote host |
| `service` | Manage services on the controlled host |
| `yum` | Manage packages on Fedora-like distributions (no difference with `dnf`) |


# Todo list
- [ ] Global : optimization (lots of `clone` out there...)
- [ ] Global : error handling (lots of `unwrap` out there...)
- [ ] HostList: introduce aliases and connection mode
- [ ] HostList: add JSON format handling
- [ ] TaskExec: modules need to produce a standardized JSON result, reusable directly by later steps ('register')
- [ ] RabbitMQ: turn connections parts into a crate
- [ ] RabbitMQ: add resiliency mechanisms (lost connection...etc)
- [ ] Connection: introduce compatibility with [QUIC](https://github.com/quinn-rs/quinn), [SSH3](https://github.com/francoismichel/ssh3), other protocol ?
- [ ] modules to handle Android and IOT devices ?
- [ ] full ansible syntax compatibility
- [ ] log generation : what is applied when on what, syslog interaction, ability to generate JSON content (for log aggregators for example)
- [ ] Create a gRPC based example implementation
