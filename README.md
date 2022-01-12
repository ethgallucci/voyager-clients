<div align="center">

[![version-shield]][crate-link] [![downloads-shield]][crate-link] [![docs-build-shield]][docs-url] [![contributors-shield]][contributors-url] [![license-shield]][license-url] [![issues-shield]][issues-url]

</div>


<h1 align="center">
    Voyager
</h1>

<div align="center">
    <img src="/docs/img/sat.png" width="244" />
</div>



## Overview
Voyager is a swiss army knife library for the NASA Open APIs. It is designed to bundle all the NASA APIs into a single package. Voyager can be used to gather data from a variety of NASA's endpoints, including: Picture of The Day, Solar Flares, Magnetic Storms, Near Earth Objects etc.

Future versions of voyager will strive to incorporate more endpoints, until all of them are integrated.

## Crate Usage
### Key Store
First create a .env file at the root of your project and add a variable named "API_KEY" with your API key from NASA as it's value. Make sure to add .env to your gitignore!
### Sample progam with voyager_client
```rust
    use voyager_client::{donki_client, timing};

    fn main() {
        use voyager_client::{donki, time};

        // Instantiate a base client
        let base = donki::GeoMagnetic::new();
        
        // Setup time
        let start = String::from("2015-01-01");
        let end = time::today();
        
        // Query the endpoint
        base.query(start, end).unwrap();
    }
```
This is a very simple program using voyager_client. We instantiate our base client for the Coronal Mass Ejection endpoint, and setup our timing parameters for our query. Then we pass the start and end dates into the query function. This will return a JSON string in prettyfied format.


## Contributing
The entire library can be found in [lib.rs](https://github.com/ethgallucci/voyager/blob/main/src/lib.rs), as well as it's documentation. [main.rs](https://github.com/ethgallucci/voyager/blob/main/src/main.rs) is a small executable that contains unit-tests for the voyager_client crate. All contributors are welcome! Simply clone this repository and work on a new branch, when you are ready you can open a PR.

The .cargo directory contains a config file that defines some aliases that are handy for test-driven development. In the root directory you can run: 
```sh
    cargo unit-test
```
This is a quick way to run all unit tests defined in [main.rs](https://github.com/ethgallucci/voyager/blob/main/src/main.rs).

You can also run:
```sh
    cargo doc-test
```

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