#!/bin/bash

# Set variables
REPO_URL="https://github.com/davidemerson/cherubgyre.git"
ECR_REPO_NAME="cherubgyre"
ECS_CLUSTER_NAME="cherubgyre-cluster"
ECS_SERVICE_NAME="cherubgyre-service"
ECS_TASK_DEFINITION_NAME="cherubgyre-task"

# Clone the repository
git clone $REPO_URL
cd cherubgyre

# Get the AWS account ID
AWS_ACCOUNT_ID=$(aws sts get-caller-identity --query Account --output text)

# Create ECR repository if it doesn't exist
aws ecr describe-repositories --repository-names ${ECR_REPO_NAME} || aws ecr create-repository --repository-name ${ECR_REPO_NAME}

# Build and push Docker image to ECR
aws ecr get-login-password --region $(aws configure get region) | docker login --username AWS --password-stdin ${AWS_ACCOUNT_ID}.dkr.ecr.$(aws configure get region).amazonaws.com
docker build -t ${ECR_REPO_NAME} .
docker tag ${ECR_REPO_NAME}:latest ${AWS_ACCOUNT_ID}.dkr.ecr.$(aws configure get region).amazonaws.com/${ECR_REPO_NAME}:latest
docker push ${AWS_ACCOUNT_ID}.dkr.ecr.$(aws configure get region).amazonaws.com/${ECR_REPO_NAME}:latest

# Create ECS cluster if it doesn't exist
aws ecs describe-clusters --clusters ${ECS_CLUSTER_NAME} || aws ecs create-cluster --cluster-name ${ECS_CLUSTER_NAME}

# Create or update ECS task definition
aws ecs register-task-definition --family ${ECS_TASK_DEFINITION_NAME} --container-definitions "[{\"name\":\"${ECR_REPO_NAME}\",\"image\":\"${AWS_ACCOUNT_ID}.dkr.ecr.$(aws configure get region).amazonaws.com/${ECR_REPO_NAME}:latest\",\"essential\":true,\"portMappings\":[{\"containerPort\":8080,\"hostPort\":8080}]}]" --requires-compatibilities FARGATE --network-mode awsvpc --cpu 256 --memory 512

# Create or update ECS service
aws ecs describe-services --cluster ${ECS_CLUSTER_NAME} --services ${ECS_SERVICE_NAME} || aws ecs create-service --cluster ${ECS_CLUSTER_NAME} --service-name ${ECS_SERVICE_NAME} --task-definition ${ECS_TASK_DEFINITION_NAME} --desired-count 1 --launch-type FARGATE --network-configuration "awsvpcConfiguration={subnets=[$(aws ec2 describe-subnets --query 'Subnets[0].SubnetId' --output text)],assignPublicIp=ENABLED}"

echo "Deployment completed successfully!"
