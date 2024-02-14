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
  - Rust: Holdinvoice, Smaug

## What's in this Repl?
- `bitcoind`: you'll use this as your blockchain backend in regtest mode to test your plugin
- `lightningd`: a full, functional install of CLN suitable for testing your plugin
