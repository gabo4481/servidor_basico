CREATE DATABASE IF NOT EXISTS farmacias_db;
USE farmacias_db;

CREATE TABLE farmacias (
	id SERIAL PRIMARY KEY,
	nombre varchar(100) NOT NULL,
	direccion TEXT NOT NULL,
	telefono varchar(20)
);

CREATE TABLE medicamentos (
	id SERIAL PRIMARY KEY,
	nombre varchar(100) NOT NULL,
	principio_activo varchar(50) NOT NULL,
	presentacion varchar(50) NOT NULL,
	precio float NOT NULL
);