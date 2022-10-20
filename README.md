# New Version 0.2.0

- Will create breaking changes since I modifed the folder structure to use Cargo Workspaces
- Front end and Back end are in two different project folders instead of just different src folders
- Workspaces now allow you to add Rust code independent of the back end server setup
- Also refactored some of the setup out of main.rs into a server.rs file

# Help
- If anyone has ideas of how to implement a build file that would handle the npm script build on the front end and the rust cargo run together, let me know.

# svelte-axum-project

Starting project template for Rust Axum backend and Svelte frontend.

Created from a similar idea as [svelte-tide-project](https://github.com/jbertovic/svelte-tide-project)

![](capture_localhost.gif)

# Template
## Using Cargo
- Must have cargo generate installed: `cargo install cargo-generate`
- Then use `cargo generate jbertovic/svelte-axum-project -n <your-project-name>`

## Using git template
- you can also just hit the "use this template" button in green on top of the repo
- if you have gh cli installed check out `--template` option

# Back end - Rust Axum
- located in `./back_end`
- serves front end directory
- middleware for checking authorization header
- middleware for checking session that user exists
- store example that holds token secret for authorization
- /api route example using authorization header
- /secure route example using sessions for authorization

Note there is no persistance beyond what's held in memory while the application is running

run as `cargo run` from parent directory and not needed to run inside `./back_end` folder

# Front end - Svelte
- Located in `./front_end`
- navbar with login and logout
- secure page that shows session information once logged in
- api fetch example, log in not required

run as `npm run build` from inside the `./front_end` directory

# Setup

Install the following
NodeJs - [Install](https://nodejs.org/en/download/)
Rust  - [Install](https://www.rust-lang.org/tools/install)

Clone the repository
- cd repository
- inside the `./front_end` folder run `npm install` to download all module dependencies inside root directory of project
- inside the `./front_end` folder run `npm run build` to bundle the js/svelte code into public folder
- inside the top level folder run `cargo run` to start the the server
- access in browser at http://localhost:8080/
