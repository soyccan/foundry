-- This file should undo anything in `up.sql`

-- drop the unique constraint on bytecode_hash
ALTER TABLE "etherscan_source_code"
  DROP CONSTRAINT "etherscan_source_code_bytecode_hash_key";

-- add the unique constraint on address
ALTER TABLE "etherscan_source_code"
  ADD CONSTRAINT "etherscan_source_code_address_key" UNIQUE ("address");

-- create the index on bytecode_hash
CREATE INDEX "etherscan_source_code_bytecode_hash" ON "etherscan_source_code" ("bytecode_hash");
