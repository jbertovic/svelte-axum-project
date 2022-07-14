# svelte-axum-project

TODO:
- Add API access check from svelte

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
- `npm install` to download all module dependencies inside root directory of project
- `npm run build` to bundle the js/svelte code into public folder
- `cargo run` to run the server
- access in browser at http://localhost:8080/