[![Contributors][contributors-shield]][contributors-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

## Installation

### Build binaries
```sh
    cargo build --release
```

### Copying binaries to your local path
```sh
    bash install.sh
```
This copies the binaries into your local path.
If the script isn't working make sure you built the source specifying the --release flag.

### Usage
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


[contributors-shield]: https://img.shields.io/github/contributors/ethgallucci/zero.svg?style=for-the-badge

[contributors-url]: https://github.com/ethgallucci/zero/graphs/contributors

[issues-shield]: https://img.shields.io/github/issues/ethgallucci/zero.svg?style=for-the-badge
[issues-url]: https://github.com/ethgallucci/zero/issues

[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/ethan-gallucci-196892221

[license-shield]: https://img.shields.io/github/license/othneildrew/Best-README-Template.svg?style=for-the-badge
[license-url]: https://github.com/ethgallucci/zero/blob/main/LICENSE.txt