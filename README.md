[![version-shield]][crate-link] [![downloads-shield]][crate-link] [![docs-build-shield]][docs-url] [![commit-shield]][commit-url] [![contributors-shield]][contributors-url] [![license-shield]][license-url] [![issues-shield]][issues-url]


## Overview
Voyager is a swiss army knife library for the NASA Open APIs. It is designed to bundle all the NASA APIs into a single package. Voyager can be used to gather data from a variety of NASA's endpoints, including: Picture of The Day, Solar Flares, Magnetic Storms etc.

Future versions of voyager will strive to incorporate more endpoints, until all of them are integrated.

## Crate Usage
### Sample progam with voyager_client
Let's see how we can use the voyager_client in our Rust projects.
```rust
    use voyager_client::*;

    fn main() {
        keys::set_key("[YOUR_API_KEY]");
        let near_earth_objects = neo().unwrap();
        let magnetic_storms = weather::magnetic().unwrap();
    }
```
After running the set_key function once, voyager will create a text file at /Users/you/voyager/.api_key.txt that will store your api key. As such, you will only need to run the function once and it will generate errors if you try to run it again. To avoid this, you can read the section below about installing the CLI and configuring the api key through the command line.

Notice we did not use the println! macro to output the responses to our console. Each API query function includes a println! statement as well as a progress bar in the terminal. This can be changed in the future very easily.

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

## CLI Usage

### Available commands
```sh
    voyager magnetic
```
This command will retrive data from NASA's magnetic storms API. 

```sh
    voyager flare
```
Retrieves solar flare data.
```sh
    voyager apod
```
This command will access NASA's 'A Picture a Day' API endpoint and retrieve data about today's picture from NASA!
The output contains the url to the picture, future versions of voyager will support flags that will allow the image to be downloaded to the current directory.

[version-shield]: https://img.shields.io/crates/v/voyager_client?style=plastic

[contributors-shield]: https://img.shields.io/github/contributors/ethgallucci/voyager?style=plastic

[contributors-url]: https://github.com/ethgallucci/voyager/graphs/contributors

[issues-shield]: https://img.shields.io/github/issues/ethgallucci/voyager?style=plastic
[issues-url]: https://github.com/ethgallucci/voyager/issues


[license-shield]: https://img.shields.io/crates/l/voyager_client?style=plastic
[license-url]: https://github.com/ethgallucci/voyager/blob/main/LICENSE.txt

[commit-shield]: https://img.shields.io/github/commit-activity/w/ethgallucci/voyager?style=plastic
[commit-url]: https://github.com/ethgallucci/voyager/commits/main

[downloads-shield]: https://img.shields.io/crates/d/voyager_client?style=plastic
[crate-link]: https://crates.io/crates/voyager_client

[docs-build-shield]: https://img.shields.io/docsrs/voyager_client/latest?label=build&style=plastic
[docs-url]: https://docs.rs/voyager_client