from pymongo import MongoClient
import time

# Attendi per dare al server MongoDB nel container il tempo di avviarsi completamente
time.sleep(5)

# Connessione al server MongoDB nel container Docker
client = MongoClient("mongodb://127.0.0.1:27017/")

# Creazione del database e della collezione se non esistono
db = client["storage"]
collection = db["kpi_collection"]

dba = client["raw-data"]
collectiona = dba["sensors"]

# Crea una lista di kpis da 1 a 5
kpis = [
    {"name": f"kpi{i}", "id": i, "formula": f"Formula {i}", "unit": f"Unit {i}", "counter": "0"}
    for i in range(1, 6)
]

sensors = [
    {"id_sensor": j, "name_sensor": f"sensor{j}", "temperature": "5"}
    for j in range (1,10)
]

# Cancella tutti gli elementi nella collezione
#delete_result = collection.delete_many({})

# Inserisci i documenti nella collezione
result = collection.insert_many(kpis)
resultb = collectiona.insert_many(sensors)

# Stampa del documento inserito
print(f"Inserted documents IDs: {result.inserted_ids}")
print(f"Inserted documents IDs: {resultb.inserted_ids}")

# Recupera tutti gli oggetti nella collezione
all_documents = collection.find()
all_documentsa = collectiona.find()
# Stampa tutti gli oggetti
for i in all_documents:
    print(i)
    
for j in all_documentsa:
    print(j)

# Chiudi la connessione
client.close()
