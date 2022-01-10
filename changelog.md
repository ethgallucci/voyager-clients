v0.2.8:

Adds support for the JPL Collection of api bindings.

Implemented Fireball Client

------------------------------------------------------------------------------------------------

v0.2.6:

Adds support for the Tech Transfer client.

Implements the patent collection by default, but can be switched to the software collection via usage of the software method.

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
