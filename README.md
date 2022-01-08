<h1 align="center">
    Voyager
</h1>

<div align="center">
    <img src="/docs/img/satellite-drawing-2.png" width="244" />
</div>


<div align="center">

[![version-shield]][crate-link] [![downloads-shield]][crate-link] [![docs-build-shield]][docs-url] [![contributors-shield]][contributors-url] [![license-shield]][license-url] [![issues-shield]][issues-url]

</div>



## Overview
Voyager is a swiss army knife library for the NASA Open APIs. It is designed to bundle all the NASA APIs into a single package. Voyager can be used to gather data from a variety of NASA's endpoints, including: Picture of The Day, Solar Flares, Magnetic Storms, Near Earth Objects etc.

Future versions of voyager will strive to incorporate more endpoints, until all of them are integrated.

## Crate Usage
### Sample progam with voyager_client
Let's see how we can use the voyager_client in our Rust projects.
```rust
    use voyager_client::{donki_client, timing};

    fn main() {
        // Instantiate Base Client
        let base = donki_client::CoronalMassEjection::new();
        
        // Setup Timing Parameters for Query
        let start = String::from("2019-01-01");
        let end = timing::today();

        // Query the API
        let res = base.query(start, end).unwrap();
    }
```
This is a very simple program using voyager_client. We instantiate our base client for the Coronal Mass Ejection endpoint, and setup our timing parameters for our query. Then we pass the start and end dates into the query function. This will return a JSON string in prettyfied format.

## CLI Installation

### Build binaries
To build the voyager binaries, run:
```sh
    cargo build --release
```

### Copying binaries to your local path
```sh
    bash install.sh
```

### Setup
Once you have the binaries installed, you must first configure your API key to use with voyager.
If you have one already setup through NASA, you can simply run the following command:
```sh
    voyager set key [YOUR_API_KEY]
```
If you don't have an API key yet, you can visit [NASA's Open API Documentation](https://api.nasa.gov/index.html) to set one up. Then run the command above to link your key with voyager. The key will be written to /Users/you/voyager/.api_key.txt.
```sh
    voyager get key
```
Run this command to ensure voyager has saved your key properly.


[version-shield]: https://img.shields.io/crates/v/voyager_client?style=plastic

[contributors-shield]: https://img.shields.io/github/contributors/ethgallucci/voyager?style=plastic

[contributors-url]: https://github.com/ethgallucci/voyager/graphs/contributors

[issues-shield]: https://img.shields.io/github/issues/ethgallucci/voyager?style=plastic
[issues-url]: https://github.com/ethgallucci/voyager/issues


[license-shield]: https://img.shields.io/crates/l/voyager_client?style=plastic
[license-url]: https://github.com/ethgallucci/voyager/blob/main/LICENSE

[commit-shield]: https://img.shields.io/github/commit-activity/w/ethgallucci/voyager?style=plastic
[commit-url]: https://github.com/ethgallucci/voyager/commits/main

[downloads-shield]: https://img.shields.io/crates/d/voyager_client?style=plastic
[crate-link]: https://crates.io/crates/voyager_client

[docs-build-shield]: https://img.shields.io/docsrs/voyager_client/latest?label=build&style=plastic
[docs-url]: https://docs.rs/voyager_client