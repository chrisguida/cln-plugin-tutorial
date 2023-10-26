# Plugin Options
- Command line options passthrough allows plugins to register their own command line options that are exposed through lightningd so that only the main process needs to be configured.
- Options can also be passed from the `config` file.
- Option values are not remembered when a plugin is stopped or killed, but can be passed as parameters to `plugin start`.

# Exercise: Add an option to your plugin

Add this code to your `helloworld.py` file (hint: make sure you insert this before `plugin.run()`):

```python
plugin.add_option('greeting', 'Hello', 'The greeting I should use.')
```

Next, edit your `hello` RPC method from the previous step to use your option in that call:

Change the line that says:

```python
greeting = "Hello"
```

To use your option:

```python
greeting = plugin.get_option('greeting')
```

Now let's test! Go to the shell and stop your plugin:

```sh
l1-cli plugin stop helloworld.py
```

Now start your plugin, inserting a value for your new option:

```sh
l1-cli -k plugin subcommand=start plugin=$(pwd)/helloworld.py greeting='A crazy'
```

Now invoke the updated `hello` RPC method:

```sh
l1-cli hello
```

Try it with various arguments, and see what happens.

- [ ] Add an option
- [ ] Test that it works as expected
