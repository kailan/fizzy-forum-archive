# blade-app my-social data crawler

Exports an archive of forums. Designed to archive `my.fizzyliving.com` forums before shutdown.

## Usage

Use [`rustup`](https://rustup.rs/) to install Rust and cargo.

Then clone the repo:

```
$ git clone git@github.com:kailan/fizzy-forum-archive.git
$ cd fizzy-forum-archive
```

Replace the `COOKIE` placeholder in `crawler/src/main.rs` with your `Cookie` header from `my.fizzyliving.com`.

Then run the script to generate an archive of posts in the `out` folder:

```
$ cd crawler
$ cargo run
Fetching forum index...
Fetching topic index for forum 'Fizzy Cooks!' (ab5420e1-7f2b-4528-a312-fa5981ec9915)
Fetching post index for topic deff0351-deda-432f-a7ca-7d56ba88e548
...
```

You can then run the viewer locally to browse the archive:

```
$ cd ../viewer
$ fastly compute serve
```
