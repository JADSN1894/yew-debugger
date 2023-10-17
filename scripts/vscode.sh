#!/bin/sh

# -- T0 VsCode
sudo apt-get autoremove --purge code --yes
rm -rfv $HOME/.vscode $HOME/.config/Code
sudo apt-get install code --yes
code --install-extension ms-vscode-remote.remote-containers

# -- T0 Docker
docker image ls | awk '/^vsc/{print $3}' | xargs docker rmi --force
docker volume rm --force vscode
docker system prune --all --volumes --force
