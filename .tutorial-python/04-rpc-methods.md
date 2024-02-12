# RPC Methods

- You can define custom behavior for any `lightning-cli` method by registering that method with `lightningd` in your `getmanifest` response.
- Whenever `lightning-cli <method> [options]` is called, CLN will pass the method and options back to the plugin, which can handle the call and respond however it wants.
- Can register any number of RPC methods, as long as the method names aren’t already taken. 
- If an RPC method is already taken when the plugin registers the method name, the plugin is killed.

# Exercise: Add an RPC method to your plugin

Add this code to your `helloworld.py` file (hint: make sure you insert this before `plugin.run()`):

```python
@plugin.method("hello")
def hello(plugin, name="world"):
    """This is the documentation string for the hello-function.
    It gets reported as the description when registering the function
    as a method with `lightningd`.
    """
    greeting = "Hello"
    s = '{} {}'.format(greeting, name)
    plugin.log(s)
    return s

```

Now let's test! Restart your plugin:

```sh
l1-cli plugin stop helloworld.py && l1-cli plugin start $(pwd)/python-plugin/helloworld.py
```

Now invoke your new RPC method:

```sh
l1-cli hello
```

Try it with an argument:

```
l1-cli hello <your name>
```

Try causing the rpc call to throw an error:

```
l1-cli hello too many arguments
```

- [ ] Add an RPC method
- [ ] Test that it works as expected
