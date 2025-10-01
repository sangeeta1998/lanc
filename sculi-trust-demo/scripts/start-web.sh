#!/bin/bash
cd web
if command -v python3 &> /dev/null; then
    python3 -m http.server 8080
elif command -v python &> /dev/null; then
    python -m SimpleHTTPServer 8080
elif command -v node &> /dev/null; then
    npx http-server -p 8080
else
    echo "No web server found. Please install Python or Node.js"
    exit 1
fi
