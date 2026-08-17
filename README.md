Branches in this repository:

[main](https://github.com/jamesjian/rust-topcoat-v0.5/tree/main): a basic web application displaying a static page

[mysql_mariadb_database](https://github.com/jamesjian/rust-topcoat-v0.5/tree/mysql_mariadb_database): a simple web application using crate sqlx to access MySql/Mariadb

[database_global_test_path_parameter](https://github.com/jamesjian/rust-topcoat-v0.5/tree/database_global_test_path_parameter): global database connection, unit test, and path parameter

# Topcoat v0.5.0 Simple Web Application

A simple web application built with **Topcoat v0.5.0**.

> **Note:** Topcoat v0.5.0 introduces breaking changes and is not compatible with previous versions.

## Overview

This project demonstrates a minimal Topcoat application that renders a simple home page.

Default URL:

http://127.0.0.1:3000/

## Running the Application

You can run the application in two ways.

### Option 1: Run with Cargo

```bash
cargo run

Option 2: Run with the Topcoat Development Server

The Topcoat development server automatically recompiles and reloads the application whenever source files change, making development much faster.

Install the Topcoat CLI:

cargo install topcoat-cli


Start the development server:

topcoat dev

Custom Host and Port

To run the application on a different host or port:

HOST=0.0.0.0 PORT=8080 topcoat dev


Example URL:

http://0.0.0.0:8080

Application Workflow

The request flow in this project is:

main.rs
    ↓
app.rs
    ↓
handlers/frontend/home.rs
    ↓
templates/frontend/home.rs

Responsibilities
main.rs – Application entry point.
app.rs – Configures and initializes the router.
handlers/frontend/home.rs – Acts as the controller and handles incoming requests.
templates/frontend/home.rs – Defines the page template and UI components.
Template System

In the current version of Topcoat, HTML files cannot be loaded directly as template files.

Instead, templates must be built using:

#[component] macro
view! macro

These macros are used to create reusable UI components and render page layouts.

Project Structure Notes

A few Rust module requirements are important:

Each folder should contain a mod.rs file where necessary.
Modules, functions, and structs that need to be accessed outside their module must be declared with the pub keyword.

These are required for proper module visibility and compilation.

Purpose

This project serves as a simple reference implementation for developers who are:

Learning Topcoat v0.5.0
Migrating from earlier Topcoat versions
Exploring the updated project structure and template system

