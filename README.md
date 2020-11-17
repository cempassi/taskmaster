# Taskmaster

Taskmaster is a process manager written in rust. It's a school project.

## Table of contents

- [General info](#general-info)
- [Technologies](#technologies)
- [Guides and documentation](#guides-and-documentation)
- [Setup](#setup)
- [Team](#team)

## General info

Taskmaster is a process control system based on [Supervisord](http://supervisord.org/)

It's a school project made to implement a unix process manager, with the
following capabilites:

1. Launch and monitor several "Jobs"
1. Define jobs behaviour from a configuration file (See below)
1. Reload the configuration and update the jobs accordingly
1. Log the events
1. A client/server architecture

The following configurations are expected for each job:

- Command to run
- Number of job to spawn
- Startup launch behaviour
- Relaunch stratagy:
  1. Always
  1. Never
  1. When an unexpected error occured
- How long the job must have been running to be considered successfull
- Number of tries
- Unexpected errors
- Signal to exit gracefully
- Wait elay after a graceful stop (If the delay is exceded, a SIGKILL is sent)
- Stdout and stderr redirections
- Environment variables
- Working directory
- Umask

## Technologies

This project is fully written in Rust

## Guides and documentation

- [Rust book](https://doc.rust-lang.org/book/)

## Setup

To run this project, clone the repository and build it with Cargo.

## Team

- [apsaint](https://github.com/apsaint)
- [cempassi](https://github.com/cempassi)
