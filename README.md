# Duxcore : embed an ansible-like automation engine right in your Rust code

<div align="center">
<img src="img/dux.png" width="20%">
</div>

# The goal
Instead of having one big automation tool (meaning configuration management or orchestration tool) trying to handle all scenarios (be scalable, performant, handle local and remote hosts through this protocol or this one, be compliant with this security standard and this one...), we prefer to build one flexible automation *engine* (this crate) and make it as easy as possible to embed in a codebase already adapted to one's specific need.

# Examples
So far, 3 versions are being built based on this crate (as proofs of concept):
- [**standard**](https://gitlab.com/dux-tool/dux-standard) : one executable taking a list of tasks and a list of hosts as input (plus extra such as username, key...) and applying these tasks to these *controlled* hosts
- **scalable** ([controller node](https://gitlab.com/dux-tool/dux-scalable-controller), [worker node](https://gitlab.com/dux-tool/dux-scalable-worker)): turned into a microservice architecture, the tasks are created by controller nodes, sent to a message broker (RabbitMQ), fetched by worker nodes (these are the one actually doing something on the *controlled* hosts) and the results are sent back to the controllers through the message broker. This allows to multiply the number of *controlled* hosts handled simultaneously by increasing the number of worker nodes
- [**agent**](https://gitlab.com/dux-tool/dux-agent) : one executable running as a service on a host. This service regularly applies a task list on its own host. This task list can be a local file or fetched via multiple methods (https, ftp, git...). It allows a *pull mode* : the *controlled* hosts are actively looking for tasks to apply to themselves. All the operator has to do is to update the task list and the rest will take place on its own.

# Contribution / help / discussion
Want some help to use this crate for your own situation ? Open to suggestions, feedback, requests and any contribution !
Will gladly exchange ideas with you right [there](https://discord.com/invite/2gxAW7uzsx) !
