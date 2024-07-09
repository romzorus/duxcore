# Duxcore : embed an ansible-like automation engine right in your Rust code

<div align="center">
<img src="img/dux.png" width="20%">
</div>

This crate gathers the core logic and types of the [Dux](https://gitlab.com/dux-tool/dux) tools. So far, 3 versions are being built based on this crate :
- **standard** : one executable taking a list of tasks and a list of hosts as input (plus extra such as username, key...) and applying these tasks to these *controlled* hosts
- **scalable** : turned into a microservice architecture, the tasks are created by controller nodes, sent to a message broker (RabbitMQ), fetched by worker nodes (these are the one actually doing something on the *controlled* hosts) and the results are sent back to the controllers through the message broker. This allows to multiply the number of *controlled* hosts handled simultaneously by increasing the number of worker nodes
- **agent** : one executable running as a service on a host. This service regularly applies a task list on its own host. This task list can be a local or a remote file (https, ftp, git...). It allows a *pull mode* : the *controlled* hosts are actively looking for tasks to apply to themselves. All the operator has to do is to update the task list and the rest will take place on its own.

Check [here](https://gitlab.com/dux-tool/dux) to try these versions.

You can also use this crate to create your own automation tool, adapted to your very specific use case.
