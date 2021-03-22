# gh-labels-cli

> A tool for managing your GitHub labels.

## Use-cases

I personally use the CLI for creating a standard set of labels when initializing a new repository on GitHub. I've created custom alias for `gh` that uses `gh-labels-cli` to create a repository and run the CLI for bulk label creation:

```console
$ gh alias set -s new 'gh repo create $1; cd $1; gh labels update --purge'
- Adding alias for new: gh repo create $1; cd $1; gh labels update --purge
✓ Changed alias new from !gh repo create $1; cd $1; gh labels update to !gh repo create $1; cd $1; gh labels update --purge
```

> **Note:** for this to work you need a [label definition file](#label-definitions-file) inside the configuration directory. See [here](#update) for more information.

## Installation

`gh-labels-cli` can be installed via homebrew:

```console
$ brew install sirwindfield/tap/gh-labels-cli
```

You can also build it from source using `cargo`:

```console
$ cargo install gh-labels-cli --locked
```

An AUR package is in the works :)

## Usage

The CLI can be used as either standalone by directly invoking it via the `gh-labels` binary or you can register aliases for the official GitHub CLI (`gh`) to integrate `gh-labels-cli` into it.

To register the aliases, run `gh-labels integration install`.

The CLI needs a personal access token with appropiate `public_repo` or `repo` scope, depending on whether you want it to work on private repositories as well. The token can be passed to the CLI using a CLI argument or via the environment variable `GH_LABELS_TOKEN`.

> **Note:** Some poeple may wish to re-use a singleton token across multiple CLIs. The CLIs I've stumbled across often use the `GITHUB_TOKEN` environment variable. This is also supported. The order in which the token is tried to be read from is `CLI argument` > `GH_LABELS_TOKEN` > `GITHUB_TOKEN`.

The CLI operates on repositories. Those can either be directly supplied via an argument in the form `owner/repo` or by running the CLI inside of an existing git repository with an upstream named `origin` pointing to `github.com`.

For more information, take a closer look at the help.

I know use the `gh new <repo-name>` alias to create a new repository instead of `gh repo create`.

## Commands

### Config

Used to query the configuration file path, content or to edit the configuration inside your terminal.

### Integration

Used to install and uninstall the `labels` alias for the `gh` CLI.

### Api

Commands related to actual GitHub API calls.

#### Create

Creates a single label inside a repository with the given values.

#### Update

Bulk-create labels and update existing ones inside of repositories. You have to supply a [label definition file](#label-definitions-file) for the command to work. The file can be supplied via the `-f,--file` argument or by putting the file inside the directory returned via the `gh-labels config --path`. The file has to be named `labels.json` when using the second option.

## Label definitions file

A label definitions file is a file that you write containing all the labels you want to apply to a repository. It's a JSON file with the following format:

```json
{
  "labels": [
    {
      "name": "type: bug",
      "color": "431232",
      "description": "A programming error"
    }
  ]
}
```

> **Note:** The description field is optional.

My own label definition file can be found [here](https://gist.github.com/SirWindfield/1fd1bb7f21c8d9170e69f52aa38c3201).

#### License

<sup>
Licensed under either of <a href="license-apache">Apache License, Version
2.0</a> or <a href="license-mit">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

