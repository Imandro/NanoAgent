use crate::permissions::PermissionLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Language,
    Role,
    Workflow,
    Autonomy,
}

impl Category {
    pub fn label(&self) -> &'static str {
        match self {
            Category::Language => "Lenguaje",
            Category::Role => "Rol",
            Category::Workflow => "Flujo de trabajo",
            Category::Autonomy => "Autonomia",
        }
    }
}

pub struct Skill {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub category: Category,
    pub instructions: &'static str,
    pub tools: &'static [&'static str],
    pub keywords: &'static [&'static str],
    pub permission_hints: &'static [(&'static str, PermissionLevel)],
}

pub static SKILLS: &[Skill] = &[
    Skill {
        id: "rust",
        name: "Rust",
        description: "Ownership, borrowing, traits, error handling con Result",
        category: Category::Language,
        instructions: "RUST:
- Prefiere el ownership claro: evita unwrap() y expect() en produccion. Usa ? y Result.
- Clona solo cuando sea necesario; pasa &str en vez de String cuando la API lo admita.
- Los errores se propagan con anyhow::Context para dar contexto al fallar.
- Los traits describen comportamiento; evita herencia de structs.
- No uses RefCell en codigo concurrente: la mutabilidad interior es costosa de revisar.
- Ejecuta cargo build y cargo test tras cada cambio para confirmar que compila.",
        tools: &["shell", "read_file", "edit_file", "grep"],
        keywords: &["rust", "cargo", "trait", "borrow", "lifetime", "rustc", "crate"],
        permission_hints: &[],
    },
    Skill {
        id: "python",
        name: "Python",
        description: "Tipado, asyncio, entornos virtuales y empaquetado",
        category: Category::Language,
        instructions: "PYTHON:
- Anota tipos en funciones publicas. El tipado estatico atrapa errores antes de ejecutar.
- Usa pathlib en vez de os.path.
- Para I/O concurrente usa asyncio; para CPU-bound usa multiprocessing.
- Instala dependencias en un venv, nunca globalmente.
- En el manejo de errores, captura excepciones concretas; nunca un except vacio.
- Formatea con el estilo de la herramienta que ya use el proyecto.",
        tools: &["shell", "read_file", "edit_file", "grep"],
        keywords: &["python", "pip", "django", "flask", "fastapi", "asyncio", "venv"],
        permission_hints: &[],
    },
    Skill {
        id: "typescript",
        name: "TypeScript / JavaScript",
        description: "Node, tipos estrictos y ecosystem web moderno",
        category: Category::Language,
        instructions: "TYPESCRIPT / JAVASCRIPT:
- Activa strict en tsconfig. No uses any; usa unknown y estrecha el tipo.
- Prefiere interfaces y type unions sobre any.
- async/await con manejo explicito de rechazos; evita callbacks anidados.
- Valida datos de entrada en la frontera del sistema (API, archivos).
- En React, no mutes estado; usa el setter funcional cuando dependa del previo.",
        tools: &["shell", "read_file", "edit_file", "grep"],
        keywords: &["typescript", "javascript", "node", "npm", "react", "next", "ts", "jsx"],
        permission_hints: &[],
    },
    Skill {
        id: "csharp",
        name: "C#",
        description: "async/await, nullable reference types y LINQ",
        category: Category::Language,
        instructions: "C#:
- Activa <Nullable>enable</Nullable> y trata los avisos del compilador como errores.
- Usa async/await de extremo a extremo; nunca .Result ni .Wait().
- Prefiere LINQ para transformaciones, pero evita encadenar cadenas largas.
- Dispose los recursos que implementen IDisposable.
- Las injECCiones de dependencias se registran en el contenedor, no se construyen a mano.",
        tools: &["shell", "read_file", "edit_file", "grep"],
        keywords: &["csharp", "dotnet", ".net", "aspnet", "entity framework", "linq", "cs"],
        permission_hints: &[],
    },
    Skill {
        id: "go",
        name: "Go",
        description: "Goroutines, canales e interfaces pequenas",
        category: Category::Language,
        instructions: "GO:
- Las goroutines necesitan contexto: acepta context.Context como primer parametro.
- Cierra los canales solo en el lado que los envia.
- Las interfaces deben ser pequenas: un solo metodo cuando se pueda.
- Formatea siempre con gofmt antes de terminar.
- Ejecuta go test ./... y go vet tras cada cambio.",
        tools: &["shell", "read_file", "edit_file", "grep"],
        keywords: &["go", "golang", "goroutine", "gorm", "gin", "grpc"],
        permission_hints: &[],
    },
    Skill {
        id: "java",
        name: "Java",
        description: "Streams, generics y Spring",
        category: Category::Language,
        instructions: "JAVA:
- Usa streams para colecciones; evita bucles anidados con ifs.
- Los records son la opcion por defecto para datos inmutables (Java 16+).
- En Spring, la inyeccion va por constructor, nunca por campo.
- Cierra los recursos con try-with-resources.
- Evita el boxing en bucles criticos.",
        tools: &["shell", "read_file", "edit_file", "grep"],
        keywords: &["java", "spring", "maven", "gradle", "jvm", "hibernate"],
        permission_hints: &[],
    },
    Skill {
        id: "javascript",
        name: "JavaScript",
        description: "ES modules, promises y el ecosistema npm",
        category: Category::Language,
        instructions: "JAVASCRIPT:
- Usa ES modules (import/export), no require, salvo que el proyecto ya sea CommonJS.
- Todo await necesita try/catch o .catch(): las promesas sin manejar fallan en silencio.
- No compares objetos con ===; compara por propiedad o usa comparacion profunda.
- Los parametros de funciones se pueden reasignar; usa const por defecto.
- Revisa las dependencias antes de anadir otra: el arbol de npm se infla rapido.",
        tools: &["shell", "read_file", "edit_file", "grep"],
        keywords: &["javascript", "js", "node", "npm", "express", "vite", "webpack"],
        permission_hints: &[],
    },
    Skill {
        id: "php",
        name: "PHP",
        description: "Composer, tipado estricto y Laravel/Symfony",
        category: Category::Language,
        instructions: "PHP:
- Declara declare(strict_types=1) en cada archivo.
- Usa PSR-12 y las herramientas de formato que ya use el proyecto.
- El acceso a base de datos va por PDO con consultas preparadas, nunca concatenando.
- Moderniza: enum, readonly, match y tipos union en lugar de sintaxis antigua.
- Instala con Composer, nunca a mano.",
        tools: &["shell", "read_file", "edit_file", "grep"],
        keywords: &["php", "laravel", "symfony", "composer", "wordpress"],
        permission_hints: &[],
    },
    Skill {
        id: "kotlin",
        name: "Kotlin",
        description: "Null safety, coroutines y Android moderno",
        category: Category::Language,
        instructions: "KOTLIN:
- El sistema de tipos ya te protege contra null: no uses !! salvo que sepas que no es null.
- Usa coroutines con viewModelScope; no bloquees el hilo principal.
- Los data class generan equals y hashCode bien; no los reescribas a mano.
- Usa sealed class o sealed interface para estados cerrados en lugar de enums con datos.
- Las extensions function son la forma idiomatica de agregar utileria.",
        tools: &["shell", "read_file", "edit_file", "grep"],
        keywords: &["kotlin", "android", "gradle", "compose", "coroutines"],
        permission_hints: &[],
    },
    Skill {
        id: "swift",
        name: "Swift",
        description: "Optionals, value types y ARC",
        category: Category::Language,
        instructions: "SWIFT:
- Desenvuelve los optionals con guard let de inmediato; no entres a un scope con optionals sin resolver.
- Los structs son el tipo por defecto. Usa class solo cuando necesites identidad o mutabilidad compartida.
- Las closuresescapantes se marcan con @escaping; olvidadarlo es bug de compilacion.
- Evita los force unwraps (!) en produccion.
- Respeta el aislamiento de actors: el trabajo concurrente debe vivir en un actor o Task.detached.",
        tools: &["shell", "read_file", "edit_file", "grep"],
        keywords: &["swift", "ios", "xcode", "uikit", "swiftui", "vapor"],
        permission_hints: &[],
    },
    Skill {
        id: "sql",
        name: "SQL",
        description: "Consultas, indices, joins y migraciones",
        category: Category::Language,
        instructions: "SQL:
- Escribe consultas parametrizadas. Nunca concatentes valores de entrada.
- Evita SELECT *: nombra las columnas para no arrastrar campos que no usas.
- Un JOIN sin ON explicito sobre una columna indexada es la causa mas comun de lentitud.
- Toda columna por la que se filtra o ordena convenientemente debe tener indice.
- Las migraciones deben ser reversibles y no perder datos existentes.
- Cuidado con UPDATE o DELETE sin WHERE: verifica el numero de filas afectadas.",
        tools: &["sql_query", "read_file", "edit_file", "shell"],
        keywords: &["sql", "query", "consulta", "select", "join", "indice", "migracion", "postgres", "mysql", "sqlite"],
        permission_hints: &[("sql_query", PermissionLevel::Ask)],
    },
    Skill {
        id: "shell",
        name: "Shell / Bash",
        description: "Scripts portables, quoting y set -euo pipefail",
        category: Category::Language,
        instructions: "SHELL:
- Empieza todo script con set -euo pipefail.
- Cita siempre las expansiones de variables y de comandos; sin comillas, los espacios rompen el argumento.
- Prefiere Here-documents o heredocs para pasar texto multilinea.
- Usa arrays para construir argumentos; concatenar cadenas rompe con espacios.
- Prefiere find -print0 con bucles -read -d '' para nombres con espacios.
- Un script debe ser idempotente si corre en CI: no asumas que parte de un estado previo.
- No uses eval; ejecuta el comando directamente.",
        tools: &["shell", "read_file", "edit_file", "grep"],
        keywords: &["bash", "shell", "script", "sh", "zsh", "powershell", "batch"],
        permission_hints: &[],
    },
    Skill {
        id: "backend",
        name: "Backend",
        description: "APIs, base de datos y logica de negocio",
        category: Category::Role,
        instructions: "BACKEND:
- Valida toda entrada en el borde del sistema antes de que llegue a la logica.
- Las transacciones de base de datos deben ser explicitas y acotadas.
- Nunca pongas secretos en el codigo ni en los logs. Van en variables de entorno.
- Devuelve errores con codigo y mensaje util, no filtres excepciones internas al cliente.
- Indices de base de datos: revisa que las consultas frecuentes esten cubiertas.
- Maneja los fallos de red con reintentos con backoff, no de inmediato.",
        tools: &["read_file", "write_file", "grep", "sql_query"],
        keywords: &["api", "endpoint", "backend", "rest", "graphql", "microservicio", "controlador", "service layer"],
        permission_hints: &[("sql_query", PermissionLevel::Ask)],
    },
    Skill {
        id: "frontend",
        name: "Frontend",
        description: "UI, estado, accesibilidad y responsive",
        category: Category::Role,
        instructions: "FRONTEND:
- Accesibilidad: todo elemento interactivo necesita rol, foco visible y teclado funcional.
- Responsive desde el inicio; evita anchos fijos.
- Carga lo pesado de forma diferida y optimiza las imagenes.
- Maneja los estados de carga y de error explicitamente en la UI.
- No guardes en el estado del cliente lo que puede venir del servidor.
- Respeta la accesibilidad del color: nunca comuniques informacion solo con color.",
        tools: &["read_file", "write_file", "grep"],
        keywords: &["frontend", "ui", "componente", "css", "react", "vue", "html", "formulario", "accesible"],
        permission_hints: &[],
    },
    Skill {
        id: "data",
        name: "Data",
        description: "Consultas, ETL y analisis",
        category: Category::Role,
        instructions: "DATA:
- Antes de escribir una consulta, revisa el esquema y los volumenes de la tabla.
- Evita SELECT *; nombra las columnas que necesitas.
- Cuidado con los JOINs que multiplican filas y disparan el uso de memoria.
- Las agregaciones sobre columnas sin indice son la causa mas comun de lentitud.
- Valida los datos en la entrada del pipeline, no al final.",
        tools: &["read_file", "sql_query", "shell"],
        keywords: &["datos", "sql", "consulta", "etl", "dataframe", "analisis", "pipeline"],
        permission_hints: &[("sql_query", PermissionLevel::Ask)],
    },
    Skill {
        id: "devops",
        name: "DevOps",
        description: "CI/CD, contenedores, despliegue y observabilidad",
        category: Category::Role,
        instructions: "DEVOPS:
- Los secretos van en el gestor de secretos, nunca en el repositorio ni en la imagen.
- Las imagenes se fijan por digest, no por latest.
- Cada despliegue debe ser reversible: versiona el artefacto.
- El pipeline debe fallar rapido: pon los tests y el lint antes del build lento.
- Incluye logs utiles en los puntos de fallo, sin datos sensibles.",
        tools: &["shell", "read_file", "write_file"],
        keywords: &["docker", "ci", "cd", "deploy", "kubernetes", "terraform", "pipeline", "yml", "infraestructura"],
        permission_hints: &[],
    },
    Skill {
        id: "mobile",
        name: "Mobile",
        description: "Android/iOS, ciclo de vida y rendimiento",
        category: Category::Role,
        instructions: "MOBILE:
- El trabajo pesado va fuera del hilo principal o la UI se congela.
- Respeta el ciclo de vida de la pantalla: guarda el estado al segundo plano.
- Mide antes de optimizar; la intuicion sobre rendimiento casi nunca acierta.
- Reduce el tamano de la app: los recursos sin usar se eliminan.",
        tools: &["read_file", "write_file", "shell"],
        keywords: &["android", "ios", "mobile", "react native", "flutter", "swift", "kotlin"],
        permission_hints: &[],
    },
    Skill {
        id: "fullstack",
        name: "Fullstack",
        description: "Contratos entre frontend y backend",
        category: Category::Role,
        instructions: "FULLSTACK:
- El contrato entre cliente y servidor se define primero, en un solo lugar.
- Valida la entrada en el servidor aunque el cliente ya la valide. El cliente no es una frontera fiable.
- Los codigos de error deben ser estables y documentados; el cliente depende de ellos.
- Cierra los endpoints con versioning (/v1) antes de que haya consumidores.
- Piensa en el caso de red lento y en el de error: la UI debe poder mostrar ambos estados.",
        tools: &["read_file", "write_file", "edit_file", "grep", "shell"],
        keywords: &["fullstack", "api", "cliente", "servidor", "endpoint", "contrato", "rest"],
        permission_hints: &[],
    },
    Skill {
        id: "sysadmin",
        name: "Sysadmin",
        description: "Servidores, procesos, logs y redes",
        category: Category::Role,
        instructions: "SYSADMIN:
- No reinicies ni mates procesos sin saber quien los inicio y que dependencias tienen.
- Cambios de sistema: copia de seguridad antes de tocar, y documenta como revertir.
- Mira los logs antes de reiniciar nada: reiniciar borra la evidencia.
- Los puertos y permisos se ajustan lo mas cerrado posible.
- Comprueba si un puerto ya esta ocupado antes de asignarlo.",
        tools: &["shell", "process", "read_file", "http_request"],
        keywords: &["servidor", "linux", "systemd", "nginx", "apache", "firewall", "red", "ssh", "dns", "proceso", "log"],
        permission_hints: &[("shell", PermissionLevel::Ask)],
    },
    Skill {
        id: "testing",
        name: "Testing",
        description: "Tests unitarios, fixtures y casos limite",
        category: Category::Workflow,
        instructions: "TESTING:
- Un test debe poder fallar por una sola razon. Si hace varias aserciones, probablemente separalo en dos.
- Nombra los tests por comportamiento: 'no acepta saldo negativo', no 'test_crear_cuenta'.
- Cubre los casos limite y los errores, no solo el camino feliz.
- No mockees lo que quieres probar; solo las fronteras (red, reloj, sistema de archivos).
- Si un test falla, la causa esta en el codigo salvo que demeure duda razonable.",
        tools: &["write_file", "shell", "edit_file"],
        keywords: &["test", "tests", "testing", "unit test", "spec", "jest", "pytest", "coverage", "prueba"],
        permission_hints: &[],
    },
    Skill {
        id: "git-workflow",
        name: "Flujo Git",
        description: "Commits atomicos, ramas y PRs",
        category: Category::Workflow,
        instructions: "GIT:
- Un commit hace una sola cosa. Si el mensaje necesita una coma, probablemente son dos commits.
- Revisa git diff antes de confirmar para no subir secretos ni archivos de depuracion.
- No uses git push --force sobre ramas compartidas.
- El mensaje describe el porque, no el que: 'evita doble cobro al reintentar', no 'fix en payment'.
- No reescribas historia ya publicada.",
        tools: &["shell"],
        keywords: &["git", "commit", "branch", "rama", "merge", "rebase", "pr", "pull request"],
        permission_hints: &[],
    },
    Skill {
        id: "code-review",
        name: "Code review",
        description: "Revisar cambios con ojo critico",
        category: Category::Workflow,
        instructions: "CODE REVIEW:
- Prioriza los fallos correctos sobre el estilo. Un comentario de estilo sin impacto se ignora.
- Seala problemas de seguridad y de concurrencia primero.
- Cada comentario debe proposer la correccion concreta, no solo el problema.
- Los errores tipograficos van aparte, al final, como nota menor.
- Si el codigo es correcto, dilo; no inventes problemas para parecer util.",
        tools: &["read_file", "grep", "git"],
        keywords: &["review", "revisar", "revisar codigo", "critica", "feedback", "diff"],
        permission_hints: &[],
    },
    Skill {
        id: "refactoring",
        name: "Refactoring",
        description: "Cambiar estructura sin cambiar comportamiento",
        category: Category::Workflow,
        instructions: "REFACTORING:
- Antes de refactorizar, asegura que hay tests que cubren el comportamiento actual.
- Haz un paso a la vez, verificando entre pasos. Un refactor grande no es revisable.
- Elimina codigo muerto en lugar de comentarlo.
- Renombra solo cuando el nombre nuevo aporta informacion.",
        tools: &["read_file", "edit_file", "grep", "shell"],
        keywords: &["refactor", "refactorizar", "limpieza", "simplificar", "deuda tecnica", "clean"],
        permission_hints: &[],
    },
    Skill {
        id: "docs",
        name: "Documentacion",
        description: "README, docstrings y comentarios utiles",
        category: Category::Workflow,
        instructions: "DOCUMENTACION:
- Comenta el porque, no el que. El que ya se lee en el codigo.
- La documentacion es para quien no conoce el contexto, no para ti dentro de seis meses.
- Mantenla junto al codigo que describe; la documentacion aparte se pudre.
- Todo ejemplo debe funcionar tal cual esta escrito.",
        tools: &["read_file", "write_file", "edit_file"],
        keywords: &["documentacion", "docs", "readme", "docstring", "comentario", "changelog"],
        permission_hints: &[],
    },
    Skill {
        id: "security",
        name: "Seguridad",
        description: "Validacion de entrada, secretos y dependencias",
        category: Category::Workflow,
        instructions: "SEGURIDAD:
- Toda entrada externa es no confiable, includo la que ya validaste en otro sitio.
- Usa consultas parametrizadas; nunca concatenes SQL ni comandos de shell.
- Compara cadenas de forma constante en las comprobaciones de credenciales.
- No registres contrasenas, tokens ni datos personales en los logs.
- Ante la duda, busca en las dependencias instaladas vulnerabilidades conocidas antes de asumir que estan bien.",
        tools: &["read_file", "edit_file", "grep"],
        keywords: &["seguridad", "security", "vulnerabilidad", "cve", "xss", "inyeccion", "autenticacion", "token"],
        permission_hints: &[("shell", PermissionLevel::Ask)],
    },
    Skill {
        id: "performance",
        name: "Rendimiento",
        description: "Medir antes de optimizar",
        category: Category::Workflow,
        instructions: "RENDIMIENTO:
- Mide antes de optimizar. Una optimizacion sin medir es una suposicion costosa.
- Identifica la fuente del cuello de botella real con un profiler.
- La optimizacion prematura hace el codigo mas dificil de leer a cambio de nada medible.
- Mide de nuevo tras cada cambio para confirmar que ayudaba.",
        tools: &["read_file", "grep", "shell"],
        keywords: &["performance", "rendimiento", "va lento", "va muy lento", "optimizar", "optimiza", "memoria", "cpu", "cache", "cuello de botella", "profiling", "latencia"],
        permission_hints: &[],
    },
    Skill {
        id: "debugging",
        name: "Debugging",
        description: "Encontrar la causa raiz antes de parchear",
        category: Category::Workflow,
        instructions: "DEBUGGING:
- Reproduce el fallo primero. Un fallo que no reproduces no lo puedes verificar.
- Aislalo minimizando: quita codigo hasta que desaparezca, ese es el culpable.
- Lee el error completo, incluido el stack trace y los warnings previos al fallo.
- No cambies varias cosas a la vez. Si funciono al cambiar una sola, sabes cual era.
- Cuando encuentres la causa, arregla la causa. Un parche que oculta el sintoma vuelve como error distinto.
- Si tras varios intentos no das con ello, di que no lo sabes en vez de adivinar una causa mas.",
        tools: &["read_file", "grep", "shell", "edit_file"],
        keywords: &["bug", "error", "falla", "no funciona", "rompe", "exception", "panic", "stacktrace", "crash", "debug", "falla"],
        permission_hints: &[],
    },
    Skill {
        id: "error-handling",
        name: "Manejo de errores",
        description: "Errores utiles y nada de silenciar fallos",
        category: Category::Workflow,
        instructions: "MANEJO DE ERRORES:
- No te tragues errores con un catch vacio. Un fallo silencioso se descubre tarde y mal.
- Si no puedes hacer nada util con el error, propagalo con contexto de donde fallo.
- El mensaje debe responder: que fallo, con que valor, y como reintentar.
- No expongas trazas de pila ni rutas internas al cliente final.
- Distingue el error esperado (validacion, no encontrado) del inesperado (bug). Solo el segundo necesita log de error.
- No trates de capturar Exception como un todo para que pase algo: oculta los bugs reales.",
        tools: &["read_file", "edit_file", "grep"],
        keywords: &["error", "exception", "catch", "try", "panic", "fallo", "resiliencia", "retry"],
        permission_hints: &[],
    },
    Skill {
        id: "api-design",
        name: "Diseno de API",
        description: "Contratos estables, versionado y compatibilidad",
        category: Category::Workflow,
        instructions: "DISEÑO DE API:
- Anade campos, nunca los quites ni cambies su tipo: romper a los clientes es el costo mas caro.
- Versiona desde el principio (/v1), aunque te parezca que no lo necesitas. Retirarlo despues es la migracion dolorosa.
- Los codigos de estado deben ser precisos: 404 es ausencia, 400 es peticion invalida, 422 es error de validacion.
- Los campos opcionales se documentan con su valor por defecto.
- Define limites y paginacion desde el inicio; anadirlos despues rompe a los clientes que ya dependen del todo.
- Sin endpoints que muten estado a traves de GET.",
        tools: &["read_file", "write_file", "edit_file"],
        keywords: &["api", "endpoint", "rest", "openapi", "swagger", "contrato", "versionado", "schema"],
        permission_hints: &[],
    },
    Skill {
        id: "migrations",
        name: "Migraciones",
        description: "Cambios de esquema sin perder datos",
        category: Category::Workflow,
        instructions: "MIGRACIONES:
- Toda migracion debe ser reversible: escribe el down, aunque luego no lo uses.
- No renombres ni elimines columnas en el mismo despliegue que deja de usarlas. Primero se deja de leer, luego se borra.
- Las claves foraneas y los indices bloquean tablas grandes: comprueba el impacto antes de ejecutarlo en produccion.
- Anadir columna con default NOT NULL reescribe la tabla en motores antiguos. Anade nullable, migra, y despues aplica el default.
- Prueba el down antes de dar por buena la migracion.",
        tools: &["sql_query", "read_file", "edit_file", "shell"],
        keywords: &["migracion", "migration", "schema", "alter table", "db", "base de datos", "indice"],
        permission_hints: &[("sql_query", PermissionLevel::Ask)],
    },
    Skill {
        id: "git-workflow-advanced",
        name: "Git avanzado",
        description: "Deshacer, rebases y recuperación",
        category: Category::Workflow,
        instructions: "GIT AVANZADO:
- Antes de una operacion dudosa, git stash o crea una rama. Deshacer en local siempre; reescribir lo publicado casi nunca.
- git reflog encuentra commits perdidos aunque los hayas borrado.
- Un rebase en una rama publicada molesta a todos los que la tienen.
- Resuelve conflictos entendiendo las dos versiones. No cojas una sin leer la otra.
- No uses git add . sin revisar: mete archivos que no querias versionar.",
        tools: &["shell", "read_file", "grep"],
        keywords: &["git", "rebase", "merge", "stash", "cherry-pick", "reflog", "conflict", "deshacer", "undo"],
        permission_hints: &[],
    },
    Skill {
        id: "accessibility",
        name: "Accesibilidad",
        description: "Web usable con teclado y lector de pantalla",
        category: Category::Workflow,
        instructions: "ACCESIBILIDAD:
- Todo elemento interactivo es alcanzable con Tab y activable con Enter o Espacio.
- Los labels de los campos son explicitos; el placeholder no es un label accesible.
- El orden del DOM debe seguir el orden visual. No reordenes con CSS para maquetar.
- El foco tiene que verse: no elimines el outline sin poner algo en su lugar.
- Los dialogs atrapan el foco y lo devuelven al cerrarse.
- El contraste de texto debe cumplir al menos 4.5:1.",
        tools: &["read_file", "edit_file", "grep"],
        keywords: &["accesibilidad", "a11y", "aria", "wcag", "teclado", "lector de pantalla", "foco", "tab"],
        permission_hints: &[],
    },
    Skill {
        id: "concurrency",
        name: "Concurrencia",
        description: "Hilos, deadlocks y condiciones de carrera",
        category: Category::Workflow,
        instructions: "CONCURRENCIA:
- Los datos compartidos entre hilos necesitan sincronizacion explicita. Sin ella, el comportamiento es indefinido.
- Mantiene los locks en orden consistente para evitar deadlocks.
- No bloquees un lock mientras esperas otra operacion lenta de I/O o de red.
- El estado compartido necesita ser visible entre hilos; si no, el optimizador lo reordena y aparece un bug fantasma.
- Los datos inmutables eliminan la categoria entera de bugs de concurrencia: prefierelos.
- Reproduce con -race o el detector equivalente antes de dar por buena una correccion.",
        tools: &["read_file", "edit_file", "grep", "shell"],
        keywords: &["concurrencia", "concurrencia", "hilo", "thread", "mutex", "lock", "race", "deadlock", "async", "paralelo", "hilos"],
        permission_hints: &[],
    },
    Skill {
        id: "observability",
        name: "Observabilidad",
        description: "Logs estructurados, metricas y trazas",
        category: Category::Workflow,
        instructions: "OBSERVABILIDAD:
- Los logs son datos estructurados, no texto libre: un log que no se puede consultar no sirve.
- Cada log lleva un identificador de correlacion para unirlo con la peticion o el trace.
- Registra los eventos y sus resultados, no cada iteracion interna.
- Los warnings y errors deben verse en produccion; un log en debug no existe para el usuario.
- Nunca registres contrasenas, tokens ni datos personales.
- Las metricas tienen nombre, unidad y etiquetas de baja cardinalidad.",
        tools: &["read_file", "write_file", "edit_file", "shell"],
        keywords: &["log", "logging", "metrica", "observabilidad", "monitoring", "traza", "trace", "alerta", "dashboard"],
        permission_hints: &[],
    },
    Skill {
        id: "api-consumer",
        name: "Consumo de APIs",
        description: "Llamadas HTTP, reintentos y limites de tasa",
        category: Category::Workflow,
        instructions: "CONSUMO DE APIs:
- Aplica timeout a toda llamada externa. Una llamada sin timeout bloquea el recurso para siempre.
- Reintenta solo errores transitorios (5xx, timeout) con backoff exponencial y jitter. Un 4xx no se arregla reintentando.
- Respeta el Retry-After y los limites de tasa del proveedor.
- Cachea cuando el dato no cambia entre peticiones y hay una ventana de tolerancia.
- Normaliza los errores del proveedor: la respuesta cruda no debe filtrarse a la logica de negocio.",
        tools: &["http_request", "read_file", "edit_file", "shell"],
        keywords: &["api", "http", "rest", "request", "fetch", "retry", "timeout", "webhook", "cliente http"],
        permission_hints: &[],
    },
    Skill {
        id: "conservador",
        name: "Modo conservador",
        description: "Pide confirmacion antes de actuar",
        category: Category::Autonomy,
        instructions: "MODO CONSERVADOR:
- Antes de modificar archivos o ejecutar comandos, explica que vas a hacer y por que, y espera confirmacion.
- Ante la duda, pregunta en lugar de asumir.",
        tools: &[],
        keywords: &[],
        permission_hints: &[
            ("shell", PermissionLevel::Ask),
            ("write_file", PermissionLevel::Ask),
            ("edit_file", PermissionLevel::Ask),
            ("patch_file", PermissionLevel::Ask),
            ("http_request", PermissionLevel::Ask),
            ("sql_query", PermissionLevel::Ask),
            ("schedule", PermissionLevel::Ask),
        ],
    },
    Skill {
        id: "balanceado",
        name: "Modo balanceado",
        description: "Escribe sin preguntar, confirma el shell",
        category: Category::Autonomy,
        instructions: "MODO BALANCEADO:
- Edita archivos y escribe codigo sin pedir confirmacion: eso es lo que te encargaron.
- Pide confirmacion antes de ejecutar comandos de shell, sobre todo si pueden modificar el sistema o la red.
- Si una accion es destructiva o irreversible (borrado, despliegue, migracion), confirma primero.",
        tools: &[],
        keywords: &[],
        permission_hints: &[
            ("write_file", PermissionLevel::Auto),
            ("edit_file", PermissionLevel::Auto),
            ("patch_file", PermissionLevel::Auto),
            ("shell", PermissionLevel::Ask),
            ("http_request", PermissionLevel::Ask),
            ("sql_query", PermissionLevel::Ask),
            ("schedule", PermissionLevel::Ask),
        ],
    },
    Skill {
        id: "autonomo",
        name: "Modo autonomo",
        description: "Actua sin preguntar y verifica el resultado",
        category: Category::Autonomy,
        instructions: "MODO AUTONOMO:
- No pidas confirmacion para tareas que el usuario ya te pidio hacer.
- Decide tu y sigue adelante, pero reporta que cambiaste y que verificaste.
- Si una accion es destructiva o irreversible (borrado, despliegue, migracion), confirma primero.",
        tools: &[],
        keywords: &[],
        permission_hints: &[
            ("shell", PermissionLevel::Auto),
            ("write_file", PermissionLevel::Auto),
            ("edit_file", PermissionLevel::Auto),
            ("patch_file", PermissionLevel::Auto),
            ("http_request", PermissionLevel::Auto),
        ],
    },
];

pub fn find(id: &str) -> Option<&'static Skill> {
    SKILLS.iter().find(|s| s.id == id)
}

pub fn by_category(category: Category) -> Vec<&'static Skill> {
    SKILLS.iter().filter(|s| s.category == category).collect()
}

pub fn suggest_for(input: &str) -> Option<&'static Skill> {
    let lower = input.to_lowercase();

    let mut best: Option<(&'static Skill, f64)> = None;

    for skill in SKILLS {
        if skill.keywords.is_empty() {
            continue;
        }

        // Los keywords cortos ("bug", "go") hacen match con facilidad de sobra y
        // Falsean la puntuacion, asi que se pesan segun su longitud.
        let score: f64 = skill
            .keywords
            .iter()
            .map(|k: &&'static str| {
                if lower.contains(*k) {
                    1.0 + k.len() as f64 / 10.0
                } else {
                    0.0
                }
            })
            .sum();

        if score <= 0.0 {
            continue;
        }

        let better = match best {
            None => true,
            Some((_, best_score)) => score > best_score,
        };

        if better {
            best = Some((skill, score));
        }
    }

    best.map(|(skill, _)| skill)
}
