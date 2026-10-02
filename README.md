# NanoAgent

**by IMANDRO** — <https://github.com/IMANDRO/NanoAgent>

Agente de IA autónomo para programación, escrito en Rust y optimizado para
consumir pocos recursos.

## Características

- CLI interactivo con lectura de línea con historial
- Múltiples proveedores de modelos (ver `src/providers/`)
- Herramientas integradas: sistema de archivos, shell, búsqueda, HTTP, editor,
  grep, memoria, base de datos, procesos y tareas programadas
- Permisos configurables por herramienta
- Persistencia en SQLite (sin servidor)

## Requisitos

- Rust estable
- Un compilador de C (MSVC Build Tools o MinGW), necesario por `rusqlite`
  con la feature `bundled`

## Uso

```bash
cargo build --release
cp .env.example .env   # y rellena tus claves
./target/release/nano-agent.exe
```

Dentro de la REPL, escribe `/help` para ver los comandos disponibles.

## Configuración

Las variables de entorno se leen de `.env`. Consulta `.env.example` para la
lista de proveedores soportados.

## Licencia

MIT
