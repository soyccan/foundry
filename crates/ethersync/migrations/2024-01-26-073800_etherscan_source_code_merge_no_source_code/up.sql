-- Merge no_source_code into etherscan_source_code by adding a `verified` column

-- add `verified` column to specify whether the source code is verified and available
ALTER TABLE "etherscan_source_code" ADD COLUMN "verified" bool;
UPDATE "etherscan_source_code" SET "verified" = true;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "verified" SET NOT NULL;

-- set columns to nullable for unverified source code
ALTER TABLE "etherscan_source_code" ALTER COLUMN "abi" DROP NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "contract_name" DROP NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "compiler_version" DROP NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "optimization_used" DROP NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "runs" DROP NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "constructor_arguments" DROP NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "evm_version" DROP NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "library" DROP NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "license_type" DROP NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "proxy" DROP NOT NULL;
ALTER TABLE "etherscan_source_code" ALTER COLUMN "swarm_source" DROP NOT NULL;

-- drop no_source_code table
DROP TABLE IF EXISTS "no_source_code";
