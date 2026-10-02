use anyhow::Result;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::path::Path;

use crate::skills::catalog::{by_category, Category};

use crate::skills::{Autonomy, Language, Profile, Verbosity};

const W: usize = 56;

fn line(ch: char) -> String {
    std::iter::repeat(ch).take(W).collect()
}

fn box_top(left: &str, right: &str) -> String {
    format!("{}{}{}", left, line('─'), right)
}

fn box_sep() -> String {
    format!("├{}┤", line('─'))
}

fn row(content: &str) -> String {
    format!("│ {:<w$} │", content, w = W - 2)
}

fn row_colored(content: String) -> String {
    format!("│ {:<w$} │", content, w = W - 2)
}

fn print_step(n: usize, total: usize, title: &str) {
    println!();
    println!("{}", box_top("╔", "╗").cyan());
    let header = format!(
        "  {}  {}  [{} de {}]",
        "◆".bright_cyan(),
        title.bold(),
        n,
        total
    );
    println!("{}", row_colored(header).cyan());
    println!("{}", box_sep().cyan());
}

fn print_wizard_exit() {
    println!();
    println!(
        "{}",
        row("  ¡Hasta luego! Configuracion cancelada.").dimmed()
    );
    println!("{}", format!("╚{}╝", line('─')).dimmed());
    println!();
}

fn read_line(rl: &mut DefaultEditor, prompt: &str) -> Result<Option<String>> {
    loop {
        match rl.readline(prompt) {
            Ok(line) => return Ok(Some(line)),
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => return Ok(None),
            Err(e) => {
                println!("  {} {}", "Error de lectura:".red(), e);
                return Ok(None);
            }
        }
    }
}

fn ask_text(
    rl: &mut DefaultEditor,
    data_dir: &Path,
    title: &str,
    n: usize,
    total: usize,
    question: &str,
    hint: &str,
) -> Option<Option<String>> {
    print_step(n, total, title);
    println!("{}", row_colored(format!("  {}", question)));
    println!("{}", row_colored(format!("  {}", hint.dimmed())));
    println!("{}", format!("╚{}╝", line('─')).cyan());

    let raw = read_line(rl, "  > ").ok().flatten()?;
    let value = raw.trim().to_string();

    if value.eq_ignore_ascii_case("q") || value.eq_ignore_ascii_case("quit") {
        print_wizard_exit();
        return None;
    }

    if value.is_empty() {
        return Some(None);
    }

    let _ = rl.add_history_entry(value.as_str());
    let _ = std::fs::create_dir_all(data_dir);
    Some(Some(value))
}

fn ask_choice(
    rl: &mut DefaultEditor,
    title: &str,
    n: usize,
    total: usize,
    question: &str,
    options: &[(&str, String)],
    default: usize,
) -> Option<usize> {
    print_step(n, total, title);
    println!("{}", row_colored(format!("  {}", question)));
    println!("{}", format!("╟{}╢", line('─')).cyan());
    for (i, (_, desc)) in options.iter().enumerate() {
        let num = format!("{}", i + 1);
        println!(
            "{}",
            row_colored(format!("  {}  {}", num.bold().cyan(), desc))
        );
    }
    println!("{}", format!("╚{}╝", line('─')).cyan());
    println!(
        "  {} {}",
        "Elige 1-".dimmed(),
        options.len().to_string().dimmed()
    );
    println!();

    loop {
        let raw = read_line(rl, "  > ").ok().flatten()?;
        let value = raw.trim();

        if value.eq_ignore_ascii_case("q") {
            print_wizard_exit();
            return None;
        }

        if value.is_empty() {
            return Some(default);
        }

        if let Ok(idx) = value.parse::<usize>() {
            if idx >= 1 && idx <= options.len() {
                return Some(idx - 1);
            }
        }

        println!("  {} {}", "Opcion no valida.".yellow(), "Intenta de nuevo.");
    }
}

