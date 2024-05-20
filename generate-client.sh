cargo run --features runtime-env --manifest-path actix-server/Cargo.toml --bin redoc_ci > ./generated-openapi-client/openapi.json
cd generated-openapi-client
npx @openapitools/openapi-generator-cli generate -i openapi.json -g rust -c ./openapi-generator.yaml -o ./ --skip-validate-spec
