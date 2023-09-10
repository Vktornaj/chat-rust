#! /bin/bash

set -e

remote_host="18.224.68.2"
remote_port=22
remote_user="admin"
local_dir=$(pwd)/
remote_dir="/home/${remote_user}/compose_projects"
ssh_key="~/Desktop/Files/aws_keys/key_001.pem"

ssh -i ${ssh_key} ${remote_user}@${remote_host} sudo rm -rf compose_projects/
rsync -avzr --exclude='.git/' --delete -e "ssh -p ${remote_port} -i ${ssh_key} -o StrictHostKeyChecking=no" ${local_dir} ${remote_user}@${remote_host}:${remote_dir}

# docker compose -f compose_projects/compose.yml build 
ssh -tt -i ${ssh_key} ${remote_user}@${remote_host} << EOF 
sudo docker compose -f compose_projects/compose.yml down || true
sudo docker compose -f compose_projects/compose.yml --env-file ~/compose_projects/config/.env up -d
exit
EOF

say "deploy done"