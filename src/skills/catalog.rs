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
        keywords: &["api", "endpoint", "backend", "rest", "graphql", "servidor", "microservicio"],
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
        keywords: &["frontend", "ui", "componente", "css", "react", "vue", "html", "accesibilidad"],
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
        keywords: &["performance", "rendimiento", "lento", "optimizar", "memoria", "cpu", "cache", "cuello de botella"],
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

    let mut best: Option<(&'static Skill, usize)> = None;

    for skill in SKILLS {
        let score = skill
            .keywords
            .iter()
            .filter(|k| lower.contains(**k))
            .count();

        if score == 0 {
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
