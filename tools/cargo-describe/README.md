Cargo Describe
===============

Basically `cargo metadata`, but with output that can be scripted against that doesn't require cryptic `jq` commands.

Install
--------
The usual way.  `cargo install cargo-describe`.

Usage
-----

```
CLI tool to produce human-friendly information from cargo-metadata

Usage: cargo-describe.exe [OPTIONS]

Options:
  -m, --manifest <MANIFEST>
          Path to a non-standard 'Cargo.toml' package manifest, if not in current directory

  -o, --output-format <OUTPUT_FORMAT>
          The output format of this tool

          [default: human-text]

          Possible values:
          - human-text: Pretty human text
          - csv:        Comma separated values
          - md-table:   Markdown Table
          - plain:      No formatting, one value per line

  -f, --fields <FIELDS>
          An optional list of manifest fields to display

          [default: name version]

          Possible values:
          - name:                          Name of the package
          - version:                       Version of the package from the manifest
          - git-version:                   'Git Describe' output of the specific package
          - module-relative-path:          Path to the module's dir, relative to the root dir
          - module-absolute-path:          Absolute path on disk to the module dir
          - module-relative-manifest-path: Path to the module's Cargo.toml, relative to the root dir
          - module-absolute-manifest-path: Absolute path on disk to the module's Cargo.toml
          - all:                           Prints everything

  -p, --package <PACKAGE>
          An optional list of crate packages members to display in this workspace

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

Examples
---------
...when run from the root of this repository:

### No Arguments, "Human Pretty Format":

```
$> cargo run --bin cargo-describe -- 
irox
        Version : 0.5.0
irox-carto
        Version : 0.5.0
irox-enums
        Version : 0.2.1
irox-enums_derive
        Version : 0.2.1
irox-time
        Version : 0.3.5
...snipped...
```

### Human Pretty, all fields:

```
$> cargo run --bin cargo-describe -- -fall
irox
        Version : 0.5.0
        Git Version : irox-0.5.0-10-g23110f9
        Module Relative Path : irox
        Module Absolute Path : V:\irox\irox\
        Manifest Relative Path : irox\Cargo.toml
        Manifest Absolute Path : V:\irox\irox\Cargo.toml
irox-carto
        Version : 0.5.0
        Git Version : irox-carto-0.5.0-44-gf2d62c7
        Module Relative Path : libraries\carto
        Module Absolute Path : V:\irox\libraries\carto\
        Manifest Relative Path : libraries\carto\Cargo.toml
        Manifest Absolute Path : V:\irox\libraries\carto\Cargo.toml
irox-enums
        Version : 0.2.1
        Git Version : irox-enums-0.2.1-48-g7b1646e
        Module Relative Path : libraries\enums
        Module Absolute Path : V:\irox\libraries\enums\
        Manifest Relative Path : libraries\enums\Cargo.toml
        Manifest Absolute Path : V:\irox\libraries\enums\Cargo.toml
irox-enums_derive
        Version : 0.2.1
        Git Version : irox-enums_derive-0.2.1-49-g845a800
        Module Relative Path : libraries\enums_derive
        Module Absolute Path : V:\irox\libraries\enums_derive\
        Manifest Relative Path : libraries\enums_derive\Cargo.toml
        Manifest Absolute Path : V:\irox\libraries\enums_derive\Cargo.toml
...snipped..
```

### Plain Output (easiest for scripting):
Will print just the values of the specified fields, one per line organized by package.

```
$> cargo run --bin cargo-describe -- -o plain -f module-relative-manifest-path
irox\Cargo.toml
libraries\carto\Cargo.toml
libraries\enums\Cargo.toml
libraries\enums_derive\Cargo.toml
libraries\time\Cargo.toml
libraries\tools\Cargo.toml
libraries\units\Cargo.toml
data-formats\csv\Cargo.toml
libraries\egui_extras\Cargo.toml
data-formats\gpx\Cargo.toml
interfaces\influxdb_v1\Cargo.toml
libraries\network\Cargo.toml
libraries\types\Cargo.toml
libraries\log\Cargo.toml
data-formats\nmea0183\Cargo.toml
libraries\progress\Cargo.toml
data-formats\raymarine-sonar\Cargo.toml
data-formats\sirf\Cargo.toml
libraries\structs\Cargo.toml
libraries\structs_derive\Cargo.toml
libraries\stats\Cargo.toml
libraries\threading\Cargo.toml
interfaces\win-location-api\Cargo.toml
eieio\api\Cargo.toml
eieio\nmea0183\Cargo.toml
libraries\build-rs\Cargo.toml
tools\cargo-describe\Cargo.toml
tools\gpsd\Cargo.toml
tools\halflifes\Cargo.toml
tools\influx-cli\Cargo.toml
tools\sonar-sdf-convert\Cargo.toml

