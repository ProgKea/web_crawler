use anyhow::{anyhow, Result};
use clap::Parser;
use linkify::{LinkFinder, LinkKind};
use std::fs;
use std::io::Write;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'u', long = "url")]
    url: String,

    #[clap(short = 'i', long = "iterations", default_value = "5")]
    iterations: usize,

    #[clap(short = 'o', long = "output")]
    output_filename: String,

    #[clap(short = 'k', long = "kind", default_value = "url")]
    kind: String,
}

struct Crawler {
    iterations: usize,
    urls: Vec<String>,
    emails: Vec<String>,
    kind: LinkKind,
    file: fs::File,
}

impl Crawler {
    fn new(
        starting_url: String,
        iterations: usize,
        kind: LinkKind,
        output_filename: String,
    ) -> Result<Self> {
        let file: fs::File = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .append(true)
            .open(output_filename)?;

        return Ok(Self {
            iterations,
            urls: {
                let mut urls = Vec::new();
                urls.push(starting_url);
                urls
            },
            emails: Vec::new(),
            kind,
            file,
        });
    }

    fn run(&mut self) -> Result<()> {
        for _ in 0..self.iterations {
            self.get_links()?;
        }

        return Ok(());
    }

    fn get_links(&mut self) -> Result<()> {
        let html = {
            let html = reqwest::blocking::get(&self.urls[0]);
            if html.is_err() {
                "".to_owned()
            } else {
                html.unwrap().text()?
            }
        };
        let mut finder = LinkFinder::new();
        finder.kinds(&[LinkKind::Url, LinkKind::Email]);

        for link in finder.links(&html) {
            match link.kind() {
                LinkKind::Url => {
                    let link = link.as_str().to_string();
                    self.urls.push(link.clone());
                    if self.kind == LinkKind::Url {
                        println!("Found url: {}", link);
                        self.file.write(link.as_bytes())?;
                        self.file.write(b"\n")?;
                    }
                }
                LinkKind::Email => {
                    let link = link.as_str().to_string();
                    if self.kind == LinkKind::Email {
                        println!("Found email: {}", link);
                        self.emails.push(link.clone());
                        self.file.write(link.as_bytes())?;
                        self.file.write(b"\n")?;
                    }
                }
                _ => unreachable!(),
            }
        }

        self.urls.remove(0);

        return Ok(());
    }
}

fn match_kind(kind_as_str: String) -> Result<LinkKind> {
    match kind_as_str.to_lowercase().as_str() {
        "url" => Ok(LinkKind::Url),
        "email" => Ok(LinkKind::Email),
        _ => Err(anyhow!(
            "`{}` is not a valid kind.\nValid kinds are: Email and Url.",
            kind_as_str
        )),
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let kind = match_kind(args.kind)?;
    let mut crawler = Crawler::new(args.url, args.iterations, kind, args.output_filename)?;

    crawler.run()?;

    return Ok(());
}
