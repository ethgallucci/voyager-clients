# nerva

> [![version-shield]][crate-link] [![downloads-shield]][crate-link] [![docs-build-shield]][docs-url] [![contributors-shield]][contributors-url] [![license-shield]][license-url] [![issues-shield]][issues-url]

## Overview
_voyager-clients (nerva)_ is a crate that contains client implementations for [NASA's Open APIs](https://api.nasa.gov). __nerva__ can be used to query any NASA Open API. The following is a simple example that employs _nerva_ to fetch the picture of the day from NASA's APOD API.

## Example: Fetch the Picture of the Day (APOD)
```Rust
use nerva::prelude::{__x::*, params::ApodPara,};
use nerva::clients::apod::Apod;
use nerva::core::Aim;

fn main() where {
    let aim = Aim::<Apod, ApodPara>::from(Apod::default());
    let apod = Nerva::from(aim);
    let response = apod.get().unwrap();
    println!("{:#?}", response);
}

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
