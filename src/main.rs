extern crate cargo;
extern crate docopt;
extern crate rustc_serialize;
extern crate toml;

pub mod ops {
    pub mod cargo_output_metadata;
}

use std::env;

use ops::cargo_output_metadata::{output_metadata, OutputTo, OutputFormat, OutputMetadataOptions};
use cargo::execute_main_without_stdin;
use cargo::util::{CliResult, CliError, Config, human};
use cargo::util::errors::ChainError;
use cargo::util::important_paths::find_root_manifest_for_wd;

#[derive(RustcDecodable)]
struct Options {
    flag_features: Vec<String>,
    flag_manifest_path: Option<String>,
    flag_no_default_features: bool,
    flag_output_format: OutputFormat,
    flag_output_path: OutputTo,
    flag_verbose: bool,
}

pub const USAGE: &'static str = r#"
Output the resolved dependencies of a project, the concrete used versions
including overrides, in machine-readable format.

Warning! This command is experimental and output format it subject to change in future.

Usage:
    cargo metadata [options]

Options:
    -h, --help               Print this message
    -o, --output-path PATH   Path the output is written to, otherwise stdout is used
    -f, --output-format FMT  Output format [default: toml]
                             Valid values: toml, json
    --features FEATURES      Space-separated list of features
    --no-default-features    Do not include the `default` feature
    --manifest-path PATH     Path to the manifest
    -v, --verbose            Use verbose output

The TOML format is e.g.:

     root = "libA"

     [packages.libA]
     dependencies = ["libB"]
     path = "/home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/libA-0.1"
     version = "0.1"

     [packages.libB]
     dependencies = []
     path = "/home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/libB-0.4"
     version = "0.4"

     [packages.libB.features]
     featureA = ["featureB"]
     featureB = []

"#;

fn main() {
    execute_main_without_stdin(run, false, USAGE);
}

fn run(options: Options, config: &Config) -> CliResult<Option<()>> {
    config.shell().set_verbose(options.flag_verbose);
    let cwd = try!(env::current_dir().chain_error(|| {
        human("couldn't get the current directory of the process")
    }));
    let manifest = try!(find_root_manifest_for_wd(options.flag_manifest_path, &cwd));
    let options = OutputMetadataOptions {
        features: options.flag_features,
        manifest_path: &manifest,
        no_default_features: options.flag_no_default_features,
        output_format: options.flag_output_format,
        output_to: options.flag_output_path,
    };

    output_metadata(options, config)
        .map(|_| None)
        .map_err(|err| CliError::from_boxed(err, 101))
}
