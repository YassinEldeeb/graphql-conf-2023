#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Download latest GraphQL Schema Introspection
graphql-client introspect-schema http://127.0.0.1:4000/graphql --output ./schema.json

# Create the directory before generating files
mkdir -p src/generated

# Search for *.graphql files and generate types for them
find ./graphql -iname "*.graphql" -type f | while read -r filename; do
    echo -e "${GREEN}Generating Rust code from $filename...${NC}"
    
    if graphql-client generate --schema-path="./schema.json" --output-directory="src/generated" "$filename" --response-derives Debug; then
        echo -e "${GREEN}Generation complete for $filename.${NC}"
    else
        echo -e "${RED}Generation failed for $filename.${NC}"
    fi
done
