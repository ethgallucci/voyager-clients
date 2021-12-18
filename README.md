[![Contributors][contributors-shield]][contributors-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]


## Overview
Voyager is a swiss army knife CLI for the NASA Open APIs. It is designed to bundle all the NASA APIs into a single CLI tool. Voyager can be used to gather data from a variety of NASA's endpoints, including: Picture of The Day, Solar Flares, Magnetic Storms, and the Exoplanet Archive. 

Future versions of voyager will strive to incorporate more endpoints, until all of them are integrated.

## Installation

### Rustup?
This tool requires the Rust tool chain to build the binaries. This section assumes it is your first time interacting with Rust, and therefore need to install the Rust toolchain before building voyager. To install Rust, follow the directions from [Rust's official documenation](https://doc.rust-lang.org/book/ch01-01-installation.html). If you don't want to click away and want to get started with voyager right away, you can run the following command pulled straight from Rust's documentations:
```sh
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
This will install a script that starts the installation of the rustup tool, which installs the latest stable version of Rust, as well as Rust's native package manager, cargo. If at any point you are getting errors from rustup or cargo, make sure to check out the above link for Rust's official documentation on installing the toolchain. To verify the installation of the Rust compiler succeeded, you can run:
```sh
    rustc --version
```
To ensure the native package manager is installed, run:
```sh
    cargo --version
```
We'll use cargo to build the binaries for voyager.

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
If you don't have an API key yet, you can visit [NASA's Open API Documentation](https://api.nasa.gov/index.html) to set one up.
Then run the command above to link your key with voyager. It's recommended to save the key in a file somewhere, just in case you need it later.
```sh
    voyager get key
```
Run this command to ensure voyager has saved your key properly.

## Usage
```sh
    voyager apod
```
This command will access NASA's 'A Picture a Day' API endpoint and retrieve data about today's picture from NASA!
The output contains the url to the picture, future versions of voyager will support flags that will allow the image to be downloaded to the current directory.

### Available commands
```sh
    voyager magnetic
```
This command will retrive data from NASA's magnetic storms API. 

```sh
    voyager flare
```
Retrieves solar flare data from the entire year.

```sh
    voyager exoplanet
```
Grabs all data from NASA's exoplanet archive database.


[contributors-shield]: https://img.shields.io/github/contributors/ethgallucci/voyager.svg?style=for-the-badge

[contributors-url]: https://github.com/ethgallucci/voyager/graphs/contributors

[issues-shield]: https://img.shields.io/github/issues/ethgallucci/voyager.svg?style=for-the-badge
[issues-url]: https://github.com/ethgallucci/voyager/issues


[license-shield]: https://img.shields.io/github/license/othneildrew/Best-README-Template.svg?style=for-the-badge
[license-url]: https://github.com/ethgallucci/voyager/blob/main/LICENSE.txt
