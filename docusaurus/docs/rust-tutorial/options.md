---
title: "Plugin Options"
sidebar_position: 5
---

# Plugin Options

- Command line options passthrough allows plugins to register their own command
  line options that are exposed through lightningd so that only the main process
  needs to be configured.
- Options can also be passed from the `config` file.
- Option values are not remembered when a plugin is stopped or killed, but can
  be passed as parameters to `plugin start`.

# Exercise: Add an option to your plugin

Add this code to your `main.rs` file (hint: insert this before `.dynamic()`):

```rust
.option(options::ConfigOption::new(
    "name",
    options::Value::String("World".to_string()),
    "Recipient of the greeting. Defaults to 'World'",
))
```

Next, add a new method `testmethod_option` RPC method to use your option in that call.

```rust
.rpcmethod("testmethod_option", "This is a test", testmethod_option)
```

```rust
async fn testmethod_option(p: Plugin<()>, _v: serde_json::Value) -> Result<serde_json::Value, Error> {
    Ok(json!(format!("Hello, {:?}", p.option("name").unwrap())))
}
```

Note we renamed `_p` to `p` because we are actually using the plugin state,
which was previously ignored. The plugin state `p` contains our options.

Now let's test!

Rebuild your plugin:

```sh
cargo build
```

Now go to the shell and stop your plugin:

```sh
l1-cli plugin stop rust-plugin
```

Now start your plugin, inserting a value for your new option:

```sh
l1-cli -k plugin subcommand=start plugin=$(pwd)/target/debug/rust-plugin name='<insert name here>'
```

Now invoke the new `testmethod_option` RPC method:

```sh
l1-cli testmethod_option
```

Try it with various arguments, and see what happens.

- [ ] Add an option
- [ ] Test that it works as expected
