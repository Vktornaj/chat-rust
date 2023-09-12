#! /bin/bash

set -e

remote_host="192.168.1.116"
remote_port=22
remote_user="vktornaj"
local_path_app=$(pwd)/
local_path_compose=$(pwd)/compose/
remote_path_app="/home/${remote_user}/chat_rust/"
remote_path_compose="/home/${remote_user}/compose_projects"
ssh_key="~/Desktop/Files/vktserver/key_01"
tag='0.0.6'

# upadte source code
echo "Cleaning"
ssh -i ${ssh_key} ${remote_user}@${remote_host} rm -rf chat_rust/
echo "Sending data"
rsync -avzr --exclude='.git/' --exclude='target/' --delete -e "ssh -p ${remote_port} -i ${ssh_key} -o StrictHostKeyChecking=no" ${local_path_app} ${remote_user}@${remote_host}:${remote_path_app}

# Build and tag docker image
echo "Building docker image"
ssh -tt -i ${ssh_key} ${remote_user}@${remote_host} << EOF 
docker build -t chat-rust:${tag} ${remote_path_app}
exit
EOF

# clean and update compose_projects
ssh -i ${ssh_key} ${remote_user}@${remote_host} rm -rf compose_projects/
rsync -avzr --exclude='.git/' --delete -e "ssh -p ${remote_port} -i ${ssh_key} -o StrictHostKeyChecking=no" ${local_path_compose} ${remote_user}@${remote_host}:${remote_path_compose}

# docker compose -f compose_projects/compose.yml build 
ssh -tt -i ${ssh_key} ${remote_user}@${remote_host} << EOF 
docker compose -f compose_projects/compose.yml down || true
docker compose -f compose_projects/compose.yml --env-file ~/compose_projects/config/.env up -d
exit
EOF

say "deploy done"