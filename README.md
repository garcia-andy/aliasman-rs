# ALIASMAN-RS

Estado actual:

[![CI](https://github.com/garcia-andy/aliasman-rs/actions/workflows/main.yml/badge.svg)](https://github.com/garcia-andy/aliasman-rs/actions/workflows/main.yml)

Licensias:

![Crates.io License](https://img.shields.io/crates/l/aliasman)
![GitHub License](https://img.shields.io/github/license/garcia-andy/aliasman-rs)

🌟 Gracias a todos

![Crates.io Total Downloads](https://img.shields.io/crates/d/aliasman)
![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/aliasman)

![Dynamic TOML Badge](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Fgarcia-andy%2Faliasman-rs%2Fmain%2FCargo.toml&query=%24.package.version&style=flat&label=In%20Dev%20Version)
[![Crates.io](https://img.shields.io/crates/v/aliasman.svg)](https://crates.io/crates/aliasman)

## Estado en GitHub:
![GitHub User's stars](https://img.shields.io/github/stars/garcia-andy?style=flat)
![GitHub watchers](https://img.shields.io/github/watchers/garcia-andy/aliasman-rs?style=flat)
[![Lines of code](https://tokei.rs/b1/github/garcia-andy/aliasman-rs?category=code)](https://github.com/garcia-andy/aliasman-rs)
![Crates.io Size](https://img.shields.io/crates/size/aliasman)
![GitHub repo size](https://img.shields.io/github/repo-size/garcia-andy/aliasman-rs)
![GitHub commit activity](https://img.shields.io/github/commit-activity/t/garcia-andy/aliasman-rs)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues-pr/garcia-andy/aliasman-rs)
![Crates.io Dependents](https://img.shields.io/crates/dependents/aliasman)
![GitHub Created At](https://img.shields.io/github/created-at/garcia-andy/aliasman-rs)
![GitHub contributors](https://img.shields.io/github/contributors/garcia-andy/aliasman-rs)
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/garcia-andy/aliasman-rs/main)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/garcia-andy/aliasman-rs/main.yml)
[![Docs.rs](https://img.shields.io/docsrs/aliasman)](https://docs.rs/crate/aliasman)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/garcia-andy/aliasman-rs/total)
![GitHub Release](https://img.shields.io/github/v/release/garcia-andy/aliasman-rs?style=flat)
![GitHub Tag](https://img.shields.io/github/v/tag/garcia-andy/aliasman-rs)


Maneja tus alias de forma fácil y rapida.

## Que hay de nuevo en la 1.2?

En esta última versión he puesto el foco en:

- Mayor estabilidad
- Estructurar el proyecto para futuras actualizaciones
- Mejorar la experiencia de usuario
- Agregar nuevas características

## Que se quiere agregar en futuras actualizaciones?

Ahora el proyecto está creciendo rápidamente, y era necesario un paso de estructurar bien todo el código (todavía en curso). Pero en futuras actualizaciones preveo implementar:

- Posibles flags para optimizaciones o para habilitar características
- Importar lista de alias desde un json
- Exportar la lista de alias hacia un json
- Posible registro de listas de alias, junto a un comando para subir la tuya ahí. ( ⭐ Posible nuevo proyecto! )

## Cómo usarse 

El funcionamiento de AliasMan se basa en estos comandos:

1. ➕ Add: para agregar un alias no existente
```shell
aliasman add hello echo "Hello World"
```
2. 🔄 Remove o Rm: para quitar un alias previamente creado
```shell
aliasman remove hello
aliasman rm hello
```
3. 📝 Replace o Edit: para cambiar el comando a ejecutar de un alias
```shell
aliasman replace hello echo "Hello!"
aliasman edit hello echo "Hello!"
```
4. 📑 List: muestra un listado de todos los alias
```shell
aliasman list
```
5. ⭐ Update: descarga el archivo de configuración desde github (ahora ya no se tendrá que actualizar el paquete para añadir soporte a otras terminales)
```shell
aliasman update
```
6. 🆙 Upgrade: descarga la última versión desde github y actualiza el binario
```shell
aliasman upgrade
```
7. ⏩ Prompt: ejecuta varios comandos seguidos utilizando el modo prompt
```shell
aliasman prompt
```

## Soporte de terminales

#### De momento aliasman esta testeado en:
1. bash | sh
2. zsh
3. fish

### Estamos trabajando activamente para añadir más terminales

## Contribuir
¿Encontraste un problema o tienes alguna sugerencia? 
Siéntete libre de abrir una issue o si necesita
más información lea [CONTRIBUTING.md].

[CONTRIBUTING.md]: https://github.com/garcia-andy/aliasman-rs/blob/master/CONTRIBUTING.md
