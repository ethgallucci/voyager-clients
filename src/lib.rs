//! # NASA Open APIs client implementations.
//! Features light bindings for a multitude of APIs. Including:
//! * Picture of the day -> [`apod`]
//! * Database of Notifications, Knowledge, Information -> [`donki`]  
//! * Jet Propulsion Laboratory -> [`jpl`]
//! * Near Earth Objects -> [`neo`]
//! * Tech Transfer -> [`tech`]
//! ## The `Aim` type
//! `nerva` uses interfaces to encapsulate the behavior of every NASA Open API, and each endpoint is represented as a subclient.
//! The subclient is then coupled with its parameters to create what we call an `Aim`.
//! The `Aim` type is a wrapper around the subclient and its parameters, and it is passed on to a generic client that can handle
//! any `Aim` type. Let's see how we can use the Aim type to fetch the Picture of the Day from APOD, then, we'll
//! use it to switch between open APIs seamlessly.  
//!
//! ### Example: Fetching the Picture of the Day
//! ```rust
//! use nerva::prelude::{__x::*, params::ApodPara,};
//! use nerva::clients::apod::Apod;
//! use nerva::core::Aim;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> where {
//!     // Create an `Aim` on `Apod`
//!     let aim = Aim::<Apod, ApodPara>::from(Apod::default());
//!     // Get a `Client` on the `Aim`
//!     let nerva = Nerva::from(aim);
//!     // Fetch the Picture of the Day
//!     let pod = nerva.get()?;
//!     println!("{:#?}", pod);
//!     return Ok(());
//! }
//!
//!
//! ```
//! ### Example: Spawning Multiple Clients
//!
//! If we, for instance then, know that our application will require connections on the `Mpc` and
//! `Flr` endpoints respectively, we can write a function to instantiate `Aim`'s for us:
//! ```no_run
//! use nerva::prelude::__x::*;
//! use nerva::core::{Aim, Params,};
//! use nerva::clients::prelude::{Mpc, Flr,};
//!
//! // Set two aims on Mpc and Flr
//! fn set_aims() -> (Aim<Mpc, impl Params>, Aim<Flr, impl Params>) {
//!     return (
//!         Aim::<Mpc, MpcPara>::from(Mpc::default()),
//!         Aim::<Flr, FlrPara>::from(Flr::default()),
//!     );
//! }
//!
//! ```
//! Now, we can use `set_aims` to establish clients on `Mpc` and `Flr`,
//! ```no_run
//! use nerva::prelude::__x::*;
//! use nerva::core::{Aim, Params,};
//! use nerva::clients::prelude::{Mpc, Flr,};
//!
//! // Set two aims on Mpc and Flr
//! fn set_aims() -> (Aim<Mpc, impl Params>, Aim<Flr, impl Params>) {
//!     return (
//!         Aim::<Mpc, MpcPara>::from(Mpc::default()),
//!         Aim::<Flr, FlrPara>::from(Flr::default()),
//!     );
//! }
//!
//! // Spawn two clients
//! fn main() -> Result<(), Box<dyn std::error::Error>> where {
//!    // Set two aims on Mpc and Flr
//!    let (mpc_aim, flr_aim) = set_aims();
//!    // Spawn an Mpc Client
//!    let mpc = Nerva::from(mpc_aim);
//!    // Spawn an Flr Client
//!    let flr = Nerva::from(flr_aim);
//!    // Query the clients
//!    let resp_mpc = mpc.get()?;
//!    let resp_flr = flr.get()?;
//!    // Print the responses
//!    println!("MPC: {:#?}\n\n\nFLR: {:#?}", resp_mpc, resp_flr);
//!    return Ok(());
//! }
//! ```
//!
//! ### Example: Switching between APIs
//!
//! ```rust
//! use nerva::prelude::{__x::*, params::{ApodPara, NeoFPara},};
//! use nerva::clients::prelude::{Apod, NeoF,};
//! use nerva::core::{Aim, Params,};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> where {
//!     // Create an `Aim` on `Apod`
//!     let apod = Aim::<Apod, ApodPara>::from(Apod::default());
//!     // Create an `Aim` on `NeoF`
//!     let neof = Aim::<NeoF, NeoFPara>::from(NeoF::default());
//!
//!     // Create a client on `Apod`
//!     let mut nerva = Nerva::from(apod);
//!     // Fetch the Picture of the Day
//!     let pod = nerva.get()?;
//!
//!     // Switch to `NeoF`
//!     let nerva = nerva.switch(neof);
//!     // Fetch the Near Earth Objects Feed
//!     let feed = nerva.get()?;
//!     
//!     // Print the responses
//!     println!("{:#?}\n\n\n{:#?}", pod, feed);
//!     return Ok(());
//!
//! }
//! ```

#![feature(return_position_impl_trait_in_trait)]
#![feature(associated_type_defaults)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]
#![warn(missing_docs)]
/// API client implementations
pub mod clients;
/// Core abstractions
pub mod core;
/// Re-exports
pub mod prelude;

#[cfg(test)]
mod readme {
    use crate::clients::prelude::{Apod, NeoF};
    use crate::core::Aim;
    use crate::prelude::{__x::*, params::{ApodPara, NeoFPara},};

    #[test]
    fn readme() {

        let today = chrono::Local::today().to_string();
        let mut aim = Aim::<Apod, ApodPara>::from(Apod::default());
        aim.set_params(ApodPara::Date(&today));
        let nerva = Nerva::from(aim);
        let pod = nerva.get().unwrap();
        println!("{:#?}", pod);
    }

    #[test]
    fn readme2() {
        let aim = Aim::<Apod, ApodPara>::from(Apod::default());
        let apod = Nerva::from(aim);

        let aim = Aim::<NeoF, NeoFPara>::from(NeoF::default());
        let neof = apod.switch(aim);
    }
}
