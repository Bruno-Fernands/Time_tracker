use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
}

impl Session {
    pub fn new(start: DateTime<Local>) -> Self {
        Session { start, end: None }
    }

    /// Duração da sessão. Retorna None se o timer ainda está rodando.
    pub fn duration(&self) -> Option<Duration> {
        self.end.map(|end| end - self.start)
    }

    /// Timer ainda está aberto (sem fim registrado)?
    pub fn is_open(&self) -> bool {
        self.end.is_none()
    }
}


pub fn has_open_session(sessions: &[Session]) -> bool {
    sessions.iter().any(|s| s.is_open())
}

pub fn completed_sessions(sessions: &[Session]) -> Vec<&Session> {
    sessions.iter().filter(|s| !s.is_open()).collect()
}

pub fn total_duration(sessions: &[Session]) -> Duration {
    completed_sessions(sessions)
        .iter()
        .filter_map(|s| s.duration())
        .fold(Duration::zero(), |acc, d| acc + d)
}

pub fn count_completed(sessions: &[Session]) -> usize {
    completed_sessions(sessions).len()
}


pub fn format_duration(d: Duration) -> String {
    let total_secs = d.num_seconds().max(0);
    let hours   = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let secs    = total_secs % 60;

    if hours > 0 {
        format!("{}h {:02}m {:02}s", hours, minutes, secs)
    } else {
        format!("{:02}m {:02}s", minutes, secs)
    }
}


pub fn data_file() -> PathBuf {
    let base = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    let dir = base.join("tt");
    fs::create_dir_all(&dir).expect("Não foi possível criar o diretório de dados");
    dir.join("sessions.json")
}

pub fn load_sessions(path: &PathBuf) -> Vec<Session> {
    if !path.exists() {
        return vec![];
    }
    let content = fs::read_to_string(path).expect("Erro ao ler sessions.json");
    serde_json::from_str(&content).unwrap_or_default()
}

pub fn save_sessions(path: &PathBuf, sessions: &[Session]) {
    let content = serde_json::to_string_pretty(sessions)
        .expect("Erro ao serializar sessões");
    fs::write(path, content).expect("Erro ao salvar sessions.json");
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn make_session(start_secs: i64, end_secs: Option<i64>) -> Session {
        let base = Local.timestamp_opt(0, 0).unwrap(); 
        Session {
            start: base + Duration::seconds(start_secs),
            end: end_secs.map(|e| base + Duration::seconds(e)),
        }
    }


    #[test]
    fn test_format_duration_horas_minutos_segundos() {
        let d = Duration::seconds(9015);
        assert_eq!(format_duration(d), "2h 30m 15s");
    }

    #[test]
    fn test_format_duration_exatamente_uma_hora() {
        let d = Duration::seconds(3600);
        assert_eq!(format_duration(d), "1h 00m 00s");
    }

    #[test]
    fn test_format_duration_so_minutos() {
        let d = Duration::seconds(300); 
        assert_eq!(format_duration(d), "05m 00s");
    }

    #[test]
    fn test_format_duration_zero() {
        let d = Duration::seconds(0);
        assert_eq!(format_duration(d), "00m 00s");
    }

    #[test]
    fn test_session_is_open_retorna_true_sem_fim() {
        let s = make_session(0, None);
        assert!(s.is_open());
    }

    #[test]
    fn test_session_is_open_retorna_false_com_fim() {
        let s = make_session(0, Some(1800));
        assert!(!s.is_open());
    }

    #[test]
    fn test_session_duration_retorna_valor_correto() {
        let s = make_session(0, Some(1800)); 
        assert_eq!(s.duration().unwrap(), Duration::seconds(1800));
    }

    #[test]
    fn test_total_duration_soma_multiplas_sessoes() {
        let sessions = vec![
            make_session(0, Some(1800)),
            make_session(2000, Some(3800)),
            make_session(4000, Some(5800)),
        ];
        assert_eq!(total_duration(&sessions), Duration::seconds(5400));
    }

    #[test]
    fn test_count_completed_conta_apenas_encerradas() {
        let sessions = vec![
            make_session(0, Some(1800)),  
            make_session(2000, None),     
        ];
        assert_eq!(count_completed(&sessions), 1);
    }

    #[test]
    fn test_has_open_session_retorna_true_quando_ha_aberta() {
        let sessions = vec![
            make_session(0, Some(1800)),
            make_session(2000, None),
        ];
        assert!(has_open_session(&sessions));
    }


    #[test]
    fn test_format_duration_59_segundos_sem_horas() {
        let d = Duration::seconds(59);
        assert_eq!(format_duration(d), "00m 59s");
    }

    #[test]
    fn test_format_duration_exatamente_60_segundos() {
        let d = Duration::seconds(60);
        assert_eq!(format_duration(d), "01m 00s");
    }

    #[test]
    fn test_format_duration_duracao_muito_grande() {
        let d = Duration::seconds(360000); 
        assert_eq!(format_duration(d), "100h 00m 00s");
    }

    #[test]
    fn test_format_duration_negativa_tratada_como_zero() {
        let d = Duration::seconds(-100);
        assert_eq!(format_duration(d), "00m 00s");
    }

    #[test]
    fn test_session_duration_retorna_none_quando_aberta() {
        let s = make_session(0, None);
        assert!(s.duration().is_none());
    }

    #[test]
    fn test_session_duration_zero_quando_inicio_igual_fim() {
        let s = make_session(100, Some(100));
        assert_eq!(s.duration().unwrap(), Duration::seconds(0));
    }

    #[test]
    fn test_total_duration_vec_vazio_retorna_zero() {
        let sessions: Vec<Session> = vec![];
        assert_eq!(total_duration(&sessions), Duration::zero());
    }

    #[test]
    fn test_total_duration_ignora_sessoes_abertas() {
        let sessions = vec![
            make_session(0, Some(1800)), 
            make_session(2000, None),    
        ];
        assert_eq!(total_duration(&sessions), Duration::seconds(1800));
    }

    #[test]
    fn test_has_open_session_retorna_false_quando_todas_encerradas() {
        let sessions = vec![
            make_session(0, Some(1800)),
            make_session(2000, Some(3800)),
        ];
        assert!(!has_open_session(&sessions));
    }

    #[test]
    fn test_has_open_session_retorna_false_em_vec_vazio() {
        let sessions: Vec<Session> = vec![];
        assert!(!has_open_session(&sessions));
    }
}
