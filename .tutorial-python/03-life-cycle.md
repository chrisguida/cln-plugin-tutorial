# Plugin Life Cycle

## 1) Plugin launches
- Plugin is started by CLN in one of two ways: at CLN startup, or dynamically once CLN is already running.
  - At startup:
    - Via a command line option: `lightningd --plugin=/path/to/plugin1 --plugin=path/to/plugin2`
    - Via an option in the config file: `plugin=/path/to/plugin`
  - Dynamically:
    - Via `lightning-cli plugin start /path/to/plugin [options]`

## 2) The `getmanifest` method is called
- CLN calls plugin’s `getmanifest` JSON-RPC method
- Plugin responds with a manifest where everything the plugin needs is registered with CLN, including options, hooks, rpc methods, and notifications.
- During startup, a plugin must respond within 60 seconds or it is killed.

The plugin's response will look something like this:
```json
{
  "options": [
    {
      "name": "greeting",
      "type": "string",
      "default": "World",
      "description": "What name should I call you?",
      "deprecated": false
    }
  ],
  "rpcmethods": [
    {
      "name": "hello",
      "usage": "[name]",
      "description": "Returns a personalized greeting for {greeting} (set via options)."
    },
    {
      "name": "gettime",
      "usage": "",
      "description": "Returns the current time in {timezone}",
      "long_description": "Returns the current time in the timezone that is given as the only parameter.\nThis description may be quite long and is allowed to span multiple lines.",
      "deprecated": false
    }
  ],
  "subscriptions": [
    "connect",
    "disconnect"
  ],
  "hooks": [
    { "name": "openchannel", "before": ["another_plugin"] },
    { "name": "htlc_accepted" }
  ],
  "featurebits": {
    "node": "D0000000",
    "channel": "D0000000",
    "init": "0E000000",
    "invoice": "00AD0000"
  },
  "notifications": [
    {
	  "method": "mycustomnotification"
	}
  ],
  "nonnumericids": true,
  "dynamic": true
}
```

## 3) The `init` method is called
- CLN parses plugin options and passes them back by calling the plugin’s `init` method. This is also the signal that `lightningd`’s JSON-RPC over Unix Socket is now up and ready to receive incoming requests from the plugin.
- The request will look something like this:
```json
{
  "options": {
    "greeting": "World",
	"number": [0]
  },
  "configuration": {
    "lightning-dir": "/home/user/.lightning/testnet",
    "rpc-file": "lightning-rpc",
    "startup": true,
    "network": "testnet",
    "feature_set": {
        "init": "02aaa2",
        "node": "8000000002aaa2",
        "channel": "",
        "invoice": "028200"
    },
    "proxy": {
        "type": "ipv4",
        "address": "127.0.0.1",
        "port": 9050
    },
    "torv3-enabled": true,
    "always_use_proxy": false
  }
}
```
- The plugin sends a success response.
  - The plugin can optionally send a response at this point indicating that it’s disabled.
- This response must occur within a certain timeout:
  - During startup, a plugin must respond within 60 seconds or it is killed.
  - Plugins launched dynamically must respond to both getmanifest and init within 60 seconds or they are killed.

## 4) The plugin is now running normally
Now your plugin can do stuff!

# Exercise: Build a bare minimum plugin that conforms to the above requirements
If you click on [`python-plugin/helloworld.py`](/home/runner/$REPL_SLUG/python-plugin/helloworld.py) in the file tree to your left, you will see a basic CLN plugin written in Python. It doesn't do much, yet. It pretty much just runs. We'll add more to it as we work through this tutorial.

Here it is:
```py
#!/usr/bin/env python3
from pyln.client import Plugin # We import `Plugin` from the `pyln-client` pip package, which does all the hard work for us

plugin = Plugin() # This is our plugin's handle

@plugin.init() # Decorator to define a callback once the `init` method call has successfully completed
def init(options, configuration, plugin, **kwargs):
    plugin.log("Plugin helloworld.py initialized")

plugin.run() # Run our plugin
```

To test, open a shell and enter the following commands:

```
mkdir -p ~/.bitcoin
source startup_regtest.sh
start_ln
```

 These three commands will launch `bitcoind` in regtest mode and attach two LN nodes to it.

 **Remember these three commands.** If your Repl freezes or if you need to leave your Repl and come back, you will need to enter these three commands into the shell again to get everything running again.

 You may see an error message like this:

 ```
error code: -4
error message:
Wallet file verification failed. SQLiteDatabase: Unable to obtain an exclusive lock on the database, is it being used by another instance of Bitcoin Core?
```

This is normal; you may safely ignore it.


Now, let's verify that everything is working:

```
l1-cli getinfo
```

You should see an output like this:
```json
{
   "id": "03dd398e70215a7bdecfcd62bf367d8c0deef7dd8e5241092790d15a23e5506b12",
   "alias": "SILENTFELONY",
   "color": "03dd39",
   "num_peers": 0,
   "num_pending_channels": 0,
   "num_active_channels": 0,
   "num_inactive_channels": 0,
   "address": [],
   "binding": [
      {
         "type": "ipv4",
         "address": "127.0.0.1",
         "port": 7171
      }
   ],
   "version": "v0.12.1",
   "blockheight": 1,
   "network": "regtest",
   "fees_collected_msat": 0,
   "lightning-dir": "/tmp/l1-regtest/regtest",
   "our_features": {
      "init": "08a000080269a2",
      "node": "88a000080269a2",
      "channel": "",
      "invoice": "02000000024100"
   }
}
```

Hooray! We have a Lightning node with a cool alias. You can enter the same commands on Node 2 by entering `l2` instead of `l1`.

Now start your plugin:

```
l1-cli plugin start $(pwd)/python-plugin/helloworld.py
```
This will launch your plugin on Node #1. You should see a list of running plugins with your rust plugin as the last entry in the list.

Now do:
```
l1-log
```
This will display Node #1's logfile. If you scroll to the bottom of the pager (shift + `G`), you should see a message from plugin-manager saying your plugin was started. Presing `g` will bring you back to the top of the logfile.

Press `q` to return to the shell.

- [ ] Launch a Python plugin with a message on init success, and find success message in the log.