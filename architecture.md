# The Architecture

The application and all it's components run in a Docker-Compose environment

## Composition

* The composition of the Application is a Micro Service Architecture
* The Backend is a Axum Restful Service
* The Frontend is a Leptos Client Application, running in an isoltated Axum Server for static pages
* The Database is a SurrealDB

## Rust specifications

* All Versions are stored only in the main Cargo.toml, and provided to the modules as from workbench