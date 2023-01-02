# athena

A small command-line utility to quickly upload code to select providers (see [#Providers](#Providers)).

## Usage

Invoke `athena` with one of the supported providers, optionally passing in a path to a file to upload. If you omit a file path,  `athena` will open `$EDITOR` and upload the contents of the saved file to the provider.

```sh
athena <provider> [path/to/file]
```

`athena` will return the url to access the paste.

## Providers

- [sourceb.in](https://sourceb.in)
- [tsplay.dev](https://tsplay-dev.vercel.app)
- [GitHub Gists](https://gist.github.com)