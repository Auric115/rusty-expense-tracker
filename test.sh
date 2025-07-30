#!/bin/bash

cargo build --quiet

echo -e "\n# Adding expenses..."
cargo run --quiet -- add --description "Coffee" --amount 3.50
cargo run --quiet -- add --description "Books" --amount 29.99
cargo run --quiet -- add --description "Lunch" --amount 12.00

echo -e "\n# Listing expenses..."
cargo run --quiet -- list

echo -e "\n# Showing full summary..."
cargo run --quiet -- summary

echo -e "\n# Showing summary for July..."
cargo run --quiet -- summary --month 7

echo -e "\n# Deleting expense with ID 2..."
cargo run --quiet -- delete --id 2

echo -e "\n# Listing expenses after deletion..."
cargo run --quiet -- list

