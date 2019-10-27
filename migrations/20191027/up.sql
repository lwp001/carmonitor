-- Database generated with pgModeler (PostgreSQL Database Modeler).
-- pgModeler  version: 0.9.2-beta1
-- PostgreSQL version: 11.0
-- Project Site: pgmodeler.io
-- Model Author: ---

-- object: gb17691 | type: ROLE --
-- DROP ROLE IF EXISTS gb17691;
-- CREATE ROLE gb17691 WITH 
-- 	CREATEROLE
-- 	LOGIN
-- 	UNENCRYPTED PASSWORD 'gb17691';
-- ddl-end --


-- Database creation must be done outside a multicommand file.
-- These commands were put in this file only as a convenience.
-- -- object: gb17691 | type: DATABASE --
-- -- DROP DATABASE IF EXISTS gb17691;
-- CREATE DATABASE gb17691;
-- -- ddl-end --
-- 

-- object: public.engine | type: TABLE --
-- DROP TABLE IF EXISTS public.engine CASCADE;
CREATE TABLE IF NOT EXISTS public.engine (
	id bigserial NOT NULL,
	vin text NOT NULL,
	utc timestamptz NOT NULL,
	sno integer NOT NULL DEFAULT 0,
	speed real NOT NULL,
	atmospheric_pressure real NOT NULL,
	real_torque smallint NOT NULL,
	friction_torque smallint NOT NULL,
	rotate real NOT NULL,
	fuel_flow real NOT NULL,
	scr_up_nox real NOT NULL,
	scr_down_nox real NOT NULL,
	reagent_tail real NOT NULL,
	air_inflow real NOT NULL,
	scr_entrance_temperature real NOT NULL,
	scr_export_temperature real NOT NULL,
	dpf_diference_pressure real NOT NULL,
	coolant_temperature smallint NOT NULL,
	oil_level real NOT NULL,
	gps_status smallint NOT NULL,
	longitude real NOT NULL,
	latitude real NOT NULL,
	mileage real NOT NULL,
	ext_data jsonb,
	created timestamptz NOT NULL DEFAULT now(),
	CONSTRAINT engine_pk PRIMARY KEY (id)

);
-- ddl-end --
COMMENT ON COLUMN public.engine.air_inflow IS 'kg/h';
-- ddl-end --
COMMENT ON COLUMN public.engine.ext_data IS '{id:1,p1:d1...}';
-- ddl-end --
ALTER TABLE public.engine OWNER TO gb17691;
-- ddl-end --

-- object: public.obd | type: TABLE --
-- DROP TABLE IF EXISTS public.obd CASCADE;
CREATE TABLE IF NOT EXISTS public.obd (
	id bigserial NOT NULL,
	vin text NOT NULL,
	utc timestamptz NOT NULL,
	sno integer NOT NULL DEFAULT 0,
	protocal smallint NOT NULL,
	mil_lamp smallint NOT NULL,
	diagnose_support smallint NOT NULL,
	diagnose_status smallint NOT NULL,
	obd_vin text,
	software_id text,
	calibration_code text,
	iupr text,
	fault_count smallint NOT NULL,
	faults text,
	created timestamptz NOT NULL DEFAULT now(),
	CONSTRAINT obd_pk PRIMARY KEY (id)

);
-- ddl-end --
ALTER TABLE public.obd OWNER TO gb17691;
-- ddl-end --

-- object: public.login_logout_records | type: TABLE --
-- DROP TABLE IF EXISTS public.login_logout_records CASCADE;
CREATE TABLE IF NOT EXISTS public.login_logout_records (
	id uuid NOT NULL,
	vin text NOT NULL,
	login_time timestamptz NOT NULL,
	login_serial integer NOT NULL,
	iccid text,
	logout_time timestamptz,
	logout_serial integer,
	created timestamptz NOT NULL DEFAULT now(),
	updated timestamptz,
	CONSTRAINT login_logout_records_pk PRIMARY KEY (id)

);
-- ddl-end --
ALTER TABLE public.login_logout_records OWNER TO gb17691;
-- ddl-end --

