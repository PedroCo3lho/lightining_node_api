# Back-end-Coding-Challenge
API JSON built in Rust that periodically consults the Mempool lightning node state, process the data, and stores in a PostgresSQL database. The API features a GET method to retrieve the node's data.

## Build tools & versions used
- Axum 
- Diesel 
- PostgresSql (Supabase)

I chose the stack inspired by [Guilherme Salustiano's talk](https://youtu.be/v0axYVJX_hI), that showcases the Diesel Internals. 

## Steps to run the app

1. Rename the .env file and configure your Database URL

```bash
mv .env.example/ .env
```

2. Setup the project through the Diesel CLI 

```bash
diesel setup
```

3. Run the migrations

```bash
diesel migration run 
```

4. Run the project

```bash
cargo run 
```

## What was the reason for your focus? What problems were you trying to solve?
My goal was to implement the solution while learning and understanding the structure of a back-end built with this new stack. 
Some challenges I faced during development were:
- Researching the stack and figuring out how to integrate all components
- Learn how to iterate on the JSON structure ([reference](https://ectobit.com/blog/parsing-json-in-rust/))
- Type `PgNumeric` and `DateTime` in the the table struct, it was conflicting when applying `serde` macros

## How long did you spend on this project?
I spent **4 days**: the 1st day studying and choosing the stack; the 2nd day creating demos and the 3rd and 4º building the solution. My learning repo, which includes the study materials I used and demos, can be found here:
- [rust_studies](https://github.com/PedroCo3lho/rust_studies/tree/main)

## Did you make any trade-offs for this project? What would you have done differently with more time?
- I chose to use Supabase PostgreSQL to avoid additional local database setup. 
- Drop from Numeric type to Float in the capacity column, due to errors working with the `struct` and `PgNumeric`

With more time I would:
- Type more the code;
- Handle better the possible errors;
- Containerize the solution; 
- Implement better concurrency with diesel_async;
- Create tests to measure the performance, errors and security;

## What do you think is the weakest part of your project?

## Is there any other information you’d like us to know?

## References
- [Axum examples](https://github.com/tokio-rs/axum/tree/main/examples)
- [Diesel guides](https://diesel.rs/guides/getting-started)
