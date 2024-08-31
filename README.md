# SnapAppDemo
Trabajo Práctico 1 de la materia Métodos y Modelos 2 / Ingeniería de Software 2.

[Enunciado](https://ingenieria-del-software-2.github.io/assignments/statement/2024/2/individual/)

## Descripción
Web API desarrollada en Rust utilizando el framework axum.
Diseñada con arquitectura por capas.

## Desafios
Empezar el proyecto y tener el entorno corriendo no fue tan dificil como lo esperaba en
un principio. Los frameworks están bien documentados y resultaron simple en su uso.

Por otro lado al encontrarse con errores a la hora de adaptar las herramientas del
framework a la solución del problema si hubo complicaciones.
El compilador de Rust suele dar mensajes utiles pero no es el caso
cuando se tratan de errores debido a los frameworks. 
La rigidez del lenguaje + la complejidad del framework hizo que la tarea de 
debuguear fuera complicada. Por suerte hay bastantes recursos online.

Por otro lado actualmente el programa no usa una base de datos sino que
mantiene su estado en memoria. En un futuro se planea utilizar la biblioteca de Rust
_sqlx_ que tiene compatibilidad con axum
y el diseño del código ya da pie a que se agrege esta
funcionalidad. No se llegó a cubrirlo en esta entrega.

## Build
### Requerimientos
* Rust 1.80 o mayor
* Cargo 1.80 o mayor

Para la instalación de estos ver documentación oficial.

### Compilación
Desde el root del repositorio se puede compilar localmente de las siguientes formas:

* Instalación local

`cargo install --path .`

* Instalación en el root del proyecto

`cargo build --release`

Para correrlo:

`cargo run`

### Variables

Por default el programa escucha en el puerto 8080
pero se puede especificar el puerto mediante la variable de entorno `PORT`:

`PORT=3000 cargo run`

## Testing
Para correr los tests, mismos requerimientos que para buildear.

Desde el root del repositorio correr:

`cargo test`

Ver la documentación oficial de cargo para mas configuración.

## Docker
La aplicación se puede correr dentro de docker.

Para buildear la imagen:

`docker build -t snap_app_demo:version .`

Para correr la imagen:

`docker run --name snap_app_demo --publish 8080:8080 snap_app_demo:version`

Observación: Puede que la imagen no sea optima.
En un futuro ver de usar `cargo chef` o algo asi.

## Links útiles
(Obviando documentación oficial ya que duh)

* https://www.reddit.com/r/rust/comments/126xeyx/exploring_the_problem_of_faster_cargo_docker/
* https://github.com/joelparkerhenderson/demo-rust-axum
* https://github.com/tokio-rs/axum
* https://spacedimp.com/blog/using-rust-axum-postgresql-and-tokio-to-build-a-blog/


