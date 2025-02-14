# uap-core-clickhouse-parser-rs
Rust-based parser of ua-parser/uap-core yaml to generate Clickhouse-compatible YamlRegExpTree yaml files for browser, OS and device

## Source Data from ua-parser
This tool expects data in the format provided by the ua-parser group in the format
that is used in uap-core. The most up-to-date data can be obtained from their
[github](https://github.com/ua-parser/uap-core) in the file `regexex.yaml`

## Usage
Creates a binary called `generate` in the `./target/` directory. To use this,
you must specify an input file in the format from above. All other arguments
are optional. It supports specifying an output directory using the `--outdir path/to/directory/`
option. The name of each output file on a per-type basis can be spcified using
`--device` `--os` and `--user-agent` options. These can be used in combination
with specifying the output directory and the system will attempt to properly
construct paths for all output.

By default the program will not overwrite existing files and will generate an
error if the options would result in overwriting. There is a `-f` or `--force`
option to force the system to overwrite existing files.

Run `generate -h` to get a full rundown.
