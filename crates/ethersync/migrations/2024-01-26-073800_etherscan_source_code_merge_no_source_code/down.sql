-- This file should undo anything in `up.sql`

ALTER TABLE "etherscan_source_code" DROP COLUMN "verified";

ALTER TABLE "etherscan_source_code" ALTER COLUMN "abi" SET NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "contract_name" SET NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "compiler_version" SET NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "optimization_used" SET NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "runs" SET NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "constructor_arguments" SET NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "evm_version" SET NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "library" SET NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "license_type" SET NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "proxy" SET NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "swarm_source" SET NOT NULL;

CREATE TABLE "no_source_code" (
	"bytecode_hash" text PRIMARY KEY
	CONSTRAINT "no_source_code_bytecode_hash" CHECK ("bytecode_hash" ~ '^0x[0-9a-f]{64}$')
);
