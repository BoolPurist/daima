
# Problem

Want to manage/have timer, clocks, stopwatchs and countdowns in a terminal.

## Solution

### Name of application 

daima


### Description

Provide a TUI interface to manage those time entities. 
This TUI interface should provide vim bindings

It also should provide a CLI interface for automation purpose.


### Terms

CLI and TUI processes are the client.

Unix socket server is the Daima Manager 
This program manages sessions and keeps the all the clients up to date. 

Features

- Sessions. Distinct collections of time entities.
- TUI and CLI should be able to choose on which session they operate.
- Allow CLI interaction in parallel to TUI interface. 
- Changes caused by a CLI commands should be visible to TUI interfaces.
- TUI within the same session should be synchronized in regard to the state of session.
- Possible to create several TUI interfaces synchronized in regard to the session states.
- TUI interface must be configurable in keybindings. Vim binding experience should be the default.
- Timer, Alarms etc should be noticable via visual indication inside clients, optional tray notification 
  and optional sound notification

### Access control

Socker file of socket server should only be accessible by the Cli,  tui and the server itself
Files saved by server should only accesible by the server 

### Used libraries

- Chrono or time for handling time/dates
- RMP serde for the binary messages as MessagePackage exchanged between the clients and the server
- UDS unix domain sockets for IPC
- TUI interactio (ratatui and crossterm)
- Clap for cli parsing
   


