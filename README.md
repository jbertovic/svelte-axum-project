# svelte-axum-project

TODO:
- (RUST) able to set and get user_id
- (SVELTE) Finish Log in and Log out
- (RUST) authorization middleware function
- (RUST) Add State to hold some arbitrary data to show how it would work

Created from a similar idea as [svelte-tide-project](https://github.com/jbertovic/svelte-tide-project)

# Backend - Rust Axum

- serves front end directory
- adds sessions middleware
- store user_id into session
- api example using authorization

run as `cargo run`

## Middleware added
- Sessions
- Authorization 

# Frontend - Svelte

- navbar with login and logout
- admin page that shows session information
- api fetch example

run as `npm run build`

# Setup

Install the following
- npm 
- rust

Clone the repository
- cd repository
- `npm install` to download all module dependencies