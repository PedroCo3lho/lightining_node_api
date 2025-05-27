# Back-end-Coding-Challenge
API JSON built in Rust that periodically consults the Mempool lightning node state, process the data, and stores in a PostgresSQL database. The API features a GET method to retrieve the node's data.

## Build tools & versions used
- Axum 
- Diesel 
- PostgresSql (Supabase)

I chose the stack inpired by [Guilherme Salustiano's talk](https://youtu.be/v0axYVJX_hI), that showcases the Diesel Internals. 

## Steps to run the app

## What was the reason for your focus? What problems were you trying to solve?
My goal was to implement the solution while learning and understanding the structure of a back-end built with this new stack. The main challenges I faced during development were:
- Researching the stack and figuring out how to integrate all components
- Iterating on the JSON structure ([reference](https://ectobit.com/blog/parsing-json-in-rust/))

## How long did you spend on this project?
I spent **4 days**: 1 day studying the stack and 3 days creating demos and building the solution. My learning repo, which includes references and demos, can be found here:
- [rust_studies](https://github.com/PedroCo3lho/rust_studies/tree/main)

## Did you make any trade-offs for this project? What would you have done differently with more time?
I chose to use Supabase PostgresSQL to avoid additional local database setup. 

## What do you think is the weakest part of your project?

## Is there any other information you’d like us to know?

## References
- [Axum examples](https://github.com/tokio-rs/axum/tree/main/examples)
- [Diesel guides](https://diesel.rs/guides/getting-started)
