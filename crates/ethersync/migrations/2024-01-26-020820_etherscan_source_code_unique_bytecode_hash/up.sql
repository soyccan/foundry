-- This migration uniquifies the bytecode_hash column in the etherscan_source_code table.

-- drop the unique constraint on address
ALTER TABLE "etherscan_source_code"
  DROP CONSTRAINT "etherscan_source_code_address_key";

-- drop the index on bytecode_hash
DROP INDEX "etherscan_source_code_bytecode_hash";

-- drop duplicate rows on bytecode_hash
WITH t AS (
  SELECT
    id,
    row_number() OVER (PARTITION BY bytecode_hash ORDER BY id) AS rn
  FROM "etherscan_source_code"
)
DELETE FROM "etherscan_source_code"
  WHERE id IN (SELECT id FROM t WHERE rn != 1);

-- add the unique constraint on bytecode_hash
ALTER TABLE "etherscan_source_code"
  ADD CONSTRAINT "etherscan_source_code_bytecode_hash_key" UNIQUE ("bytecode_hash");