fn ask_multiselect(
    rl: &mut DefaultEditor,
    title: &str,
    n: usize,
    total: usize,
    question: &str,
    hint: &str,
    skills: &[&'static crate::skills::Skill],
) -> Option<Vec<String>> {
    print_step(n, total, title);
    println!("{}", row_colored(format!("  {}", question)));
    println!("{}", row_colored(format!("  {}", hint.dimmed())));
    println!("{}", format!("╟{}╢", line('─')).cyan());
    for (i, s) in skills.iter().enumerate() {
        let num = format!("{}", i + 1);
        println!(
            "{}",
            row_colored(format!(
                "  {}  {:<14} {}",
                num.bold().cyan(),
                s.name.cyan(),
                s.description.dimmed()
            ))
        );
    }
    println!("{}", format!("╚{}╝", line('─')).cyan());
    println!(
        "  {} {}",
        "Separa con comas".dimmed(),
        "(1,3) · Enter = ninguna · all = todas".dimmed()
    );
    println!();

    loop {
        let raw = read_line(rl, "  > ").ok().flatten()?;
        let value = raw.trim().to_lowercase();

        if value == "q" || value == "quit" {
            print_wizard_exit();
            return None;
        }

        if value.is_empty() {
            return Some(Vec::new());
        }

        if value == "all" || value == "todas" {
            return Some(skills.iter().map(|s| s.id.to_string()).collect());
        }

        let mut picked = Vec::new();
        let mut valid = true;

        for part in value.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            match part.parse::<usize>() {
                Ok(idx) if idx >= 1 && idx <= skills.len() => {
                    picked.push(skills[idx - 1].id.to_string());
                }
                _ => {
                    println!("  {} {}", "Numero no valido:".yellow(), part);
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            picked.sort();
            picked.dedup();
            return Some(picked);
        }

        println!(
            "  {} {}",
            "Intenta de nuevo.".yellow(),
            "Ejemplo: 1,3,5".dimmed()
        );
    }
}

fn print_summary(profile: &Profile) {
    println!();
    println!("{}", box_top("╔", "╗").cyan());
    println!(
        "{}",
        row_colored(format!("  {}", "  RESUMEN".bold())).cyan()
    );
    println!("{}", box_sep().cyan());

    if let Some(name) = &profile.name {
        println!(
            "{}",
            row_colored(format!("  {}  {}", "Usuario:".dimmed(), name.cyan()))
        );
    }
    println!(
        "{}",
        row_colored(format!(
            "  {}  {} · {} · {}",
            "Preferencias:".dimmed(),
            profile.autonomy.label().yellow(),
            profile.verbosity.label().yellow(),
            profile.language.label().yellow()
        ))
    );
    println!("{}", box_sep().cyan());

    if profile.skills.is_empty() {
        println!(
            "{}",
            row_colored(format!("  {}", "Sin skills activas.".yellow()))
        );
        println!(
            "{}",
            row_colored(format!(
                "  {}",
                "Puedes activarlas luego con /skills".dimmed()
            ))
        );
    } else {
        for skill in profile.active() {
            println!(
                "{}",
                row_colored(format!(
                    "  {}  {:<14} {}",
                    "✓".green(),
                    skill.name.cyan(),
                    skill.category.label().dimmed()
                ))
            );
        }
    }

    println!("{}", format!("╚{}╝", line('─')).cyan());
}

pub fn run_wizard(data_dir: &Path, existing: Option<Profile>) -> Result<Option<Profile>> {
    const TOTAL: usize = 8;

    let mut rl = DefaultEditor::new()?;

    let previous = existing.unwrap_or_default();
    let mut profile = Profile {
        name: previous.name.clone(),
        ..Profile::default()
    };

    println!();
    println!("{}", "  Bienvenido a NanoAgent".bright_cyan().bold());
    println!(
        "{}",
        "  Vamos a configurar tus skills. 8 preguntas, puedes cancelar con q.".dimmed()
    );

    match ask_text(
        &mut rl,
        data_dir,
        "TU PERFIL",
        1,
        TOTAL,
        "¿Como te llamas?",
        "Opcional. Enter para saltar.",
    ) {
        Some(value) => profile.name = value,
        None => return Ok(None),
    }

    let roles = by_category(Category::Role);
    let role_ids = match ask_multiselect(
        &mut rl,
        "TU ROL",
        2,
        TOTAL,
        "¿Cual es tu rol principal?",
        "Define el tipo de software que construyes.",
        &roles,
    ) {
        Some(v) => v,
        None => return Ok(None),
    };
    for id in &role_ids {
        let _ = profile.activate(id);
    }

    let languages = by_category(Category::Language);
    let lang_ids = match ask_multiselect(
        &mut rl,
        "LENGUAJES",
        3,
        TOTAL,
        "¿Con que lenguajes trabajas?",
        "Adaptan las convenciones y los comandos de build.",
        &languages,
    ) {
        Some(v) => v,
        None => return Ok(None),
    };
    for id in &lang_ids {
        let _ = profile.activate(id);
    }

    let workflows = by_category(Category::Workflow);
    let wf_ids = match ask_multiselect(
        &mut rl,
        "FLUJOS DE TRABAJO",
        4,
        TOTAL,
        "¿Que flujos de trabajo quieres?",
        "Practicas que el agente debe seguir siempre.",
        &workflows,
    ) {
        Some(v) => v,
        None => return Ok(None),
    };
    for id in &wf_ids {
        let _ = profile.activate(id);
    }

    let autonomy_opts: Vec<(&str, String)> = [
        Autonomy::Conservador,
        Autonomy::Balanceado,
        Autonomy::Autonomo,
    ]
    .iter()
    .map(|a| (a.label(), a.description().to_string()))
    .collect();

    let autonomy_idx = match ask_choice(
        &mut rl,
        "AUTONOMIA",
        5,
        TOTAL,
        "¿Cuanto debe actuar por su cuenta?",
        &autonomy_opts,
        1,
    ) {
        Some(v) => v,
        None => return Ok(None),
    };
    profile.autonomy = Autonomy::from_index(autonomy_idx);
    let _ = profile.activate(profile.autonomy.skill_id());

    let verbosity_opts: Vec<(&str, String)> =
        [Verbosity::Conciso, Verbosity::Normal, Verbosity::Detallado]
            .iter()
            .map(|v| (v.label(), v.description().to_string()))
            .collect();

    let verbosity_idx = match ask_choice(
        &mut rl,
        "VERBOSIDAD",
        6,
        TOTAL,
        "¿Cuanto detalle quieres en las respuestas?",
        &verbosity_opts,
        1,
    ) {
        Some(v) => v,
        None => return Ok(None),
    };
    profile.verbosity = Verbosity::from_index(verbosity_idx);

    let language_opts: Vec<(&str, String)> =
        [Language::Espanol, Language::Ingles, Language::SegunMensaje]
            .iter()
            .map(|l| (l.label(), l.description().to_string()))
            .collect();

    let language_idx = match ask_choice(
        &mut rl,
        "IDIOMA",
        7,
        TOTAL,
        "¿En que idioma respondes?",
        &language_opts,
        2,
    ) {
        Some(v) => v,
        None => return Ok(None),
    };
    profile.language = Language::from_index(language_idx);

    print_step(8, TOTAL, "LISTO");
    println!(
        "{}",
        row_colored(format!("  {}", "Tus skills estan listas.".green()))
    );
    println!();

    loop {
        print_summary(&profile);
        println!(
            "  {} {}  {} {}  {} {}",
            "s".bold().green(),
            "guardar".dimmed(),
            "r".bold().cyan(),
            "repetir".dimmed(),
            "q".bold().yellow(),
            "cancelar".dimmed()
        );
        println!();

        let raw = match read_line(&mut rl, "  > ").ok().flatten() {
            Some(v) => v,
            None => {
                print_wizard_exit();
                return Ok(None);
            }
        };

        match raw.trim().to_lowercase().as_str() {
            "s" | "" => {
                profile.stamp();
                profile.save(data_dir)?;
                println!();
                println!(
                    "{}",
                    format!(
                        "  {} {} skills activas.",
                        "✓".green().bold(),
                        profile.skills.len()
                    )
                    .green()
                );
                println!(
                    "  {} {}",
                    "Guardado en".dimmed(),
                    Profile::path(data_dir).display().to_string().cyan()
                );
                println!();
                return Ok(Some(profile));
            }
            "r" => {
                let mut rerun = profile.clone();
                rerun.skills.clear();
                return run_wizard(data_dir, Some(rerun)).or_else(|_| Ok(None));
            }
            "q" | "quit" => {
                print_wizard_exit();
                return Ok(None);
            }
            _ => println!("  {}", "Elige s, r o q.".yellow()),
        }
    }
}

pub fn print_skills(profile: &Profile) {
    println!();
    println!("{}", box_top("╔", "╗").cyan());
    println!(
        "{}",
        row_colored(format!("  {}", "  SKILLS ACTIVAS".bold())).cyan()
    );
    println!("{}", box_sep().cyan());

    let active = profile.active();
    if active.is_empty() {
        println!(
            "{}",
            row_colored(format!(
                "  {}",
                "Ninguna. Usa /skills +<id> para activar.".yellow()
            ))
        );
    } else {
        for s in &active {
            println!(
                "{}",
                row_colored(format!(
                    "  {}  {:<16} {}",
                    "✓".green(),
                    s.name.cyan(),
                    s.category.label().dimmed()
                ))
            );
        }
    }

    println!("{}", box_sep().cyan());
    println!(
        "{}",
        row_colored(format!("  {}", "DISPONIBLES".bold())).cyan()
    );
    println!("{}", box_sep().cyan());

    for s in profile.inactive() {
        println!(
            "{}",
            row_colored(format!(
                "  {}  {:<16} {}",
                " ",
                s.name.dimmed(),
                s.category.label().dimmed()
            ))
        );
    }

    println!("{}", format!("╚{}╝", line('─')).cyan());
    println!(
        "{}",
        "  /skills +<id> activar   /skills -<id> desactivar   /skills ?<id> ver detalle".dimmed()
    );
    println!();
}

pub fn print_skill_detail(id: &str) -> bool {
    let skill = match crate::skills::find(id) {
        Some(s) => s,
        None => {
            println!("  {}", format!("Skill desconocida: {}", id).red());
            return false;
        }
    };

    println!();
    println!("{}", box_top("╔", "╗").cyan());
    println!(
        "{}",
        row_colored(format!(
            "  {}  [{}]",
            skill.name.bold(),
            skill.category.label().dimmed()
        ))
        .cyan()
    );
    println!("{}", box_sep().cyan());
    println!("{}", row_colored(format!("  {}", skill.description)));
    println!("{}", box_sep().cyan());
    println!("{}", row_colored(format!("  {}", "INSTRUCCIONES".bold())));
    println!("{}", box_sep().cyan());

    for l in skill.instructions.lines() {
        println!("{}", row_colored(format!("  {}", l)));
    }

    if !skill.tools.is_empty() {
        println!("{}", box_sep().cyan());
        println!(
            "{}",
            row_colored(format!(
                "  {}  {}",
                "HERRAMIENTAS".bold(),
                skill.tools.join(", ").cyan()
            ))
        );
    }

    if !skill.permission_hints.is_empty() {
        println!("{}", box_sep().cyan());
        let perms: Vec<String> = skill
            .permission_hints
            .iter()
            .map(|(tool, level)| {
                let label = match level {
                    crate::permissions::PermissionLevel::Auto => "auto",
                    crate::permissions::PermissionLevel::Ask => "ask",
                    crate::permissions::PermissionLevel::Deny => "deny",
                };
                format!("{}={}", tool, label)
            })
            .collect();
        println!(
            "{}",
            row_colored(format!(
                "  {}  {}",
                "PERMISOS".bold(),
                perms.join(", ").yellow()
            ))
        );
    }

    println!("{}", format!("╚{}╝", line('─')).cyan());
    println!();
    true
}
