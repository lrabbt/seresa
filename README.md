# Seresa

Seresa is a program to publish and search research papers, akin to [Sci-Hub][1], that uses [Freechains][2] as its underlying infraestructure.

## Objective

Seresa objective is to allow users to easily share, search and consume research papers, without fear of censorship and without relying on a centralized institution.

## Usage

TODO

## Installation

TODO

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

### Query

Seresa will have support for search on all fields at first, using a simple include search.

### Resource Location

Every post on the Share Forum will have to provide an URI for the posted paper, the accepted schemes are:

- http
- fchs

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
    "next": {
      "description": "Next block of the resource, \"null\" if last block",
      "type": ["string", "null"],
      "format": "uri"
    }
  },
  "required": ["title", "content", "next"]
}
```

If any resource block is not on the consensus, the resource cannot be retrieved.

[1]: https://sci-hub.se/
[2]: https://github.com/Freechains/README/
[3]: http://json-schema.org/
