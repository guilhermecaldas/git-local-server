# Git Local Server

Serves a Git bare repository over local LAN using `rclone` webdav server

## Getting started

Install rclone by downloading from https://rclone.org/downloads/

## Serving

1. Place the script `serve-git.sh` in a directory that will hold your
   repositories, like `repos/`.
2. Copy the file `.env.sample` to `.env` and set your preferred repository name
   and port
3. Execute the script `./serve-git.sh`. It will create a repository whose name
   is set in `REMOTE`, in the current directory, and serve it on the port
   `PORT` (defaults to 5005)
4. Clone your repository, eg.: `git clone http://yourhost:5005/repo-name.git repo-name`

## Notes

- This script was tested on MacOS Sequoia. For any bugs, report to
  gcaldas@chemis.tech
- Don't use as a public solution. The intention of this script is to serve
  locally in a private environment to private machines
