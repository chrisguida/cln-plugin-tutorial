---
title: "Background"
sidebar_position: 2
---

# What is a plugin anyway?
- Executable file
- Started by CLN
- Runs as a subprocess of CLN
- Communicates with CLN via the plugin’s stdin and stdout
- Can be written in any language
- Can be launched at CLN startup or dynamically launched
- Extends CLN’s functionality via:
  - RPC methods
  - Event stream notifications
  - Hooks

In this tutorial, we'll look at each of these elements more closely.

## Examples of plugins

### [Built in plugins](https://github.com/ElementsProject/lightning/tree/master/plugins):
- Pay (pays invoices)
- Bcli (connects to a bitcoin backend)
- Keysend (aids in sending and receiving keysend payments)
- Bookkeeper (accounting manager)

### [Official plugins repo](https://github.com/lightningd/plugins)
- Summary (Print a nice summary of the node status)
- Circular (A smart rebalancing plugin for Core Lightning routing nodes)
- Watchtower (Watchtower client for The Eye of Satoshi)
- Sauron (A Bitcoin backend relying on Esplora's API)
- Reckless (An experimental plugin manager)

### Other
- [CLBOSS](https://github.com/ZmnSCPxj/clboss) (Automated node manager)

## Languages and Frameworks
### Python
- [pyln-client](https://github.com/ElementsProject/lightning/tree/master/contrib/pyln-client) by @cdecker

### Rust
- [cln-plugin](https://github.com/ElementsProject/lightning/blob/master/plugins) by @cdecker

### Go
- [glightning](https://github.com/niftynei/glightning) by @niftynei

### C
- [libplugin](https://github.com/ElementsProject/lightning/blob/master/plugins/libplugin.h) by @rustyrussell

### C++ 
- [lightning-cpp](https://github.com/darosior/lightningcpp) by @darosior

### Javascript
- [clightningjs](https://github.com/lightningd/clightningjs) by @darosior

### Typescript
- [core-ln.ts](https://github.com/runcitadel/core-ln.ts) by @AaronDewes

### Java
- [JRPClightning](https://github.com/clightning4j/JRPClightning) by @vincenzopalazzo

### C#
- [DotNetLightning](https://github.com/joemphilips/DotNetLightning) by  @joemphilips

### Kotlin
- [Tutorial/example using JRPClightning Java Framework](https://vincenzopalazzo.medium.com/a-day-in-a-c-lightning-plugin-with-koltin-c8bbd4fa0406) by @vincenzopalazzo
