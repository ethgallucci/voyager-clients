[![Contributors][contributors-shield]][contributors-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]


## Overview
Voyager is a swiss army knife CLI for the NASA Open APIs. It is designed to bundle all the NASA APIs into a single CLI tool. Voyager can be used to gather data from a variety of NASA's endpoints, including: Picture of The Day, Solar Flares, Magnetic Storms, and the Exoplanet Archive. 

Future versions of voyager will strive to incorporate more endpoints, until all of them are integrated.

## Installation

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
