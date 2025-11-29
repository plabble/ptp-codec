#!/bin/bash

# Check if a number of bytes is passed as an argument
if [ -z "$1" ]; then
    echo "Usage: $0 <number_of_bytes>"
    exit 1
fi

# Number of bytes to generate
NUM_BYTES=$1

# Generate random bytes and encode them in base64URL format
str=$(base64 -w 0 <(head -c "$NUM_BYTES" /dev/urandom) | tr '+/' '-_' | tr -d '=')
echo "$str"
