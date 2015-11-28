cargo metadata
==============

This tool adds ability get to metadata for a cargo project in a machine readable format (JSON or TOML).

The metadata includes information about crates, dependencies, their versions and paths.

This should be useful for Rust plugins and IDEs.


Installation
============

If you have a recent version of cargo (>= 0.6) you can cargo install it.

```bash
$ cargo install cargo-edit
```

Usage
=====

Inside the project directory `cargo metadata -f` will output information like this:


```json
{
  "root": {
    "name": "meta-example",
    "version": "0.1.0",
    "features": null
  },
  "packages": [
    {
      "name": "libc",
      "version": "0.2.2",
      "dependencies": [],
      "targets": [
        {
          "kind": [
            "lib"
          ],
          "name": "libc",
          "src_path": "\/home\/user\/.multirust\/toolchains\/stable\/cargo\/registry\/src\/github.com-88ac128001ac3a9a\/libc-0.2.2\/src\/lib.rs",
          "metadata": {
            "metadata": "f654c8036439f8ef",
            "extra_filename": "-f654c8036439f8ef"
          }
        }
      ],
      "manifest_path": "\/home\/user\/.multirust\/toolchains\/stable\/cargo\/registry\/src\/github.com-88ac128001ac3a9a\/libc-0.2.2\/Cargo.toml"
    },
    {
      "name": "meta-example",
      "version": "0.1.0",
      "dependencies": [
        {
          "name": "libc",
          "req": "^0.2.2"
        }
      ],
      "targets": [
        {
          "kind": [
            "lib"
          ],
          "name": "meta-example",
          "src_path": "\/home\/user\/meta-example\/src\/lib.rs",
          "metadata": {
            "metadata": "a8c2c094624697d4",
            "extra_filename": "-a8c2c094624697d4"
          }
        }
      ],
      "manifest_path": "\/home\/user\/meta-example\/Cargo.toml"
    }
  ]
}
```



