#! /bin/bash

set -e

local_path=$(pwd)/
tag='0.0.7'

# Build, tag and push docker image to aws ecr
echo "Pushing docker image"

aws ecr get-login-password --region us-east-2 | docker login --username AWS --password-stdin 569233066229.dkr.ecr.us-east-2.amazonaws.com
docker build -t chat-rust:${tag} ${local_path}
docker tag chat-rust:${tag} 569233066229.dkr.ecr.us-east-2.amazonaws.com/chat-rust:${tag}
docker push 569233066229.dkr.ecr.us-east-2.amazonaws.com/chat-rust:${tag}

## only for Mac OS:
say "image created"