// main.rs — apenas a interface de linha de comando.
// Toda a lógica vive em lib.rs para ser testável.

use clap::{Parser, Subcommand};
use chrono::Local;
use tt::{
    data_file, load_sessions, save_sessions,
    has_open_session, completed_sessions, total_duration, count_completed,
    format_duration, Session,
};

#[derive(Parser)]
#[command(name = "tt", about = "Time tracker de linha de comando", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Start,
    Stop,
    Status,
    Log,
    Summary,
}

fn main() {
    let cli = Cli::parse();
    let path = data_file();
    let mut sessions = load_sessions(&path);

    match cli.command {
        Command::Start => {
            if has_open_session(&sessions) {
                eprintln!("Já existe um timer rodando! Use 'tt stop' primeiro.");
                std::process::exit(1);
            }
            let now = Local::now();
            sessions.push(Session::new(now));
            save_sessions(&path, &sessions);
            println!("▶ Timer iniciado às {}", now.format("%H:%M:%S"));
        }

        Command::Stop => {
            match sessions.iter_mut().find(|s| s.is_open()) {
                None => {
                    eprintln!("Nenhum timer rodando. Use 'tt start' primeiro.");
                    std::process::exit(1);
                }
                Some(session) => {
                    let now = Local::now();
                    session.end = Some(now);
                    let duration = session.duration().unwrap();
                    save_sessions(&path, &sessions);
                    println!("■ Timer parado. Sessão durou {}", format_duration(duration));
                }
            }
        }

        Command::Status => {
            match sessions.iter().find(|s| s.is_open()) {
                None => println!("Nenhum timer rodando."),
                Some(session) => {
                    let elapsed = Local::now() - session.start;
                    println!(
                        "▶ Rodando desde {} — {} decorrido",
                        session.start.format("%H:%M:%S"),
                        format_duration(elapsed)
                    );
                }
            }
        }

        Command::Log => {
            let done = completed_sessions(&sessions);
            if done.is_empty() {
                println!("Nenhuma sessão registrada ainda.");
                return;
            }
            println!("{:<22} {:<22} {}", "Início", "Fim", "Duração");
            println!("{}", "─".repeat(58));
            for s in done {
                println!(
                    "{:<22} {:<22} {}",
                    s.start.format("%d/%m/%Y %H:%M:%S"),
                    s.end.unwrap().format("%d/%m/%Y %H:%M:%S"),
                    format_duration(s.duration().unwrap())
                );
            }
        }

        Command::Summary => {
            let n = count_completed(&sessions);
            if n == 0 {
                println!("Nenhuma sessão registrada ainda.");
                return;
            }
            println!("Sessões registradas : {}", n);
            println!("Tempo total         : {}", format_duration(total_duration(&sessions)));
        }
    }
}
