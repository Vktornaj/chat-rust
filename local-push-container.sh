#! /bin/bash

set -e

local_path=$(pwd)

echo "Pushing docker image"

# Login to AWS ECR
password=$(aws ecr get-login-password --region us-east-2)
echo $password | docker login --username AWS --password-stdin 569233066229.dkr.ecr.us-east-2.amazonaws.com

# Delete latest tag
aws ecr batch-delete-image --region us-east-2 --repository-name chat-rust --image-ids imageTag=latest > /dev/null 2>&1

LATEST_TAGS=$(aws ecr describe-images --region us-east-2 --repository-name chat-rust --query 'sort_by(imageDetails,& imagePushedAt)[-1].imageTags' --output json)
# Use jq to parse the JSON and remove the quotes around the tags
TAGS=$(echo $LATEST_TAGS | jq -r '.[]')

BIGGEST_TAG=""

# Loop over each tag
for TAG in $TAGS; do
  # Split the tag into major, minor, and patch versions
  IFS='.' read -ra VERSION <<< "$TAG"

  # Convert the version parts to an integer that can be compared
  INT_VERSION=$((VERSION[0]*10000 + VERSION[1]*100 + VERSION[2]))

  # If this version is bigger than the biggest version found so far, update the biggest version and tag
  if (( INT_VERSION > BIGGEST_VERSION )); then
    BIGGEST_VERSION=$INT_VERSION
    BIGGEST_TAG=$TAG
  fi
done

# Split the tag into major, minor, and patch versions
IFS='.' read -ra VERSION <<< "$BIGGEST_TAG"

# Increment the patch version
PATCH=$((VERSION[2] + 1))

# Construct the new tag
NEW_TAG="${VERSION[0]}.${VERSION[1]}.$PATCH"

docker build -t chat-rust:${NEW_TAG} ${local_path}
docker tag chat-rust:${NEW_TAG} 569233066229.dkr.ecr.us-east-2.amazonaws.com/chat-rust:${NEW_TAG}
docker push 569233066229.dkr.ecr.us-east-2.amazonaws.com/chat-rust:${NEW_TAG}

# # Set latest tag to new tag
MANIFEST=$(aws ecr batch-get-image --region us-east-2 --repository-name chat-rust --image-ids imageTag=$NEW_TAG --output text --query 'images[].imageManifest')
aws ecr put-image --region us-east-2 --repository-name chat-rust --image-tag latest --image-manifest "$MANIFEST" > /dev/null 2>&1

## only for Mac OS:
# say "container pushed to aws ecr"