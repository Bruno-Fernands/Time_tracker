# tt — Time Tracker

## Funcionalidades

| Comando       | Descrição                                     |
|---------------|-----------------------------------------------|
| tt start      | Inicia o timer                                |
| tt stop       | Para o timer e salva a sessão                 |
| tt status     | Mostra se o timer está ativo e há quanto tempo|
| tt log        | Lista todas as sessões registradas            |
| tt summary    | Mostra o total de tempo acumulado             |

---

## Instalação

### Compilar e instalar

```bash
git clone https://github.com/Bruno-Fernands/Time_tracker.git
cd Time_tracker
cargo install --path .
```
---

## Pipeline CI/CD

O pipeline (GitHub Actions) executa automaticamente a cada push na branch main:

| Job       | O que faz                                      |
|-----------|------------------------------------------------|
| test      | Executa os 20 testes unitários (cargo test)    |
| build     | Compila o binário release                      |
| notify    | Envia e-mail com o resultado do pipeline       |
| deploy    | Publica o binário como GitHub Release          |

O deploy e a notificação só ocorrem após test e build finalizarem.  
O deploy só executa se ambos tiverem passado.
