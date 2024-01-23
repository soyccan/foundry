CREATE TABLE "etherscan_source_code" (
	"id" serial PRIMARY KEY,
	"date_fetched" date NOT NULL DEFAULT now(),
	"bytecode_hash" text NOT NULL,
	"address" text UNIQUE NOT NULL,
	"source_code_file" text,
	"source_code_files" jsonb,
	"source_code_language" text,
	"source_code_settings" jsonb,
	"abi" jsonb NOT NULL,
	"contract_name" text NOT NULL,
	"compiler_version" text NOT NULL,
	"optimization_used" integer NOT NULL,
	"runs" integer NOT NULL,
	"constructor_arguments" bytea NOT NULL,
	"evm_version" text NOT NULL,
	"library" text NOT NULL,
	"license_type" text NOT NULL,
	"proxy" boolean NOT NULL,
	"implementation" text,
	"swarm_source" text NOT NULL
	CONSTRAINT "etherscan_source_code_address" CHECK ("address" ~ '^0x[0-9a-f]{40}$')
	CONSTRAINT "etherscan_source_code_implementation" CHECK (
		"implementation" IS NULL OR "implementation" ~ '^0x[0-9a-f]{40}$'
	)
	CONSTRAINT "etherscan_source_code_bytecode_hash" CHECK ("bytecode_hash" ~ '^0x[0-9a-f]{64}$')
);

CREATE INDEX "etherscan_source_code_bytecode_hash" ON "etherscan_source_code" ("bytecode_hash");

CREATE TABLE "no_source_code" (
	"bytecode_hash" text PRIMARY KEY
	CONSTRAINT "no_source_code_bytecode_hash" CHECK ("bytecode_hash" ~ '^0x[0-9a-f]{64}$')
);
