set export

default:
	just --list

@gen version:
	openapi-generator generate -g rust -i https://raw.githubusercontent.com/airbytehq/airbyte/master/airbyte-api/src/main/openapi/config.yaml -o .  --additional-properties packageName=airbyte,packageVersion=$version
	head -n -5 README.md > README2.md
	mv README2.md README.md
	sed -i 's/Option<Option/Option<OperatorNormalizationOption/g' src/models/operator_normalization.rs
	sed -i 's/Option {/OperatorNormalizationOption {/g' src/models/operator_normalization.rs
	cargo fmt
