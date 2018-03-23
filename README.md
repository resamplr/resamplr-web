# [Resamplr](https://resamplr.com)

Resamplr is a virtual instruments and sampler community + store.  [Resamplr](https://resamplr.com) is currently running Ruby on Rails.  This is an effort to port the site to a more managable language.

## Why rewrite in Rust?
RoR is great - but the magic can be too much.  Resamplr outgrew Rails' approaches, and maintaining the original Ruby codebase became much too cumbersome.  

Resamplr's new stack also uses Elm, a reactive and functional front-end language with static typing and no runtime errors.

## Why open source?
The original code is *not* open-sourced or "free".  However, because Rust is still nascent when it comes to web, we thought it could be a valuable learning tool for the community.   

## Running
Make sure you have [Vagrant](https://www.vagrantup.com/) and VirtualBox installed.

Simply run `vagrant up` and then `vagrant ssh`.  Navigate to the working directory at `/vagrant/`.

> Note: You'll also need to run `cargo install diesel_cli --no-default-features --features "postgres"` as the Vagrant box accidentally omits it.  This should be fixed in future versions.

To build, run `just dev-build`.  To serve and watch for changes,
instead run `just dev-run`.

### Running without Vagrant
Because I use a Windows system, I prefer using Vagrant.  However, if you don't want to use Vagrant and would rather install it yourself, you'll need a few things:

- Postgresql and it's development libraries
- Diesel CLI tools
- Rust Nightly
- NPM / Node (unfortunately)

## Known problems
- If you run into a problem where NPM won't install stuff, make sure you use the `--no-bin-links` flag when installing on a Windows host.

## License 
This software is currently unlicensed (Meaning we haven't yet come up with something sensible).  Resamplr is a commercial project, and will require some scrutiny when selecting a license.

In the meantime, you can follow these "don't be a jerk" rules:

- **Don't** clone this codebase and create a competing project
- **Don't** sell this code to others or mislead others in any way
- **Do** feel free to learn and steal snippets or ideas
- **Do** feel free to submit pull requests and otherwise contribute
