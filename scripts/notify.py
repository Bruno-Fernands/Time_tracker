#!/usr/bin/env python3

import smtplib
import sys
import os
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart

def main():
    # Le argumentos 
    if len(sys.argv) < 3:
        print("Uso: notify.py <status> <run_url>")
        sys.exit(1)

    status  = sys.argv[1]   
    run_url = sys.argv[2]   

    # Lê variáveis de ambiente 
    to_email      = os.environ.get("NOTIFY_EMAIL")
    smtp_user     = os.environ.get("SMTP_USER")
    smtp_password = os.environ.get("SMTP_PASSWORD")

    if not all([to_email, smtp_user, smtp_password]):
        print("Erro: NOTIFY_EMAIL, SMTP_USER e SMTP_PASSWORD devem estar definidos.")
        sys.exit(1)

    # Monta o email 
    emoji  = "✅" if status == "success" else "❌"
    subject = f"{emoji} tt CI/CD — {status.upper()}"

    body = f"""
Pipeline do projeto tt finalizado.

Status : {status.upper()}
Detalhes: {run_url}
"""

    msg = MIMEMultipart()
    msg["From"]    = smtp_user
    msg["To"]      = to_email
    msg["Subject"] = subject
    msg.attach(MIMEText(body, "plain"))

    # Envia via gmail SMTP 
    try:
        with smtplib.SMTP_SSL("smtp.gmail.com", 465) as server:
            server.login(smtp_user, smtp_password)
            server.sendmail(smtp_user, to_email, msg.as_string())
        print(f"E-mail enviado para {to_email} ({status})")
    except Exception as e:
        print(f"Falha ao enviar e-mail: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
