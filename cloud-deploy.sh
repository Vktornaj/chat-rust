#! /bin/bash

set -e

remote_host="3.138.154.229"
remote_port=22
remote_user="admin"
local_path_app=$(pwd)/
local_path_compose=$(pwd)/compose/
remote_path_app="/home/${remote_user}/chat_rust"
ssh_key="~/Desktop/Files/aws_keys/key_001.pem"


# docker down
ssh -tt -i ${ssh_key} ${remote_user}@${remote_host} << EOF 
docker compose -f ${remote_path_app}/compose/compose.yml down || true
exit
EOF

# upadte source code
echo "Cleaning"
ssh -i ${ssh_key} ${remote_user}@${remote_host} rm -rf chat_rust/
echo "Sending data"
rsync -avzr --exclude='.git/' --exclude='target/' --delete -e "ssh -p ${remote_port} -i ${ssh_key} -o StrictHostKeyChecking=no" ${local_path_app} ${remote_user}@${remote_host}:${remote_path_app}


# docker compose -f compose_projects/compose.yml build
ssh -tt -i ${ssh_key} ${remote_user}@${remote_host} << EOF 
docker compose -f ${remote_path_app}/compose/compose.yml --env-file ${remote_path_app}/compose/config/.env up -d --build
exit
EOF

say "deploy done"