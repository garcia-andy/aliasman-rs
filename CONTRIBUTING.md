# Pautas de contribución de AliasMan

¡Gracias por su interés en mejorar AliasMan-Rust (también conocido como AMRS)
! Nos encantaría contar con su contribución. Esperamos que todos los colaboradores
cumplan con el [código de conducta de Rust], que puede encontrar en ese enlace o en el archivo
[`CODE_OF_CONDUCT.md`] en este repositorio.

[Código de conducta de Rust]: https://www.rust-lang.org/policies/code-of-conduct
[`CODE_OF_CONDUCT.md`]: https://github.com/garcia-andy/aliasman-rs/blob/main/CODE_OF_CONDUCT.md

## Licencia

AMRS está bajo la licencia MIT, al igual que todas
las contribuciones. Consulte los archivos [`LICENSE.md`] en este directorio para obtener más detalles.

[`LICENSE.md`]: https://github.com/garcia-andy/aliasman-rs/blob/main/LICENSE.md

## Solicitudes de incorporación de cambios

Para realizar cambios en AMRS, envíe solicitudes de incorporación de cambios en GitHub a la rama `main`. Las revisaremos y las fusionaremos o solicitaremos cambios. Github Actions también prueba todo, por lo que también puede recibir comentarios al respecto.

Si realiza adiciones u otros cambios en una solicitud de incorporación de cambios, no dude en modificar las confirmaciones
anteriores o solo agregar nuevas, como prefiera. Es posible que le pidamos que elimine
sus confirmaciones antes de fusionarlas, según corresponda.

## Lista de Issues

Puede encontrar la lista de Issues en [
GitHub](https://github.com/garcia-andy/aliasman-rs/issues). Si ha encontrado un
problema con AMRS, abra un problema allí.

Usamos las siguientes etiquetas:

* `enhancement`: Esto es para cualquier solicitud de nuevas secciones o funcionalidades.
* `bug`: Esto es para cualquier cosa que esté en AMRS, pero que sea incorrecta o no funcione.
* `discussion`: Una discusión sobre cómo mejorar algo en AMRS; esto puede llevar a nuevas mejoras o problemas de errores.

## Flujo de trabajo de desarrollo

Para compilar AMRS, [instale Rust +1.80.1](https://www.rust-lang.org/tools/install), y luego:

```bash
$ git clone https://github.com/garcia-andy/aliasman-rs
$ cd aliasman-rs
$ cargo build
```

Para ejecutar las pruebas:

```bash
$ cargo test
```