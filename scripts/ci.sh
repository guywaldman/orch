#!/bin/bash

set -e

for i in 1 2 3; do
	systemctl is-active ollama.service && sudo systemctl stop ollama.service
	curl -fsSL https://ollama.com/install.sh | sh
	sleep 5
	if systemctl is-active ollama.service; then
		break
	fi
done

ollama serve &
ollama pull phi3:mini
ollama pull nomic-embed-text:latest
