# rlog

Rlog is a minimalistic blog/portfolio starter project intended to be used for a personal blog.

The project is built using only Rust, HTML and CSS as to remain as lightweight and performant as possible.


## Inspiration

Rlog was inspired by [Bearblog](https://bearblog.dev/), a similar project written in Django. If you're looking
for a blog that you can edit online, definitely use Bearblog.


## Usage

If you wish to use rlog for your own blog, it is recommended to fork the project, as to not interfere with the
project's development.

Rlog is a self-hosted project, meaning that you will need to host it on your own server.


### Without Docker

To build the project, you will need to have Rust installed on your machine.

To run the project, run the following command:

```bash
cargo run
```

This will start a server on `localhost:8080`, which you can access in your browser.

To build the project for production, run the following command:

```bash
cargo build --release
```

This will create an executable in the `target/release` directory, which you can run to start the server.

```bash
./target/release/rlog
```

You will have to move the `rlog` executable to the root of the project, so it has access to the `posts`,
`templates` and `static` directories.


### With Docker

Rlog can also be run using Docker. The Dockerfile and docker-compose.yml files are already provided in 
the project. All that is needed to run the project is to build the Docker image and run the container,
using the following command:

```bash
docker-compose up -d
```

This will start a container running the project on `localhost:8080`.


### Additional notes

To host the project on a server, you will need to configure a reverse proxy to forward requests to the
service running the project. I will not go into detail on how to do this, as it is out of the scope of this
project. Creating, running and hosting a service is not unique to rlog.

**NOTE: This project is completely free to use and modify, all I ask is that you leave the `made with rlog`
footer in the project.**

## Features

Currently, there are 4 distinct pages in the project:
 - Home
 - About page
 - Contact page
 - Blog page

The blog page is the only dynamic page in the project, as it reads the blog posts from a `posts` directory
and parses, formats and displays them on the page.


## How it works

The blog posts are written in markdown, while their metadata is stored in a TOML file. You will find an example
of a blog post in the `posts` directory. The metadata file is used to store the title, slug and date of a blog post,
while the markdown file has the actual content of the post.