-- object: public.vehicle | type: TABLE --
-- DROP TABLE IF EXISTS public.vehicle CASCADE;
CREATE TABLE IF NOT EXISTS public.vehicle (
	id uuid NOT NULL,
	vin text NOT NULL,
	lpn text NOT NULL,
	lpn_color smallint,
	org_id uuid NOT NULL,
	model_id uuid,
	division_code text,
	lpn_classify smallint,
	trade_classify smallint,
	service_start_date date,
	service_end_date date,
	note text,
	status smallint NOT NULL DEFAULT 1,
	created timestamptz NOT NULL DEFAULT now(),
	CONSTRAINT vehicle_pk PRIMARY KEY (id),
	CONSTRAINT vin_unique UNIQUE (vin),
	CONSTRAINT lpn_unique UNIQUE (lpn)

);
-- ddl-end --
COMMENT ON COLUMN public.vehicle.lpn IS 'license plate number';
-- ddl-end --
COMMENT ON COLUMN public.vehicle.lpn_color IS 'license plate color 2 yellow';
-- ddl-end --
COMMENT ON COLUMN public.vehicle.org_id IS 'organization id';
-- ddl-end --
COMMENT ON COLUMN public.vehicle.model_id IS '车型 vehicle models id';
-- ddl-end --
COMMENT ON COLUMN public.vehicle.division_code IS '行政区划代码';
-- ddl-end --
COMMENT ON COLUMN public.vehicle.lpn_classify IS '车牌分类';
-- ddl-end --
COMMENT ON COLUMN public.vehicle.trade_classify IS '行业分类';
-- ddl-end --
COMMENT ON COLUMN public.vehicle.service_start_date IS '服务开始时间';
-- ddl-end --
COMMENT ON COLUMN public.vehicle.service_end_date IS '服务结束时间';
-- ddl-end --
COMMENT ON COLUMN public.vehicle.status IS '1 nomal  0 freeze';
-- ddl-end --
ALTER TABLE public.vehicle OWNER TO gb17691;
-- ddl-end --

-- object: public.sys_organize | type: TABLE --
-- DROP TABLE IF EXISTS public.sys_organize CASCADE;
CREATE TABLE IF NOT EXISTS public.sys_organize (
	id uuid NOT NULL,
	name text NOT NULL,
	parent_id uuid NOT NULL DEFAULT '',
	parent_ids_path text NOT NULL DEFAULT '/',
	code text,
	vehicle_count int4 NOT NULL DEFAULT 0,
	linker text,
	phone text,
	logo text,
	address text,
	email text,
	note text,
	business_certificate text,
	status integer NOT NULL DEFAULT 1,
	created timestamptz NOT NULL DEFAULT now(),
	CONSTRAINT sys_organize_pk PRIMARY KEY (id),
	CONSTRAINT name_unique UNIQUE (name)

);
-- ddl-end --
COMMENT ON COLUMN public.sys_organize.business_certificate IS '经营许可证';
-- ddl-end --
COMMENT ON COLUMN public.sys_organize.status IS ' 1 normal  0 freeze';
-- ddl-end --

-- object: public.sys_user | type: TABLE --
-- DROP TABLE IF EXISTS public.sys_user CASCADE;
CREATE TABLE IF NOT EXISTS public.sys_user (
	id uuid NOT NULL,
	account text NOT NULL,
	password text NOT NULL,
	name text NOT NULL,
	org_id uuid NOT NULL,
	sex smallint DEFAULT 0,
	phone text,
	photo text,
	email text,
	note text,
	status smallint NOT NULL DEFAULT 1,
	created timestamptz NOT NULL DEFAULT now(),
	CONSTRAINT sys_user_pk PRIMARY KEY (id),
	CONSTRAINT account_unique UNIQUE (account)

);
-- ddl-end --
COMMENT ON COLUMN public.sys_user.sex IS '0  man  1 women';
-- ddl-end --

-- object: public.sys_relation | type: TABLE --
-- DROP TABLE IF EXISTS public.sys_relation CASCADE;
CREATE TABLE IF NOT EXISTS public.sys_relation (
	id serial NOT NULL,
	user_id uuid NOT NULL,
	resource_id uuid NOT NULL,
	type smallint,
	created timestamptz NOT NULL DEFAULT now()
);
-- ddl-end --
COMMENT ON COLUMN public.sys_relation.resource_id IS '资源ID';
-- ddl-end --
COMMENT ON COLUMN public.sys_relation.type IS '1 user~role;2 user~org; 3 role~permission;4 menu~btnfun';
-- ddl-end --
ALTER TABLE public.sys_relation OWNER TO postgres;
-- ddl-end --

-- object: org_fk | type: CONSTRAINT --
-- ALTER TABLE public.vehicle DROP CONSTRAINT IF EXISTS org_fk CASCADE;
ALTER TABLE public.vehicle ADD CONSTRAINT org_fk FOREIGN KEY (org_id)
REFERENCES public.sys_organize (id) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: uer_org_fk | type: CONSTRAINT --
-- ALTER TABLE public.sys_user DROP CONSTRAINT IF EXISTS uer_org_fk CASCADE;
ALTER TABLE public.sys_user ADD CONSTRAINT uer_org_fk FOREIGN KEY (org_id)
REFERENCES public.sys_organize (id) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: fk_owner | type: CONSTRAINT --
-- ALTER TABLE public.sys_relation DROP CONSTRAINT IF EXISTS fk_owner CASCADE;
ALTER TABLE public.sys_relation ADD CONSTRAINT fk_owner FOREIGN KEY (user_id)
REFERENCES public.sys_user (id) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: fk_resource_org | type: CONSTRAINT --
-- ALTER TABLE public.sys_relation DROP CONSTRAINT IF EXISTS fk_resource_org CASCADE;
ALTER TABLE public.sys_relation ADD CONSTRAINT fk_resource_org FOREIGN KEY (resource_id)
REFERENCES public.sys_organize (id) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --


