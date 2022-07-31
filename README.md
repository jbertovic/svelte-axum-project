# svelte-axum-project

Starting project template for Rust Axum backend and Svelte frontend.

Created from a similar idea as [svelte-tide-project](https://github.com/jbertovic/svelte-tide-project)

![](capture_localhost.gif)

# Backend - Rust Axum
- located in `./src`
- serves front end directory
- middleware for checking authorization header
- middleware for checking session that user exists
- store example that holds token secret for authorization
- /api route example using authorization header
- /secure route example using sessions for authorization

Note there is no persistance beyond what's held in memory while the application is running

run as `cargo run`

# Frontend - Svelte
- Located in `./src_front`
- navbar with login and logout
- secure page that shows session information once logged in
- api fetch example, log in not required

run as `npm run build`

# Setup

Install the following
NodeJs - [Install](https://nodejs.org/en/download/)
Rust  - [Install](https://www.rust-lang.org/tools/install)

Clone the repository
- cd repository
- `npm install` to download all module dependencies inside root directory of project
- `npm run build` to bundle the js/svelte code into public folder
- `cargo run` to run the server
- access in browser at http://localhost:8080/