#!/bin/bash
# Script to reset Docker Compose environment: removes containers, defined volumes, and restarts

# Stop execution on error
set -e

cd "$(dirname "$0")"

# Set path to docker-compose file (default to current directory)
COMPOSE_FILE="./docker-compose.yml"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    -f|--file)
      COMPOSE_FILE="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      echo "Usage: $0 [-f|--file docker-compose-file.yml]"
      exit 1
      ;;
  esac
done

# Check if docker-compose file exists
if [ ! -f "$COMPOSE_FILE" ]; then
  echo "Error: Docker Compose file '$COMPOSE_FILE' not found!"
  exit 1
fi

echo "🛑 Stopping containers defined in $COMPOSE_FILE..."
docker compose -f "$COMPOSE_FILE" down

# Extract volume names from docker-compose file using grep and awk
echo "📋 Extracting volume names from $COMPOSE_FILE..."
VOLUMES=$(awk '/^volumes:/{flag=1; next} flag && /^[a-zA-Z_]/{flag=0} flag && /^  [a-zA-Z0-9_-]+:/{gsub(/^  /, ""); gsub(/:.*/, ""); print}' "$COMPOSE_FILE")

if [ -z "$VOLUMES" ]; then
  echo "⚠️ No named volumes found in $COMPOSE_FILE"
else
  echo "🗑️ Removing volumes:"
  for VOLUME in $VOLUMES; do
    echo "  - Removing $VOLUME..."
    docker volume rm "local_$VOLUME" 2>/dev/null || echo "    ⚠️ Could not remove volume local_$VOLUME (might not exist)"
  done
fi

echo "🚀 Starting containers with docker compose up..."
docker compose -f "$COMPOSE_FILE" up -d

echo "✅ Done! All containers have been recreated with fresh volumes."
echo "📊 Container status:"
docker compose -f "$COMPOSE_FILE" ps
