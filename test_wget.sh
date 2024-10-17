#!/bin/bash

# Couleurs ANSI pour la sortie du terminal
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # Pas de couleur

# Fonction pour afficher le succès ou l'échec
success() {
    echo -e "${GREEN}RÉUSSI${NC}"
}

failure() {
    echo -e "${RED}ÉCHOUÉ${NC}"
}

# Step 1: Build the project
echo "Building the project..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed. Exiting."
    exit 1
fi
success

# Fonction pour tester le téléchargement d'un fichier
test_download() {
    local url=$1
    local expected_file=$2
    echo "Testing download: $url"
    ./target/release/wget "$url"

    # Vérification de l'existence du fichier
    if [ -f "$expected_file" ]; then
        success
    else
        failure
    fi
}

# Test 1: Téléchargement d'une image
test_download "https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg" "downloads/EMtmPFLWkAA8CIS.jpg"

# Test 2: Téléchargement d'un fichier au choix
test_download "https://golang.org/dl/go1.16.3.linux-amd64.tar.gz" "downloads/go1.16.3.linux-amd64.tar.gz"

# Test 3: Téléchargement d'un gros fichier
echo "Testing large file download..."
./target/release/wget http://ipv4.download.thinkbroadband.com/100MB.zip
if [ -f "downloads/100MB.zip" ]; then
    success
else
    failure
fi

# Test 4: Test du fichier de sortie personnalisé
echo "Testing output file..."
./target/release/wget -O=downloads/test_20MB.zip http://ipv4.download.thinkbroadband.com/20MB.zip
if [ -f "downloads/test_20MB.zip" ]; then
    success
else
    failure
fi

# Test 5: Téléchargement dans un répertoire personnalisé
echo "Testing custom directory..."
DOWNLOAD_DIR="$HOME/Downloads/"
mkdir -p "$DOWNLOAD_DIR"
./target/release/wget -O=test_20MB.zip -P="$DOWNLOAD_DIR" http://ipv4.download.thinkbroadband.com/20MB.zip
if [ -f "$DOWNLOAD_DIR/test_20MB.zip" ]; then
    success
else
    failure
fi

# Test 6: Limitation de vitesse
echo "Testing rate limit of 300KB/s..."
./target/release/wget --rate-limit=300k http://ipv4.download.thinkbroadband.com/20MB.zip
success

echo "Testing rate limit of 700KB/s..."
./target/release/wget --rate-limit=700k http://ipv4.download.thinkbroadband.com/20MB.zip
success

echo "Testing rate limit of 2MB/s..."
./target/release/wget --rate-limit=2M http://ipv4.download.thinkbroadband.com/20MB.zip
success

# Test 7: Téléchargement en lot depuis une liste de fichiers
echo "Testing batch download from list..."
echo -e "https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg\nhttp://ipv4.download.thinkbroadband.com/20MB.zip\nhttp://ipv4.download.thinkbroadband.com/10MB.zip" > downloads.txt
./target/release/wget -i=downloads.txt
if [ -f "downloads/EMtmPFLWkAA8CIS.jpg" ] && [ -f "downloads/20MB.zip" ] && [ -f "downloads/10MB.zip" ]; then
    success
else
    failure
fi

# Test 8: Téléchargement silencieux avec le flag -B
echo "Testing silent download with background flag..."
./target/release/wget -B http://ipv4.download.thinkbroadband.com/20MB.zip
if [ -f "wget-log" ]; then
    success
else
    failure
fi

# Vérifier le fichier de log
if [ -f "wget-log" ]; then
    echo "Log file created successfully:"
    success
    cat wget-log
else
    echo "Log file not found."
    failure
fi

# Test 9: Test du mode miroir
echo "Testing mirror functionality..."
./target/release/wget --mirror --convert-links http://corndog.io/
if [ -f "./corndog.io/index.html" ]; then
    success
else
    failure
fi

./target/release/wget --mirror https://oct82.com/
if [ -f "./oct82.com/index.html" ]; then
    success
else
    failure
fi

./target/release/wget --mirror --reject=gif https://oct82.com/
if [ -f "./oct82.com/index.html" ] && [ ! -f "./oct82.com/*.gif" ]; then
    success
else
    failure
fi

./target/release/wget --mirror https://trypap.com/
if [ -d "./trypap.com" ]; then
    success
else
    failure
fi

./target/release/wget --mirror -X=/img https://trypap.com/
if [ -f "./trypap.com/index.html" ] && [ ! -d "./trypap.com/img" ]; then
    success
else
    failure
fi

echo "All tests completed."
