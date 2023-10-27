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

Now, let's use the `serde_json::Value` parameter which allows us to pass arguments.
Create a new method `testmethod_argument` and register it in your main() function.
Change `_v` to `v` as it is no longer an unused parameter.

```rust
.rpcmethod("testmethod_argument", "This is a test", testmethod_argument)
```

```rust
async fn testmethod_argument(p: Plugin<()>, v: serde_json::Value) -> Result<serde_json::Value, Error> {
  Ok(json!(format!("Hello {}", v)))
}
```

Try it with an argument:

```
l1-cli testmethod_argument <your name>
```

The v argument is passed to our RPC method as json.
Parsing and printing just the string (properly formatted, without escape characters) is left as an exercise to the reader.
As a bonus exercise, print an error if more than 1 argument is passed.

- [ ] Add an RPC method
- [ ] Test that it works as expected
