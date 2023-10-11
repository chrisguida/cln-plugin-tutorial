#!/usr/bin/env python3
from pyln.client import Plugin # We import `Plugin` from the `pyln-client` pip package, which does all the hard work for us

plugin = Plugin() # This is our plugin's handle

@plugin.init() # Decorator to define a callback once the `init` method call has successfully completed
def init(options, configuration, plugin, **kwargs):
    plugin.log("Plugin helloworld.py initialized")

plugin.run() # Run our plugin
