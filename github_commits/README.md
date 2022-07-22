## About

Simple script to practice a little rust and look up all my commits
in an organization after a date. Helps me remember what to write
about in a performance review.

## Build and run

Running cargo build for development. Run cargo build --release so 
the `commits.sh` can read it.

Update the minimum date inside the `commits.sh` to get latest commits.

```
$ cargo build --release
$ cd ~/Development/mapbox && sh ~/github_commits/commits.sh
```
