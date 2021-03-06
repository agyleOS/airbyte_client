set export

default:
	just --list

@gen version:
	openapi-generator generate -g rust -i https://raw.githubusercontent.com/airbytehq/airbyte/master/airbyte-api/src/main/openapi/config.yaml -o .  -p packageName=airbyte_client,packageVersion=$version
	head -n -5 README.md > README2.md
	mv README2.md README.md

	sed -i 's/edition = "2018"/edition = "2021"\ndescription = "Airbyte Client is an unofficial client library for the Airbyte API"\nlicense = "MIT"\ndocumentation = "https:\/\/docs.rs\/crate\/airbyte_client\/latest"\nhomepage = "https:\/\/github.com\/agyleOS\/airbyte_client"\nrepository = "https:\/\/github.com\/agyleOS\/airbyte_client"/g' Cargo.toml

	sed -i 's/Option<Option/Option<OperatorNormalizationOption/g' src/models/operator_normalization.rs
	sed -i 's/Option {/OperatorNormalizationOption {/g' src/models/operator_normalization.rs

	sed -i 's/Option<Box<crate::models::AttemptStats>>/Option<crate::models::AttemptStats>/g' src/models/attempt_stream_stats.rs
	sed -i 's/stats: Box::new(stats)/stats/g' src/models/attempt_stream_stats.rs

	sed -i 's/Option<Box<crate::models::ResourceRequirements>>/Option<crate::models::ResourceRequirements>/g' src/models/job_type_resource_limit.rs
	sed -i 's/resource_requirements: Box::new(resource_requirements)/resource_requirements/g' src/models/job_type_resource_limit.rs

	cargo fmt
