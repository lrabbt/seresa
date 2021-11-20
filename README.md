# Seresa

Seresa is a program to publish and search research papers, akin to [Sci-Hub][1], that uses [Freechains][2] as its underlying infraestructure.

## Objective

Seresa objective is to allow users to easily share, search and consume research papers, without fear of censorship and without relying on a centralized institution.

## Usage

The program assumes the user has a Freechains node running on a reachable machine and that every chain and key pairs have already been created, since reimplementing those functionalities would overlap functionalities with `freechains` command.

The program is divided on two major subcommands: `share` and `resource`.

`share` subcommand is responsible for sharing and searching paper information, such as finding the paper on a Freechains chain, getting the paper title or getting the paper URI.

`resource` subcommand is responsible for uploading and downloading files on a Freechains chain, splitting the files on blocks which fit on Freechains maximum post size.

#### Share

Sharing a resource on the "#forum" chain, at a `http` site and tagging it `freenet` and `p2p`:

```bash
$ seresa share -c '#forum' post --sign $PVTKEY -t "Freenet Advances" --uri 'http://example.site/freenet-advances.pdf' --tag freenet --tag p2p --author "Some A. <some-author@example.site>"
1_B5691AE5009C08BE7202B3961D54A819BC27569DD267B9D7E9448C24D3B6607D
```

This will create a post with the information on a `JSON` format.

If we want to search for an article on a chain, we can use the `search` subcommand:

```bash
$ seresa share -c '#forum' search -s free
1_B5691AE5009C08BE7202B3961D54A819BC27569DD267B9D7E9448C24D3B6607D
```

And all posts on the "#forum" chain which contains "free" on its title, tags or authors will have their hashes printed.

For simplicity sake, there are commands to parse and print a post `uri` and `title` fields:

```bash
$ seresa share -c '#forum' get-uri --hash 1_B5691AE5009C08BE7202B3961D54A819BC27569DD267B9D7E9448C24D3B6607D
http://example.site/freenet-advances.pdf
```

```bash
$ seresa share -c '#forum' get-title --hash 1_B5691AE5009C08BE7202B3961D54A819BC27569DD267B9D7E9448C24D3B6607D
Freenet Advances
```

If the node is not on the local machine, we can specify its host:

```bash
$ seresa share -c '#forum' --host some-other-host --port 1234 search -s free
1_B5691AE5009C08BE7202B3961D54A819BC27569DD267B9D7E9448C24D3B6607D
```

The `JSON` format of the post is discusses below.

#### Resource

The `resource` subcommand downloads and uploads resources to a Freechains chain. References to a resource on a Freechains chain can be made by the `fchs` URI scheme, discussed below.

Uploading a file to a Freechains chain.

```bash
$ seresa resource upload -c '#forum' -f freenet.pdf -s $PVTKEY -t "Freenet"
2_DF294EFD01B567622111B2E808FF05983B32F5A2666E9A6594A8AAE08BFA9EA4
3_D78CB4F6817F996CAE5FEA0EBBE51A68CF1226FD6B21EE7A205C9FCB94172C5A
4_DB0AD9E3CC2E1ED33F6C3917FFD56BEC9F92E48A8AF460BF46FD9386C975E597
```

The three hashes on the output are the hashes of the file blocks. Each block has a `JSON` payload and its format and encoding are discussed below.

To download the recently uploaded file, we must get the reference for the last block of the file. The reference follows the `fchs` URI scheme:

```bash
$ seresa resource download -u 'fchs:#forum:4_DB0AD9E3CC2E1ED33F6C3917FFD56BEC9F92E48A8AF460BF46FD9386C975E597' > freenet.pdf
```

or

```bash
$ seresa resource download -u 'fchs:#forum:4_DB0AD9E3CC2E1ED33F6C3917FFD56BEC9F92E48A8AF460BF46FD9386C975E597' -o freenet.pdf
```

