# Tutorial for building a Core Lightning (CLN) plugin in replit.com

Hello!

This tutorial will teach you how to build a CLN plugin.

In order to do this tutorial, you need an account on replit.com.

Fork my repl to do the tutorial in Python or Rust:
- Python: https://replit.com/@chrisguida1/Writing-Your-First-CLN-Plugin
- Rust: https://replit.com/@chrisguida1/Writing-Your-First-CLN-Plugin-Rust-version

**make sure to fork it to your own user**

## The below version doesn't work yet, use the replit link above and **make sure to fork it to your own user** or it will not work.
Please register for an account if you don't already have one, and then click [this link](https://replit.com/github/chrisguida/cln-plugin-tutorial) to start the tutorial. Clicking this link will clone this github repo into your replit.com user's account, and then open it.

This repo will not do anything useful if you don't click the link above.

You will need to enter one of the commands below in the shell tab to let the tutorial know whether you would like to build a Python plugin or a Rust plugin.

- If you would like to work in **Python**, rename the `.tutorial-python` directory to `.tutorial`, or create a symlink like so:

```sh
ln -s .tutorial-python/ .tutorial
```

- Similarly, if you would like to work in **Rust**:

```sh
ln -s .tutorial-rust/ .tutorial
```

The `.tutorial` directory is what will be displayed in the Replit Tutorial tab.

If you are getting the message `awaiting bitcoind...` then you need to run these commands

```
$ rm ~/.bitcoin/regtest/bitcoind.pid
$ start_ln
```

Note: If the "Console" tab is open, please close it, as you will not be needing it.

The only things you'll need are:
- the file tree sidebar on your left (click the "Open Sidebar" button in the upper left if you don't see it)
- the Tutorial tab to the right of that
- an editor tab to the right of that, and
- a Shell tab on the right.

Make sure the editor tab is displaying either `python-plugin/helloworld.py` (for Python) or `rust-plugin/src/main.rs` (for Rust). You can find both files in the file tree sidebar.

The screen should look something like this when you are done setting up:

![photo_2024-09-25_17-17-47](https://github.com/user-attachments/assets/74868b43-7bf6-4ba9-bc2a-375918963def)

Happy hacking!
