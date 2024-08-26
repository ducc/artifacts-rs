#!/usr/bin/env zsh

set -e

echo "Downloading latest openapi.json"
curl https://api.artifactsmmo.com/openapi.json | jq > openapi.json

echo "Generating openapi types"
npx @openapitools/openapi-generator-cli generate -i openapi.json -g rust-server

echo "Delete examples"
rm -rf examples/

echo "Fixing parameters named param_r#type"
sed -i -e 's/param_r#type/param_type/g' src/**/*.rs

echo "Formatting"
cargo fmt

echo "Removing range validators"
sed -i -e "s/#\[validate(range(.*))\]//g" src/**/*.rs

echo "Allow strings as numbers in required quantity fields"
sd '#\[serde\(rename = "quantity"\)\]\n\s+pub quantity: u' '#[serde(rename = "quantity", deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]\n    pub quantity: u' src/models.rs

echo "Derive default for models"
sed -i -e "s/#\[derive(Debug, Clone/#[derive(Debug, Clone, Default/g" src/models.rs

echo "Debug print errors"
sed -i -e "s/Err(_)/Err(e)/g" src/server/mod*.rs
sed -i -e "s/\/\/ Application code/dbg!(e);\n\/\/ Application code/g" src/server/mod*.rs

echo "Formatting again"
cargo fmt

echo "Checking"
cargo check

echo "Cleanup"
rm openapi.json