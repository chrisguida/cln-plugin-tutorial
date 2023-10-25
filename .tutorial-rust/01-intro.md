# Writing Your First CLN Plugin

Speaker: Chris Guida<br>
Twitter: @cguida6

Welcome! In this tutorial, you'll write your own CLN plugin! To help you get started as fast as possible, we've put together a repl.it environment with Bitcoin Core (`bitcoind`) and Core Lightning (`lightningd`) already installed. So all you need to do is read through each section and follow the examples, and at the end you'll have a working CLN plugin.

## Session Agenda:
- Build a minimal rust plugin
  - An init method
  - An RPC method
  - An option
  - An event subscription
  - A hook
- Then we'll look at more complex plugins
  - Python: Sauron, Summary
  - Go: Peerswap, Circular
  - Rust: Watchtower, Hodlvoice

## What's in this Repl?
- `bitcoind`: you'll use this as your blockchain backend in regtest mode to test your plugin
- `lightningd`: a full, functional install of CLN suitable for testing your plugin
- `lightning`: clone of [the official CLN repo](https://github.com/ElementsProject/lightning)
- `plugins`: clone of [the official CLN plugins repo](https://github.com/lightningd/plugins)

## Chris's Hackathon Plugin Ideas He Could Help You With:

1. (Easy) Implement an RPC method as a CLI tool that reports various stats about your node.
2. (Medium) Write a telegram bot that communicates with your node
3. (Hard) Add support for trampoline routing between liquid and normal lightning network.
