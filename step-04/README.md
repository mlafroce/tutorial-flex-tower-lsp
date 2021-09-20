* Lenguaje: test

* Definiciones:
    * `PALABRA`: una secuencia de caracteres alfabeticos
    * `NUMERO`: una secuencia de digitos
    * `ESPACIO`: una secuencia de espacios en blanco

Este programa recibe texto por STDIN y extrae secuencias de tipo `PALABRA`, `NUMERO` y `ESPACIO`. En este paso integramos el lexer a código rust.

Compilar con `make`, requiere tener instalado flex.

Ejemplo de ejecución:

```{.bash}
cargo run < test.md
```
