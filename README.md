# rusty-monitor

Discord bot that serves as a client for https://github.com/magiclen/m-prober, a linux resource monitor written in rust. Capable of writing new users and their servers to a (postgres) db via sea-orm, an object relational model library for rust. Commands are available to get momentary snapshots and to also initiate a monitor that makes frequent checks to the provided server address and DMs the user if any monitors exceed set thresholds. I was motivated to build this after getting frustrated with the steps needed to jump through to get resource info about a remote machine, and figured a discord bot is more helpful than a react app or something. I also wanted to learn rust.

In order to run this the mprober application needs to be running and exposed to the web on the machine you intend to monitor. `mprober web --only-api -a <auth_key>` is the command I run.
