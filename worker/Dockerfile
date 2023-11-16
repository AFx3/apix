# Usa l'immagine di Python
FROM python:3.8

# Imposta il working directory
WORKDIR /app

# Copia i file necessari
COPY . .

# Installa le dipendenze necessarie
RUN pip install -r requirements.txt

# Comando di avvio dello script quando il container viene eseguito
CMD ["python", "mongo_server.py"]