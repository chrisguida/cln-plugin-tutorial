---
title: "Events"
sidebar_position: 6
---

# Event notifications

- Also referred to as "Event stream subscriptions"
- Asynchronous events
- Allow a plugin to subscribe to event topics such that CLN will send the plugin
  a push notification whenever events corresponding to such topics occur.
- For events where it doesn’t matter how many plugins are listening to the event
  or in what order the event is processed.
- Topics include:
  - `channel_opened`
  - `connect`
  - `sendpay_success`
  - `block_added`

For the full list of topics, see
[the official docs](https://docs.corelightning.org/docs/event-notifications).

# Exercise: Add an event stream subscription to your plugin

Add this code to your `main.rs` file (hint: insert this before `.dynamic()`):

```rust
.subscribe("connect", connect_handler)
```

This registers the plugin to listen for `connect` events and respond with a
handler, named `connect_handler`.

Now add the `connect_handler` event notification handler function (this should
go after your rpc handler, function, at the bottom of the file):

```rust
async fn connect_handler(_p: Plugin<()>, v: serde_json::Value) -> Result<(), Error> {
    log::info!("Got a connect notification: {}", v);
    Ok(())
}
```

Now let's test! Go to the shell and rebuild:

```sh
cargo build
```

Now restart your plugin:

```sh
l1-cli plugin stop rust-plugin && 
l1-cli plugin start $(pwd)/target/debug/rust-plugin
```

Grab Node 1’s pubkey and listening address:

```sh
l1-cli getinfo
```

Now paste the connection info for Node 1 into this `connect` command from Node
2:

```sh
l2-cli connect <Node 1’s pubkey>@<hostname>:<port>
```

Now check Node 1's logs to make sure your callback ran.

- [ ] Add an event stream subscription
- [ ] Test that it works as expected
