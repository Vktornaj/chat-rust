#! /bin/bash

set -e

remote_host="3.137.167.229"
remote_port=22
remote_user="admin"
local_path=$(pwd)/
remote_path="/home/${remote_user}/chat_rust/"
ssh_key="~/Desktop/Files/aws_keys/key_001.pem"
tag='0.0.6'


# Upadte source code
echo "Cleaning"
ssh -i ${ssh_key} ${remote_user}@${remote_host} sudo rm -rf chat_rust/
echo "Sending data"
rsync -avzr --exclude='.git/' --exclude='target/' --delete -e "ssh -p ${remote_port} -i ${ssh_key} -o StrictHostKeyChecking=no" ${local_path} ${remote_user}@${remote_host}:${remote_path}

# Build, tag and push docker image to aws ecr
echo "Pushing docker image"
ssh -tt -i ${ssh_key} ${remote_user}@${remote_host} << EOF 
aws ecr get-login-password --region us-east-2 | sudo docker login --username AWS --password-stdin 569233066229.dkr.ecr.us-east-2.amazonaws.com
sudo docker build -t chat-rust:${tag} ${remote_path}
sudo docker tag chat-rust:${tag} 569233066229.dkr.ecr.us-east-2.amazonaws.com/chat-rust:${tag}
sudo docker push 569233066229.dkr.ecr.us-east-2.amazonaws.com/chat-rust:${tag}
exit
EOF

## only for Mac OS:
say "image created"