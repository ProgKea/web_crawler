# web_crawler

This is a simple web_crawler written in rust.

# usage

```console
git clone https://github.com/ProgKea/web_crawler
cargo run -- -i <iterations> -u <starting url> -k <kind of the thing you want to scrape> -o <output filename>
```

if you want to remove the duplications you can use coreutils for that:

```console
cat <output filename> | sort | uniq
```
