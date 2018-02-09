# Resamplr

This is a rewrite in Elm and Rocket.  To run, install VirtualBox and Vagrant.

## Running

Simply run `vagrant up` and then `vagrant ssh`

To build, run `just dev-build`.  To serve and watch for changes,
instead run `just dev-run`.

## Problems
If you run into a problem where NPM won't install stuff, make sure you use the `--no-bin-links` flag when installing.