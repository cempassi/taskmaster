# Taskmaster

Taskmaster is a process manager written in `rust`. It's a school project.

## Table of contents

- [Taskmaster](#taskmaster)
  - [Table of contents](#table-of-contents)
  - [General info](#general-info)
    - [How to configure Taskmaster](#how-to-configure-taskmaster)
  - [Technologies](#technologies)
  - [Setup](#setup)
  - [Guides and documentation](#guides-and-documentation)
  - [Terms](#terms)
  - [Team](#team)

## General info

Taskmaster is a process control system based on [Supervisord](http://supervisord.org/)

It's a school project made to implement a unix process manager, with the
following capabilites:

1. Launch and monitor several **Tasks**
1. Define **tasks** behaviour from a configuration file (See below)
1. Reload the configuration and update the **tasks** accordingly
1. Log the events
1. A client/server architecture

### How to configure Taskmaster

The following configurations are expected for each **task**:

- Name of the **task**
- Command to run
- Number of **process** to spawn
- Startup launch behaviour
- Relaunch strategy:
  1. Always
  1. Never
  1. When an unexpected error occured
- Expected exit status
- How long the **process** must have been running to be considered successfull
- Number of tries a restart should be attempted before aborting
- Signal to exit gracefully
- Wait delay after a graceful stop (If the delay is exceded, a `SIGKILL` is sent)
- IO redirections, stdout & stderr for the time beeing
- Environment variables
- Working directory
- Umask

The configuration is expected to be written in `toml`

```toml
[[task]]
name = "write bar"      # name of the TASK
cmd = "echo bar"        # the command to execute, shell-like
autostart = true        # boolean, the TASK start with taskmaster
numprocess = 42         # uint, number to PROCESSES the TASK have to run
umask = 0777            # uint, set umask aka default permission on created file from process
workingdir = "/tmp"     # working directory of the PROCESS
stdout = "/tmp/foo.out" # redirect STDOUT to <file>
stderr = "/tmp/foo.err" # redirect STDERR to <file>
stopsignal = "TERM"     # signal to send to stop the running PROCESS
```

## Technologies

This project is fully written in `Rust`

## Setup

To run this project, clone the repository and build it with `Cargo`.

## Guides and documentation

- [Rust book](https://doc.rust-lang.org/book/)
- [Supervisord](http://supervisord.org/)

## Terms

| Term                | Description                                                        |
| ------------------- | ------------------------------------------------------------------ |
| Task / Tasks        | A task is refere to a configuration for running some **processes** |
| Process / Processes | A process is a unix process                                        |
| IO                  | refere to stdin, stdout and stderr                                 |

## Team

- [apsaint](https://github.com/apsaint)
- [cempassi](https://github.com/cempassi)
