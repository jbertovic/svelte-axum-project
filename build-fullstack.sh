#!/usr/bin/env bash

# this script concurrently builds the frontend and backend. if either fails, the errors are logged.
# this uses `script` to capture color of output and it writes the status to a file to capture error codes since `script`` does not return that.
# stopping this script will orphan either build, so they will still consume system resources until finished.

# if you wish to change to cargo run, the server will continue to run and still take up the port.
# see https://stackoverflow.com/questions/11583562/how-to-kill-a-process-running-on-particular-port-in-linux
# kill $(lsof -t -i:8080) (add -9 for extra power)

npm_output=$(mktemp)
cargo_output=$(mktemp)
npm_status=$(mktemp)
cargo_status=$(mktemp)

# run in background, capture outputs, error codes
script -q -c "cd front_end && npm i && npm run build; echo \$? > $npm_status" /dev/null > $npm_output 2> $npm_output
npm_pid=$!

script -q -c "cargo build; echo \$? > $cargo_status" /dev/null > $cargo_output 2> $cargo_output
cargo_pid=$!

wait $npm_pid
npm_exit_status=$(cat $npm_status)

# log if something failed
if [ $npm_exit_status -ne 0 ]; then
    echo "Frontend build failed:"
    cat $npm_output
fi

wait $cargo_pid
cargo_exit_status=$(cat $cargo_status)

if [ $cargo_exit_status -ne 0 ]; then
    echo "Backend build failed:"
    cat $cargo_output
fi

rm $npm_output
rm $cargo_output

if [ $npm_exit_status -eq 0 ] && [ $cargo_exit_status -eq 0 ]; then
    echo "Build success"
fi