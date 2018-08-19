# Trope Doc

Trope is a simple command line utility for merging various config file types and environmental variables into
either YAML or JSON. It is a pure Rust implementation built for speed and as a showcase of Rust's ability to
handle merging different types. It uses *clap-rs* for CLI interpretation, *config-rs* for
file handling and *serde* for serialization.

## Supported File Types

Trope supports the following file types as inputs:

* [YAML](http://yaml.org/spec/1.2/spec.html)
* [JSON](https://tools.ietf.org/html/rfc7159)
* [HJSON](https://hjson.org/)
* [TOML](https://npf.io/2014/08/intro-to-toml/)
* [INI](https://en.wikipedia.org/wiki/INI_file)

## Installation

## Usage

Once installed, using Trope is simple: `trope [FLAGS] [OPTIONS]...`  
Typing `trope -h` provides an in-console listing of all the information listed here.

### Features

#### Flags

* `-h` `--help` Displays inline help
* `-V` `--version` Displays current version
* `-e` `--env` Merges any environmental variables with the prefix 'TROPE'
* `-d` `--debug` When set, trope displays the merged configs instead of writing to file
  
  NOTE: `--output` must still be satisfied

#### Options

* `-I` `--input` Used to add merging candidates with syntax: *PATH/FILENAME.extension* each candidate separated by a space
* `-O` `--output` Defines or creates the desired output file with syntax: *PATH/FILENAME.extension*

## Examples

Using three sample files: *Settings.yaml*, *Server.toml* and *Johndoe.json* listed below:

### Settings.yaml

```yaml
---
debug: false
port: 8080
host: 0.0.0.0
test-float: 3.4
test-int: 43
array:
  - One
  - two:
      a: 1
      b: 2
  - three
```

### Server.toml

```toml
[servers]

  [servers.alpha]
  ip = "10.0.0.1"
  dc = "eqdc10"

  [servers.beta]
  ip = "10.0.0.2"
  dc = "eqdc10"
  country = "中国"
```

### Johndoe.json

```json
{
    "name": "John Doe",
    "age": 40,
    "address": {
      "street": "11 Castle Lane",
      "city": "London"
    },
    "phones": [
      "+44 1234567",
      "+44 2345678"
    ]
  }
```

Running `trope -I Settings.yaml Server.toml Johndoe.json -O output.yaml` from the working directory will return a file named `output.yaml`:

```yaml
---
age: 40
"test-int": 43
address:
  street: 11 Castle Lane
  city: London
host: 0.0.0.0
phones:
  - +44 1234567
  - +44 2345678
name: John Doe
port: 8080
array:
  - One
  - two:
      b: 2
      a: 1
  - three
debug: false
servers:
  beta:
    dc: eqdc10
    ip: 10.0.0.2
    country: 中国
  alpha:
    ip: 10.0.0.1
    dc: eqdc10
"test-float": 3.4
```

## Known Bugs

* Trope incorrectly handles **-inf**, **+inf** and **NaN** values.
  It handles these values in an unpredictable fashion and should be
  avoided if possible.  This is as a result of the libraries used,
  and will correct as these libraries are updated.