The reason we use a diferent URI scheme is only so we can reference the resource on a `uri` field on the `share` subcommand, the same way we would reference a `http` resource, for example:

```json
"uri": "fchs:#forum:4_DB0AD9E3CC2E1ED33F6C3917FFD56BEC9F92E48A8AF460BF46FD9386C975E597"
```

## Installation

### From Binary

Check out for the most recent [release](https://github.com/lrabbt/seresa/releases/tag/v0.1.0).

### From Source

Pre-requisites:

- rust >= 1.55.0
- cargo >= 1.55.0

After cloning the repository:

```bash
$ cargo build --release
```

The binary will be generated on `target/release/seresa`.

## Concepts

Here, we will present some concepts and patterns used by the program.

### Share Forum

Anyone can create their own forums to share links to resources, as well as anyone can join those forums to search and post links. We will call those forums "Share Forums".

Share forums are intended to be used as an entrypoint to share and find research papers and are simple Freechains Public Forums that follow certain patterns we will discuss later. As any Freechains Public Forum, the reputation system will be used to define if the shared resource is a good one for the forum or not.

Since bad resources will be ommited by dislikes on the Share Forum, when searching, the user will only be presented with resourced flagged as good resources by the community.

### Post Format

Since the program uses [Freechains][2] as its infraestructure, each resource will be shared on a Share Forum as a post, but a format will be defined so the application can post and search resources.

The post format will have a `JSON` payload, with the following schema, as defined using [JSON Schema][3]:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Seresa Share Forum post format",
  "description": "Format for posts containing reference to research paper",
  "type": "object",
  "properties": {
    "title": {
      "description": "Title of the article",
      "type": "string"
    },
    "authors": {
      "description": "Authors of the article",
      "type": "array",
      "items": {
        "type": "string"
      },
      "uniqueItems": true
    },
    "tags": {
      "description": "Tags of the article",
      "type": "array",
      "items": {
        "type": "string"
      },
      "uniqueItems": true
    },
    "uri": {
      "description": "Article URI",
      "type": "string",
      "format": "uri"
    }
  },
  "required": ["title", "authors", "tags", "uri"]
}
```

### Resource Location

Every post on the Share Forum will have to provide an URI for the posted paper, for the purpose of simplicity, any URI can be posted, for instance, "http", "ftp", etc.

A new schema for publishing files on a Freechains chain was created, and it will be discussed later.

### Search Query

Seresa will have support for search on all fields at first, using a simple include search.

### Freechains URI

`fchs` URI Schema will identify a resource on a public, private or public identity forum. The identified resource post must be accepted as consensus.

The URI will contain information about the forum, the shared/public keys and the post which contains a file, or the beginning of a file.

The following format will be used for the URI:

```
fchs:[<forum>[:<forum keys>]]:<post hash>
```

Where:

- "forum" is the name of the public, private or public identity forum.
- "forum keys" is a list of public or private keys separated by colons. Applied only for public and public identity forums.
- "post hash" is the hash of the post containing the file or the beginning of a file.

### Resource Format

For the program to be able to load a file from a `fsch` URI, the content pointed by the URI must be a `JSON` of the following [JSON Schema][3]::

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Seresa Resource post format",
  "description": "Format for posts containing potentially partitioned resource",
  "type": "object",
  "properties": {
    "title": {
      "description": "Resource title",
      "type": "string"
    },
    "content": {
      "description": "Base64 representation of the resource block",
      "type": "string"
    },
    "prev": {
      "description": "Previous block of the resource, \"null\" if last block",
      "type": ["string", "null"],
      "format": "uri"
    }
  },
  "required": ["title", "content", "prev"]
}
```

Since the file is cut in various blocks, the only thing limiting the size of the file is the reputation of the user on the chain the file is being posted.

If any resource block is not on the consensus, or has a low reputation (< -3), the resource will not be retrieved.

[1]: https://sci-hub.se/
[2]: https://github.com/Freechains/README/
[3]: http://json-schema.org/
