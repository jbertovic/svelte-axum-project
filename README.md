# svelte-axum-project

TODO:
- (RUST) Add CORS
- (JS) test livereload again
- (RUST) Add Sessions
    able to set user_id
- (SVELTE) Finish Log in and Log out
- (RUST) Add State to hold some arbitrary data to show how it would work

Created from a similar idea as [svelte-tide-project](https://github.com/jbertovic/svelte-tide-project)

Used examples from Axum project to help with setup, especially [Session](https://github.com/tokio-rs/axum/tree/main/examples/sessions).

# Backend - Rust Axum

- created with State with only `user_id` set

run as `cargo run`

## Middleware added
- CORS
- Sessions

# Frontend - Svelte

run as `npm run build`

# Setup

Install the following
- npm 
- rust

Clone the repository
- cd repository
- npm install to download all module dependencies