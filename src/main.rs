mod agent;
mod config;
mod db;
mod onboarding;
mod permissions;
mod providers;
mod skills;
mod tools;

use anyhow::Result;
use colored::*;
use rustyline::config::Builder as RlConfig;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::agent::Agent;
use crate::config::Config;
use crate::permissions::Permissions;
use crate::skills::Profile;

const VERSION: &str = "0.3.0";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    print_banner();

    let config = Config::load()?;
    if !config.is_configured() {
        print_setup_instructions();
        std::process::exit(1);
    }

    println!(
        "{} Provider: {} | Modelo: {} | v{}",
        "✓".green(),
        config.provider.to_string().yellow(),
        config.model.dimmed(),
        VERSION
    );
    println!("{} Base de datos: {:?}\n", "✓".green(), config.db_path());

    let mut permissions = Permissions::load(&config.data_dir);

    let mut profile = match Profile::load(&config.data_dir) {
        Some(p) if p.is_configured() => p,
        _ => {
            println!();
            println!(
                "{}",
                "  Primera ejecucion: configuramos tus skills."
                    .cyan()
                    .bold()
            );

            match onboarding::run_wizard(&config.data_dir, None) {
                Ok(Some(p)) => p,
                Ok(None) => {
                    println!();
                    println!(
                        "{}",
                        "  Configuracion cancelada. Puedes ejecutarla luego con /setup.".yellow()
                    );
                    Profile::default()
                }
                Err(e) => {
                    eprintln!("Error en la configuracion: {}", e);
                    Profile::default()
                }
            }
        }
    };

    apply_skill_permissions(&mut permissions, &profile, &config.data_dir);

    let data_dir = config.data_dir.clone();
    let mut agent = Agent::new(config, permissions, profile.clone())?;
    let mut suggested_in_session = false;

    let rl_config = RlConfig::new()
        .auto_add_history(true)
        .max_history_size(1000)?
        .build();
    let mut rl = DefaultEditor::with_config(rl_config)?;

    let history_path = dirs::config_dir()
        .unwrap_or_default()
        .join("nano-agent")
        .join("history.txt");
    let _ = rl.load_history(&history_path);

    print_help();
    let mut running = true;

    while running {
        let prompt = format!("{} ", "> ".green().bold());
        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim().to_string();
                if input.is_empty() {
                    continue;
                }

                // Comandos con argumento: /skills +rust, -testing, ?git-workflow
                let lower = input.to_lowercase();
                if let Some(arg) = lower.strip_prefix("/skills") {
                    let arg = arg.trim();
                    if !arg.is_empty() {
                        handle_skills_command(&mut profile, arg, agent.permissions_mut());
                        let _ = profile.save(&data_dir);
                        continue;
                    }
                }

                // Comandos especiales
                match input.to_lowercase().as_str() {
                    "/quit" | "/exit" | "/q" | "/bye" => {
                        running = false;
                        continue;
                    }
                    "/new" | "/reset" => {
                        agent.new_conversation()?;
                        println!("{}\n", "Nueva conversación iniciada".green());
                        continue;
                    }
                    "/help" | "/h" => {
                        print_help();
                        continue;
                    }
                    "/tools" => {
                        print_tools(&agent);
                        continue;
                    }
                    "/permissions" | "/perms" => {
                        print_permissions(&agent);
                        continue;
                    }
                    "/compact" => {
                        agent.compact_conversation()?;
                        println!("{}\n", "Conversación compactada".green());
                        continue;
                    }
                    "/skills" => {
                        onboarding::print_skills(&profile);
                        continue;
                    }
                    "/setup" => {
                        println!();
                        match onboarding::run_wizard(&data_dir, Some(profile.clone())) {
                            Ok(Some(updated)) => {
                                apply_skill_permissions(
                                    agent.permissions_mut(),
                                    &updated,
                                    &data_dir,
                                );
                                *agent.profile_mut() = updated.clone();
                                agent.refresh_system_prompt();
                                profile = updated;
                                println!(
                                    "  {} {}\n",
                                    "✓".green().bold(),
                                    "Perfil actualizado".green()
                                );
                            }
                            Ok(None) => {
                                println!("  {} {}\n", "○".yellow(), "Sin cambios".yellow());
                            }
                            Err(e) => {
                                eprintln!("Error: {}", e);
                            }
                        }
                        continue;
                    }
                    _ => {}
                }

                // Sugerir una skill relevante en el primer mensaje de la sesion
                if !suggested_in_session && profile.is_configured() {
                    suggested_in_session = true;
                    if let Some(skill) = profile.suggest(&input) {
                        println!();
                        println!(
                            "  {} {}",
                            "💡".yellow(),
                            format!(
                                "La skill \"{}\" podria servir aqui. {}",
                                skill.name.cyan(),
                                format!("/skills +{} para activarla", skill.id).dimmed()
                            )
                        );
                        println!();
                    }
                }

                // Procesar mensaje
                match agent.process_message(&input).await {
                    Ok(response) => {
                        println!("\n{}\n", response);
                    }
                    Err(e) => {
                        println!(" {} {}\n", "Error:".red(), e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("\n{}", "Ctrl+C. Usa /quit para salir.".yellow());
            }
            Err(ReadlineError::Eof) => {
                running = false;
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                running = false;
            }
        }
    }

    let _ = rl.save_history(&history_path);
    println!("\n{}", "¡Hasta luego!".green().bold());
    Ok(())
}

