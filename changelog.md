------------------------------------------------------------------------------------------------
v0.2.0:

Overhauled all modules to make them more dynamic, composable, and modular. Made each client it's own struct with base_urls, and implementations for each struct based on the nature of the endpoint.

Added more dynamic and maintainable ways for querying endpoints by abstracting the client from the functions themselves, instead of simply having functions for each endpoint.

------------------------------------------------------------------------------------------------

v0.2.2:

Added support for Insight Rover and Coronal Mass Ejection API endpoints.

Moved start and end fields outside of struct and into parameters for the query functions.
