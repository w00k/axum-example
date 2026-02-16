# Prueba de Rust y Axum

Este proyecto es una idea de como funciona Axum para microservicios, este es el primer commit y la idea es ir separando por capas el proyecto y sus diferentes funcionalidades.

## Setup
Es necesario:
- Rust + Cargo.
- Docker.
- Cliente para ejecutar el script de base de datos. 
- Bruno para abrir las colecciones. 

1. Inicia docker-compose.yml, este correr el script de base de datos **create_table.sql**.

```bash 
docker compose up 
```
No olvidar bajar docker-compose.yml cuando dejemos de usarlo.
```bash 
docker compose down 
```

2. ingresar a la carpeta **axum** y correr Rust. 

3. Usar las collections de Bruno. 
