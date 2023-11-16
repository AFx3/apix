#!/bin/bash

# Avvia MongoDB e aspetta che sia pronto
mongod &

# Aspetta che MongoDB sia avviato completamente (potrebbe essere necessario un tempo maggiore)
sleep 10

# Mantieni il container in esecuzione
tail -f /dev/null