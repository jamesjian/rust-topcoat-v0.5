This repository is all about Rust crate TopCoat v0.50 to create web applications. 

Branches in this repository:

[main](https://github.com/jamesjian/rust-topcoat-v0.5/tree/main): a basic web application displaying a static page

[mysql_mariadb_database](https://github.com/jamesjian/rust-topcoat-v0.5/tree/mysql_mariadb_database): a simple web application using crate sqlx to access MySql/Mariadb

[database_global_test_path_parameter](https://github.com/jamesjian/rust-topcoat-v0.5/tree/database_global_test_path_parameter): global database connection, unit test, and path parameter


# Topcoat v0.5.0 — Simple Web Application using global database connection and path parameter. 

A minimal web application built with **Topcoat v0.5.0** that connects to a MySQL/MariaDB database and get a product record from a table.

> **Note:** Topcoat v0.5.0 introduces breaking changes and is **not** backward compatible with earlier versions.

## Overview

This project demonstrates a minimal Topcoat application that queries a MySQL/MariaDB database and displays the contents of a `product` table.

**Default URL:** [http://127.0.0.1:3000/products](http://127.0.0.1:3000/productdetail/3)

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- A running MySQL or MariaDB server
- `cargo` available on your `PATH`

## Setup

1. Clone the repository and enter the project directory.
2. Create a `.env` file in the project root with your database connection string:

   ```env
   DATABASE_URL=mysql://mydb_user:password123@localhost/mydb
   ```

3. Set up the database and sample data — see [Database Setup](#database-setup) below.

## Running the Application

You can run the application in two ways.

### Option 1: Run with Cargo

```bash
cargo run
```

### Option 2: Run with the Topcoat Development Server

The Topcoat development server automatically recompiles and reloads the application whenever source files change, making development much faster.

Install the Topcoat CLI:

```bash
cargo install topcoat-cli
```

Start the development server:

```bash
topcoat dev
```

#### Custom Host and Port

To run the application on a different host or port:

```bash
HOST=0.0.0.0 PORT=8080 topcoat dev
```

Example URL: [http://0.0.0.0:8080](http://0.0.0.0:8080)

## Application Workflow

The request flow through the project is:

```
main.rs
  → loads DATABASE_URL from .env via the dotenvy crate and set a global database connection by 
     DB_POOL.set(pool).expect("Failed to initialized globle DB pool");
    ↓
app.rs
  → builds the router without passing database connection
    ↓
handlers/frontend/home.rs → productdetail()
  → acts as the controller, handles the incoming request. Use path_param, context to get the id from the url
	#[path_param(error = bad_request("Product ID must be a number!"))]
	pub struct ProductId(pub u32);
	#[page("/productdetail/{product_id}")]
	async fn productdetail(cx: &Cx)->Result{  ... }
    ↓
templates/frontend/t_home.rs → product_detail()
  → act as a view to call the model and render the page
  ↓
models/m_product.rs → m_product::get_product_detail(id)

  → act as a model. Uses the sqlx crate to query the database and return an Option<ProductItem>
```

## Project Structure & Responsibilities

| File / Folder                        | Responsibility                                              |
|---------------------------------------|---------------------------------------------------------------|
| `main.rs`                             | Application entry point.                                      |
| `app.rs`                              | Configures and initializes the router and app context.        |
| `handlers/frontend/home.rs`           | Controller — handles incoming requests.                       |
| `templates/frontend/t_home.rs`        | Defines the page template and UI components.                  |
| `models/m_product.rs`                    | Defines the database model.                                |

**Notes:**
- Each folder should contain a `mod.rs` file where necessary for module resolution.
- Any module, function, or struct that needs to be accessed outside its own module must be declared with the `pub` keyword. This is required for correct visibility and compilation.

## To do unit test:

cargo test

It will use init_test_db() in main.rs to run the test test_get_product_detail() defined in modes/m_product.rs

## Purpose

This project serves as a simple reference implementation for developers who are:

- Learning Topcoat v0.5.0
- Migrating from earlier Topcoat versions
- Exploring the updated project structure and template system
- Exploring the `sqlx` crate for database access

## Database Setup

Create the database, table, and sample data:

```sql
CREATE DATABASE mydb;
USE mydb;

CREATE TABLE product (
    id                INT AUTO_INCREMENT PRIMARY KEY,
    product_name      VARCHAR(255) NOT NULL,
    product_quantity  INT NOT NULL
);

INSERT INTO product (product_name, product_quantity) VALUES
    ('Wireless Mouse', 150),
    ('Mechanical Keyboard', 45),
    ('27-inch Monitor', 30),
    ('USB-C Hub', 200),
    ('Bluetooth Headphones', 85),
    ('External Hard Drive 1TB', 60),
    ('Webcam 1080p', 40),
    ('Desk Mat', 120),
    ('Ergonomic Office Chair', 15),
    ('LED Desk Lamp', 75);
```

Create a dedicated database user and grant it access:

```sql
CREATE USER 'mydb_user'@'localhost' IDENTIFIED BY 'password123';
GRANT ALL PRIVILEGES ON mydb.* TO 'mydb_user'@'localhost';
FLUSH PRIVILEGES;
```

> **Security tip:** `password123` is a placeholder for local development only. Use a strong, unique password for any non-local environment, and avoid committing real credentials to version control.