fn print_banner() {
    let mascot = r#"  ███████████  
  ██  ███  ██  
████  ███  ████
  ███████████  
   ▀▀     ▀▀   "#;
    println!("{}", mascot.bright_blue());
    println!();
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║                                                   ║".cyan()
    );
    println!(
        "{}",
        "║     NanoAgent - AI Coding Assistant               ║".cyan()
    );
    println!(
        "{}",
        "║     by IMANDRO - github.com/IMANDRO               ║".cyan()
    );
    println!(
        "{}",
        "║                                                   ║".cyan()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════╝".cyan()
    );
    println!();
}

fn apply_skill_permissions(
    permissions: &mut Permissions,
    profile: &Profile,
    data_dir: &std::path::Path,
) {
    let changed = permissions.apply_overrides(&profile.permission_overrides());
    let _ = permissions.save(&data_dir.to_path_buf());

    if !changed.is_empty() {
        println!(
            "  {} {} {}",
            "🔐".dimmed(),
            "Permisos ajustados por skills:".dimmed(),
            changed.join(", ").yellow()
        );
    }
}

fn handle_skills_command(profile: &mut Profile, arg: &str, permissions: &mut Permissions) {
    let (op, id) = arg.split_at(1);
    let id = id.trim();

    match op {
        "+" => match profile.activate(id) {
            Ok(()) => {
                println!();
                println!(
                    "  {} {} {}",
                    "✓".green(),
                    "Skill activada:".green(),
                    id.cyan()
                );
                let changed = permissions.apply_overrides(&profile.permission_overrides());
                if !changed.is_empty() {
                    println!(
                        "    {} {}",
                        "Permisos:".dimmed(),
                        changed.join(", ").yellow()
                    );
                }
            }
            Err(e) => println!("  {} {}", "✗".red(), e),
        },
        "-" => {
            if profile.deactivate(id) {
                println!();
                println!(
                    "  {} {} {}",
                    "○".yellow(),
                    "Skill desactivada:".yellow(),
                    id.cyan()
                );
            } else {
                println!("  {} {}", "○".yellow(), "No estaba activa".yellow());
            }
        }
        "?" => {
            onboarding::print_skill_detail(id);
        }
        _ => {
            println!("  {} {}", "✗".red(), "Usa /skills +id, -id o ?id".red());
        }
    }
}

fn print_help() {
    println!(
        "{}",
        "╔══════════════════════════════════════════╗".dimmed()
    );
    println!(
        "{}",
        "║           COMANDOS DISPONIBLES           ║".dimmed()
    );
    println!(
        "{}",
        "╠══════════════════════════════════════════╣".dimmed()
    );
    println!(
        "{}",
        "║ /help    - Mostrar esta ayuda            ║".dimmed()
    );
    println!(
        "{}",
        "║ /new     - Nueva conversación            ║".dimmed()
    );
    println!(
        "{}",
        "║ /compact - Compactar contexto            ║".dimmed()
    );
    println!(
        "{}",
        "║ /tools   - Listar herramientas           ║".dimmed()
    );
    println!(
        "{}",
        "║ /perms   - Ver permisos                  ║".dimmed()
    );
    println!(
        "{}",
        "║ /skills  - Ver y gestionar skills        ║".dimmed()
    );
    println!(
        "{}",
        "║ /setup   - Reconfigurar tu perfil        ║".dimmed()
    );
    println!(
        "{}",
        "║ /quit    - Salir                         ║".dimmed()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════╝".dimmed()
    );
    println!();
    println!("{}", "  Ejemplos:".dimmed());
    println!(
        "{}",
        "    /skills +testing      activa la skill testing".dimmed()
    );
    println!(
        "{}",
        "    /skills -rust         desactiva la skill rust".dimmed()
    );
    println!(
        "{}",
        "    /skills ?git-workflow muestra el detalle".dimmed()
    );
    println!();
}

fn print_tools(agent: &Agent) {
    println!("{}", "╔══════════════════════════════════════════╗".cyan());
    println!("{}", "║           HERRAMIENTAS                   ║".cyan());
    println!("{}", "╠══════════════════════════════════════════╣".cyan());

    let tools = agent.list_tools();
    for (name, desc, safe) in &tools {
        let icon = if *safe { "🟢" } else { "🟡" };
        let safe_text = if *safe { "auto" } else { "ask " };
        println!(
            "║ {} {} - {} [{}]     ║",
            icon,
            name.yellow(),
            desc.dimmed(),
            safe_text.dimmed()
        );
    }

    println!("{}", "╚══════════════════════════════════════════╝".cyan());
    println!();
}

fn print_permissions(agent: &Agent) {
    println!("{}", "╔══════════════════════════════════════════╗".cyan());
    println!("{}", "║           PERMISOS                       ║".cyan());
    println!("{}", "╠══════════════════════════════════════════╣".cyan());
    println!(
        "{}",
        "║ 🟢 auto  - Se ejecuta sin preguntar      ║".dimmed()
    );
    println!(
        "{}",
        "║ 🟡 ask   - Pide confirmación antes       ║".dimmed()
    );
    println!(
        "{}",
        "║ 🔴 deny  - Bloqueado                     ║".dimmed()
    );
    println!("{}", "╚══════════════════════════════════════════╝".cyan());
    println!();
}

fn print_setup_instructions() {
    println!("\n{}", "⚠ No hay API key configurada.".yellow().bold());
    println!("\nCrea un archivo .env con:\n");
    println!("  AI_PROVIDER=groq");
    println!("  GROQ_API_KEY=tu_api_key");
    println!("\nObtén una API key gratis en: https://console.groq.com");
}
