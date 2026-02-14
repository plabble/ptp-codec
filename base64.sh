#!/bin/bash

# Check if a number of bytes is passed as an argument
if [ -z "$1" ]; then
    echo "Usage: $0 <number_of_bytes> [hex]"
    exit 1
fi

# Number of bytes to generate
NUM_BYTES=$1

# If second arg is 'hex', output hex; if 'both', output both from same bytes
if [ "$2" = "hex" ]; then
    head -c "$NUM_BYTES" /dev/urandom | hexdump -v -e '1/1 "%02x"'
    echo
    exit 0
fi

if [ "$2" = "both" ]; then
    tmp=$(mktemp)
    head -c "$NUM_BYTES" /dev/urandom >"$tmp"
    echo $(base64 -w 0 "$tmp" | tr '+/' '-_' | tr -d '=')
    echo $(hexdump -v -e '1/1 "%02x"' "$tmp")
    rm -f "$tmp"
    exit 0
fi

# Generate random bytes and encode them in base64URL format (default)
str=$(base64 -w 0 <(head -c "$NUM_BYTES" /dev/urandom) | tr '+/' '-_' | tr -d '=')
echo "$str"
