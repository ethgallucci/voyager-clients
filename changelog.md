v0.3.0:

Breaking Change!

This release restructures the entire crate into more a intuitive package architecture. Modules have been moved into their own files for easier maintainability in development, main has been removed, tests have moved into the tests directory in unit_test.rs. While there were strong efforts to keep all modules in the same trees, some had to move to successfully make the existing structure more intuitive.

- All donki clients have moved into the donki module
- All jpl clients moved into the jpl module
- Timing module was renamed to time
- To_pretty module was renamed to pretty
- Insight_client was renamed to insight

------------------------------------------------------------------------------------------------

v0.2.9:

Adds more support for the JPL Collection of api bindings.

Implemented light support for Mission Design api bindings. Including Mission Design in Query mode (Q), Accessible mode (A) and Map mode (M).

------------------------------------------------------------------------------------------------

v0.2.8:

Adds support for the JPL Collection of api bindings.

Implemented Fireball Client

------------------------------------------------------------------------------------------------

v0.2.6:

Adds support for the Tech Transfer client.

Implements the patent collection by default, but can be switched to the software collection via usage of the switch method.

# Example
```rust
    let mut base = TechTransferClient::new();
    base.software();
```

------------------------------------------------------------------------------------------------

v0.2.5:

Breaking Change!

Overhauled key store architecture, deprecated CLI.
Voyager now relies on a .env file in the root with a variable called "API_KEY" in order for its base clients to successfully query endpoints

------------------------------------------------------------------------------------------------

v0.2.2:

Added support for Insight Rover and Coronal Mass Ejection API endpoints.

Moved start and end fields outside of struct and into parameters for the query functions.

------------------------------------------------------------------------------------------------
v0.2.0:

Overhauled all modules to make them more dynamic, composable, and modular. Made each client it's own struct with base_urls, and implementations for each struct based on the nature of the endpoint.

Added more dynamic and maintainable ways for querying endpoints by abstracting the client from the functions themselves, instead of simply having functions for each endpoint.
