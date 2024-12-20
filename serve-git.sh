#!/bin/bash

# Check for environment variables file
if [ -f .env ]; then
    source .env
else
    echo -e ".env not found\nUsing default repository"
fi

# Initialize defaults if not set in .env
REMOTE=${REMOTE:="demo.git"}
PORT=${PORT:=5005}

# Resolve localhost name of machine to avoid use of ephemeral IP
SERVING_HOST="$(scutil --get LocalHostName).local:$PORT/$REMOTE"

# Initialize repository if it does not exist
if [ ! -d "$REMOTE" ]; then
    (
        mkdir $REMOTE
        chmod -R 755 $REMOTE
        cd $_
        git init --bare --shared
        git --bare update-server-info
        mv hooks/post-update.sample hooks/post-update
     )
fi

# Start repository webdav server
echo "Serving repository at http://$SERVING_HOST"
rclone serve webdav . --addr :$PORT &> server-git.log
