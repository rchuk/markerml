# MarkerML CLI

## General
This is a crate that provides CLI
for the MarkerML language.

For the full grammar overview,
refer to the [`markerml`](https://crates.io/crates/markerml) crate.

## Commands
The resulting program provides several commands:

- Command to convert file with MarkerML code into HTML
```sh
markerml_cli convert --input file.txt --output file.html
```
- Command to watch the given file with MarkerML code
  and track changes on a live-reloading HTML page
```sh
markerml_cli watch --input file.txt
```
- Command to display credits information
```sh
markerml_cli credits
```
- Command to display list of commands
```sh
markerml_cli help
```
