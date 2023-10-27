# RPC Methods

- You can define custom behavior for any `lightning-cli` method by registering
  that method with `lightningd` in your `getmanifest` response.
- Whenever `lightning-cli <method> [options]` is called, CLN will pass the
  method and options back to the plugin, which can handle the call and respond
  however it wants.
- Can register any number of RPC methods, as long as the method names aren’t
  already taken.
- If an RPC method is already taken when the plugin registers the method name,
  the plugin is killed.

# Exercise: Add an RPC method to your plugin

Add this code to your `rust-plugin/src/main.rs` file (hint: make sure you insert
this before `.dynamic()`):

```rust
.rpcmethod("testmethod", "This is a test", testmethod)
```

This registers the `testmethod` rpc method with a handler, also named
`testmethod`.

Now add the `testmethod` rpc handler function (this should go after your main
function, at the bottom of the file):

```rust
async fn testmethod(_p: Plugin<()>, _v: serde_json::Value) -> Result<serde_json::Value, Error> {
    Ok(json!("Hello"))
}
```

Now let's test!

Rebuild your plugin:

```sh
cargo build
```

Then restart your plugin:

```sh
l1-cli plugin stop rust-plugin && l1-cli plugin start $(pwd)/target/debug/rust-plugin
```

Now invoke your new RPC method:

```sh
l1-cli testmethod
```

Now, lets use the `serde_json::value` parameter which allows us to pass arguments.
Change `_v` to `v` as it is no longer an unused parameter.

```rust
async fn testmethod(p: Plugin<()>, v: serde_json::Value) -> Result<serde_json::Value, Error> {
  Ok(json!(format!("Hello {}", v)))
}
```

Try it with an argument:

```
l1-cli testmethod <your name>
```

- [ ] Add an RPC method
- [ ] Test that it works as expected
