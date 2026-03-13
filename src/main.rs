use clap::{Parser, Subcommand};              
use chrono::{DateTime, Duration, Local};      
use serde::{Deserialize, Serialize};           
use std::{fs, path::PathBuf};                  

#[derive(Parser)]
#[command(name = "tt", about = "Time tracker de linha de comando")]
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


#[derive(Serialize, Deserialize, Clone)]
struct Session {
    start: DateTime<Local>,
    end: Option<DateTime<Local>>,
}

impl Session {

    fn duration(&self) -> Option<Duration> {
        self.end.map(|end| end - self.start)
    }

    fn is_open(&self) -> bool {
        self.end.is_none()
    }
}

fn data_file() -> PathBuf {
    let base = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from(".")); 

    let dir = base.join("tt");
    fs::create_dir_all(&dir).expect("Não foi possível criar o diretório de dados");

    dir.join("sessions.json")
}

fn load_sessions(path: &PathBuf) -> Vec<Session> {
    if !path.exists() {
        return vec![];
    }
    let content = fs::read_to_string(path).expect("Erro ao ler sessions.json");
    serde_json::from_str(&content).unwrap_or_default()
}

fn save_sessions(path: &PathBuf, sessions: &[Session]) {
    let content = serde_json::to_string_pretty(sessions)
        .expect("Erro ao serializar sessões");
    fs::write(path, content).expect("Erro ao salvar sessions.json");
}

fn format_duration(d: Duration) -> String {
    let total_secs = d.num_seconds();
    let hours   = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let secs    = total_secs % 60;

    if hours > 0 {
        format!("{}h {:02}m {:02}s", hours, minutes, secs)
    } else {
        format!("{:02}m {:02}s", minutes, secs)
    }
}


fn main() {
    let cli = Cli::parse();           
    let path = data_file();          
    let mut sessions = load_sessions(&path);

    match cli.command {

        Command::Start => {
            if sessions.iter().any(|s| s.is_open()) {
                eprintln!("Já existe um timer rodando! Use 'tt stop' primeiro.");
                std::process::exit(1);
            }

            let now = Local::now();
            sessions.push(Session { start: now, end: None });
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
            let done: Vec<&Session> = sessions.iter().filter(|s| !s.is_open()).collect();

            if done.is_empty() {
                println!("Nenhuma sessão registrada ainda.");
                return;
            }

            println!("{:<22} {:<22} {}", "Início", "Fim", "Duração");
            println!("{}", "─".repeat(58));

            for s in &done {
                println!(
                    "{:<22} {:<22} {}",
                    s.start.format("%d/%m/%Y %H:%M:%S"),
                    s.end.unwrap().format("%d/%m/%Y %H:%M:%S"),
                    format_duration(s.duration().unwrap())
                );
            }
        }

        Command::Summary => {
            let done: Vec<&Session> = sessions.iter().filter(|s| !s.is_open()).collect();

            if done.is_empty() {
                println!("Nenhuma sessão registrada ainda.");
                return;
            }

            let total: Duration = done
                .iter()
                .filter_map(|s| s.duration())
                .fold(Duration::zero(), |acc, d| acc + d);

            println!("Sessões registradas : {}", done.len());
            println!("Tempo total         : {}", format_duration(total));
        }
    }
}
