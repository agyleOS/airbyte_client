set export

default:
	just --list

@gen version:
	openapi-generator generate -g rust -i https://raw.githubusercontent.com/airbytehq/airbyte/master/airbyte-api/src/main/openapi/config.yaml -o .  -p packageName=airbyte_client,packageVersion=$version
	head -n -5 README.md > README2.md
	mv README2.md README.md

	sed -i 's/description = ".*"//g' Cargo.toml
	sed -i 's/edition = "2018"/edition = "2021"\ndescription = "Airbyte Client is an unofficial client library for the Airbyte API"\ndocumentation = "https:\/\/docs.rs\/crate\/airbyte_client\/latest"\nhomepage = "https:\/\/github.com\/agyleOS\/airbyte_client"\nrepository = "https:\/\/github.com\/agyleOS\/airbyte_client"/g' Cargo.toml

	sed -i 's/Option<Option/Option<OperatorNormalizationOption/g' src/models/operator_normalization.rs
	sed -i 's/Option {/OperatorNormalizationOption {/g' src/models/operator_normalization.rs

	cargo fmt
