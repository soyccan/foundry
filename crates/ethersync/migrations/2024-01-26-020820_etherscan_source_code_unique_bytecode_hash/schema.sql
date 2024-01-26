--
-- PostgreSQL database dump
--

-- Dumped from database version 16.0 (Debian 16.0-1.pgdg120+1)
-- Dumped by pg_dump version 16.0 (Debian 16.0-1.pgdg120+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: etherscan_source_code; Type: TABLE; Schema: public; Owner: proxychecker
--

CREATE TABLE public.etherscan_source_code (
    id integer NOT NULL,
    date_fetched date DEFAULT now() NOT NULL,
    bytecode_hash text NOT NULL,
    address text NOT NULL,
    source_code_file text,
    source_code_files jsonb,
    source_code_language text,
    source_code_settings jsonb,
    abi jsonb NOT NULL,
    contract_name text NOT NULL,
    compiler_version text NOT NULL,
    optimization_used integer NOT NULL,
    runs integer NOT NULL,
    constructor_arguments bytea NOT NULL,
    evm_version text NOT NULL,
    library text NOT NULL,
    license_type text NOT NULL,
    proxy boolean NOT NULL,
    implementation text,
    swarm_source text NOT NULL,
    CONSTRAINT etherscan_source_code_address CHECK ((address ~ '^0x[0-9a-f]{40}$'::text)),
    CONSTRAINT etherscan_source_code_bytecode_hash CHECK ((bytecode_hash ~ '^0x[0-9a-f]{64}$'::text)),
    CONSTRAINT etherscan_source_code_implementation CHECK (((implementation IS NULL) OR (implementation ~ '^0x[0-9a-f]{40}$'::text)))
);


ALTER TABLE public.etherscan_source_code OWNER TO proxychecker;

--
-- Name: etherscan_source_code_id_seq; Type: SEQUENCE; Schema: public; Owner: proxychecker
--

CREATE SEQUENCE public.etherscan_source_code_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.etherscan_source_code_id_seq OWNER TO proxychecker;

--
-- Name: etherscan_source_code_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: proxychecker
--

ALTER SEQUENCE public.etherscan_source_code_id_seq OWNED BY public.etherscan_source_code.id;


--
-- Name: etherscan_source_code id; Type: DEFAULT; Schema: public; Owner: proxychecker
--

ALTER TABLE ONLY public.etherscan_source_code ALTER COLUMN id SET DEFAULT nextval('public.etherscan_source_code_id_seq'::regclass);


--
-- Name: etherscan_source_code etherscan_source_code_bytecode_hash_key; Type: CONSTRAINT; Schema: public; Owner: proxychecker
--

ALTER TABLE ONLY public.etherscan_source_code
    ADD CONSTRAINT etherscan_source_code_bytecode_hash_key UNIQUE (bytecode_hash);


--
-- Name: etherscan_source_code etherscan_source_code_pkey; Type: CONSTRAINT; Schema: public; Owner: proxychecker
--

ALTER TABLE ONLY public.etherscan_source_code
    ADD CONSTRAINT etherscan_source_code_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