```

### CSV with Filtering:

```
$> cargo run --bin cargo-describe -- -o csv -p irox-tools -p cargo-describe
Name,Version
irox-tools,0.4.1
cargo-describe,0.1.0
```

### Markdown Table (filtered):

using `-o md-table` will print a markdown-formatted table

```
$> cargo run --bin cargo-describe -- -o md-table -p irox-tools -p cargo-describe
| Name           | Version |
|----------------|---------|
| irox-tools     | 0.4.1   |
| cargo-describe | 0.1.0   |
```

Which renders as:

| Name           | Version |
|----------------|---------|
| irox-tools     | 0.4.1   |
| cargo-describe | 0.1.0   |

### Markdown Table with All Fields (filtered):

using `-f all` will print all available fields:

```
$> cargo run --bin cargo-describe -- -o md-table -p irox-tools -p cargo-describe -f all

| Name           | Version | GitVersion                      | ModuleRelativePath   | ModuleAbsolutePath            | ModuleRelativeManifestPath      | ModuleAbsoluteManifestPath              |
|----------------|---------|---------------------------------|----------------------|-------------------------------|---------------------------------|-----------------------------------------|
| irox-tools     | 0.4.1   | irox-tools-0.4.1-14-g465c708    | libraries\tools      | V:\irox\libraries\tools\      | libraries\tools\Cargo.toml      | V:\irox\libraries\tools\Cargo.toml      |
| cargo-describe | 0.1.0   | cargo-describe-0.1.0-1-g1762e52 | tools\cargo-describe | V:\irox\tools\cargo-describe\ | tools\cargo-describe\Cargo.toml | V:\irox\tools\cargo-describe\Cargo.toml |

```

Which renders as:

| Name           | Version | GitVersion                      | ModuleRelativePath   | ModuleAbsolutePath            | ModuleRelativeManifestPath      | ModuleAbsoluteManifestPath              |
|----------------|---------|---------------------------------|----------------------|-------------------------------|---------------------------------|-----------------------------------------|
| irox-tools     | 0.4.1   | irox-tools-0.4.1-14-g465c708    | libraries\tools      | V:\irox\libraries\tools\      | libraries\tools\Cargo.toml      | V:\irox\libraries\tools\Cargo.toml      |
| cargo-describe | 0.1.0   | cargo-describe-0.1.0-1-g1762e52 | tools\cargo-describe | V:\irox\tools\cargo-describe\ | tools\cargo-describe\Cargo.toml | V:\irox\tools\cargo-describe\Cargo.toml |

## Fields

### Name

* `-f name` or `-f all`
* Output: The package name from the Cargo.toml manifest
* Example for irox-tools: `irox-tools`

### Version

* `-f version` or `-f all`
* Output: The package version from the Cargo.toml manifest
* Example for irox-tools: `0.4.1`

### Git Version

This provides a slightly different variant of `git describe`.  Rather than operating on the entire repository, it only
operates on the specific package directory of the repository.  It would be like if `git describe` accepted a path 
filtering parameter like `git log <path>...` does.  As if `git log <package-dir> | git describe` worked.

* `-f git-version` or `-f all`
* Output: My preferred variant of `git describe`:
    * `{name}-{version}-{commit_count}-g{short_hash}{-dirty}`
      * `name`: The same as `-f name` - the package name from the manifest
      * `tag`: Attempts to find the latest git tag that contains the last commit for that packages directory. If it cannot
        find a commit, then will revert back to the output of `-f version` - the version from the manifest file
      * `commit_count`: The number of commits `HEAD` is after `{tag}`
      * `short_hash`: The git short-hash of `HEAD`
      * `-dirty`: Optional dirty suffix if the working tree is dirty
* Example for irox-tools: `irox-tools-0.4.1-14-g465c708`
* Example for dirty irox-tools: `irox-tools-0.4.1-14-g465c708-dirty`

### Module Relative Path

* `-f module-relative-path` or `-f all`
* Output: The relative path of the package against the root of the workspace.  What would be in the root manifests `members` array
* Example for irox-tools: `libraries\tools`

### Module Absolute Path

* `-f module-absolute-path` or `-f all`
* Output: The absolute path on disk of the package.  If run from the root of the workspace, the same as `pwd/module-relative-path`
* Example for irox-tools: `V:\irox\libraries\tools`

### Module Relative Manifest Path

* `-f module-relative-manifest-path` or `-f all`
* Output: `module-relative-path` + `Cargo.toml`. 
* Example for irox-tools: `libaries\tools\Cargo.toml`

### Module Absolute Manifest Path

* `-f module-absolute-manifest-path` or `-f all`
* Output: `module-absolute-path` + `Cargo.toml`
* Example for irox-tools: `V:\irox\libraries\tools\Cargo.toml`