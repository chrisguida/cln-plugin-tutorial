# Event notifications

- Also referred to as "Event stream subscriptions"
- Asynchronous events
- Allow a plugin to subscribe to event topics such that CLN will send the plugin a push notification whenever events corresponding to such topics occur.
- For events where it doesn’t matter how many plugins are listening to the event or in what order the event is processed.
- Topics include:
  - `channel_opened`
  - `connect`
  - `sendpay_success`
  - `block_added`

For the full list of topics, see [the official docs](https://docs.corelightning.org/docs/event-notifications).

# Exercise: Add an event stream subscription to your plugin

Add this code to your `helloworld.py` file (hint: make sure you insert this before `plugin.run()`):

```python
@plugin.subscribe("connect")
def on_connect(plugin, id, address, **kwargs):
    plugin.log("Received connect event for peer {}".format(id))


@plugin.subscribe("disconnect")
def on_disconnect(plugin, id, **kwargs):
    plugin.log("Received disconnect event for peer {}".format(id))
```

Now let's test! Go to the shell and restart your plugin:

```sh
l1-cli plugin stop helloworld.py && 
l1-cli plugin start $(pwd)/python-plugin/helloworld.py
```

Grab Node 1’s pubkey and listening address:

```sh
l1-cli getinfo
```

Now paste the connection info for Node 1 into this `connect` command from Node 2:

```sh
l2-cli connect <Node 1’s pubkey>@<hostname>:<port>
```

Now check Node 1's logs to make sure your callback ran.

Try the same thing with your `disconnect` subscription:

```sh
l2-cli disconnect <Node 1’s pubkey>
```

- [ ] Add an event stream subscription
- [ ] Test that it works as expected
