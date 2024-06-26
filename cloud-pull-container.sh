#! /bin/bash

set -e

remote_host="3.138.154.229"
remote_port=22
remote_user="admin"
local_path_app=$(pwd)/compose/qa/
remote_path_app="/home/${remote_user}/compose"
ssh_key="~/projects/aws/key_001.pem"


# docker down
ssh -tt -i ${ssh_key} ${remote_user}@${remote_host} << EOF 
docker compose -f ${remote_path_app}/compose.yml down || true
exit
EOF

# upadte compose code
echo "Cleaning"
ssh -i ${ssh_key} ${remote_user}@${remote_host} rm -rf ${remote_path_app}
echo "Sending data"
rsync -avzr --exclude='.git/' --exclude='target/' --delete -e "ssh -p ${remote_port} -i ${ssh_key} -o StrictHostKeyChecking=no" ${local_path_app} ${remote_user}@${remote_host}:${remote_path_app}


# docker compose -f compose_projects/compose.yml build
ssh -tt -i ${ssh_key} ${remote_user}@${remote_host} << EOF 
aws ecr get-login-password --region us-east-2 | docker login --username AWS --password-stdin 569233066229.dkr.ecr.us-east-2.amazonaws.com
docker compose -f ${remote_path_app}/compose.yml --env-file ${remote_path_app}/config/.env up -d --build
exit
EOF

# say "container pulled from aws ecr"
