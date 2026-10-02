mod agent;
mod config;
mod db;
mod permissions;
mod providers;
mod tools;

use anyhow::Result;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::config::Builder as RlConfig;
use rustyline::DefaultEditor;

use crate::agent::Agent;
use crate::config::Config;
use crate::permissions::{PermissionLevel, Permissions};

const VERSION: &str = "0.2.0";

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
    println!(
        "{} Base de datos: {:?}\n",
        "✓".green(),
        config.db_path()
    );

    let permissions = Permissions::load(&config.data_dir);
    let mut agent = Agent::new(config, permissions)?;

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
                    _ => {}
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
    println!("{}", "╔═══════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                                                   ║".cyan());
    println!("{}", "║     NanoAgent - AI Coding Assistant               ║".cyan());
    println!("{}", "║     by IMANDRO - github.com/IMANDRO               ║".cyan());
    println!("{}", "║                                                   ║".cyan());
    println!("{}", "╚═══════════════════════════════════════════════════╝".cyan());
    println!();
}

fn print_help() {
    println!("{}", "╔══════════════════════════════════════════╗".dimmed());
    println!("{}", "║           COMANDOS DISPONIBLES           ║".dimmed());
    println!("{}", "╠══════════════════════════════════════════╣".dimmed());
    println!("{}", "║ /help    - Mostrar esta ayuda            ║".dimmed());
    println!("{}", "║ /new     - Nueva conversación            ║".dimmed());
    println!("{}", "║ /compact - Compactar contexto            ║".dimmed());
    println!("{}", "║ /tools   - Listar herramientas           ║".dimmed());
    println!("{}", "║ /perms   - Ver permisos                  ║".dimmed());
    println!("{}", "║ /quit    - Salir                         ║".dimmed());
    println!("{}", "╚══════════════════════════════════════════╝".dimmed());
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
        println!("║ {} {} - {} [{}]     ║", icon, name.yellow(), desc.dimmed(), safe_text.dimmed());
    }

    println!("{}", "╚══════════════════════════════════════════╝".cyan());
    println!();
}

fn print_permissions(agent: &Agent) {
    println!("{}", "╔══════════════════════════════════════════╗".cyan());
    println!("{}", "║           PERMISOS                       ║".cyan());
    println!("{}", "╠══════════════════════════════════════════╣".cyan());
    println!("{}", "║ 🟢 auto  - Se ejecuta sin preguntar      ║".dimmed());
    println!("{}", "║ 🟡 ask   - Pide confirmación antes       ║".dimmed());
    println!("{}", "║ 🔴 deny  - Bloqueado                     ║".dimmed());
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
