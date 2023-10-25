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
</br>
</br>

### Rust
- [cln-plugin](https://github.com/ElementsProject/lightning/blob/master/plugins) by @cdecker
</br>
</br>

### Go
- [glightning](https://github.com/niftynei/glightning) by @niftynei
</br>
</br>

### C
- [libplugin](https://github.com/ElementsProject/lightning/blob/master/plugins/libplugin.h) by @rustyrussell
</br>
</br>

### C++ 
- [lightning-cpp](https://github.com/darosior/lightningcpp) by @darosior
</br>
</br>

### Javascript
- [clightningjs](https://github.com/lightningd/clightningjs) by @darosior
</br>
</br>

### Typescript
- [core-ln.ts](https://github.com/runcitadel/core-ln.ts) by @AaronDewes
</br>
</br>

### Java
- [JRPClightning](https://github.com/clightning4j/JRPClightning) by @vincenzopalazzo
</br>
</br>

### C#
- [DotNetLightning](https://github.com/joemphilips/DotNetLightning) by  @joemphilips
</br>
</br>

### Kotlin
- [Tutorial/example using JRPClightning Java Framework](https://vincenzopalazzo.medium.com/a-day-in-a-c-lightning-plugin-with-koltin-c8bbd4fa0406) by @vincenzopalazzo
</br>
</br>
