# cherubgyre
cherubgyre is an anonymous community defense social network

https://cherubgyre.com is under construction, but it's got some links.
https://api.cherubgyre.com has api docs.

## deployment instructions

### aws

1. Open AWS CloudShell.

2. In CloudShell, create a repository in AWS Elastic Container Registry (ECR).
```
aws ecr create-repository --repository-name cherubgyre-dev --region us-east-1
```

You'll receive a return similar to the following:
```
{
    "repository": {
        "repositoryArn": "arn:aws:ecr:us-east-1:[youraccount]:repository/cherubgyre-dev",
        "registryId": "[youraccount]",
        "repositoryName": "cherubgyre-dev",
        "repositoryUri": "[youraccount].dkr.ecr.us-east-1.amazonaws.com/cherubgyre-dev",
        "createdAt": "2024-11-12T00:25:01.243000+00:00",
        "imageTagMutability": "MUTABLE",
        "imageScanningConfiguration": {
            "scanOnPush": false
        },
        "encryptionConfiguration": {
            "encryptionType": "AES256"
        }
    }
}
```
In particular, note `repositoryUri:` from the returned parameters.

3. Authenticate Docker with AWS Elastic Container Registry
```
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin [youraccount].dkr.ecr.us-east-1.amazonaws.com
```

4. Clone cherubgyre repository to CloudShell
```
git clone https://github.com/davidemerson/cherubgyre
cd cherubgyre
```

5. Build Docker
```
docker build -t cherubgyre-dev .
```   

6. Tag the Docker image for AWS Elastic Container Registry:
```
docker tag cherubgyre-dev:latest [youraccount].dkr.ecr.us-east-1.amazonaws.com/cherubgyre-dev:latest
```

7. Push the image to AWS Elastic Container Registry:
```
docker push [youraccount].dkr.ecr.us-east-1.amazonaws.com/cherubgyre-dev:latest
```

8. You will now go to the AWS Elastic Container Service (ECS) console and create a cluster for your deployment. We'll use Fargate to keep things serverless.
- Open the console at https://console.aws.amazon.com/ecs/v2
- From the navigation bar, upper right, make sure your region is appropriate.
- In the navigation pane, left, choose `Clusters`.
- Under `Cluster configuration` set the cluster name. For us, this was:
```
Cluster name = cherubgyre-dev
```
- Leave `AWS Fargate (serverless)` checked.
- Click `Create`

7. Create a task definition. Modify this JSON with your account values as appropriate to `Create new task definition with JSON`:
```
{
    "containerDefinitions": [
        {
            "name": "cherubgyre-dev",
            "image": "[youraccount].dkr.ecr.us-east-1.amazonaws.com/cherubgyre-dev:latest",
            "cpu": 0,
            "portMappings": [
                {
                    "name": "rust-api-container-8080-tcp",
                    "containerPort": 8080,
                    "hostPort": 8080,
                    "protocol": "tcp",
                    "appProtocol": "http"
                }
            ],
            "essential": true,
            "environment": [],
            "environmentFiles": [],
            "mountPoints": [],
            "volumesFrom": [],
            "ulimits": [],
            "logConfiguration": {
                "logDriver": "awslogs",
                "options": {
                    "awslogs-group": "/ecs/cherubgyre-dev",
                    "mode": "non-blocking",
                    "awslogs-create-group": "true",
                    "max-buffer-size": "25m",
                    "awslogs-region": "us-east-1",
                    "awslogs-stream-prefix": "ecs"
                },
                "secretOptions": []
            },
            "systemControls": []
        }
    ],
    "family": "cherubgyre-dev",
    "executionRoleArn": "arn:aws:iam::[youraccount]:role/ecsTaskExecutionRole",
    "networkMode": "awsvpc",
    "volumes": [],
    "placementConstraints": [],
    "requiresCompatibilities": [
        "FARGATE"
    ],
    "cpu": "512",
    "memory": "1024",
    "runtimePlatform": {
        "cpuArchitecture": "X86_64",
        "operatingSystemFamily": "LINUX"
    },
    "tags": []
}
```

9. Run the Task & Test API.
Once the task definition is registered, you can run the task in your ECS cluster using the following command. Replace the placeholders accordingly

```
aws ecs run-task \
    --cluster rust-api \
    --launch-type FARGATE \
    --task-definition RustAPI \
    --network-configuration 'awsvpcConfiguration={
        subnets=["subnet-0ac341ecb24bee027", "subnet-0c0d9667c504aa776", "subnet-002bf7f9fbe43da9a", "subnet-06f2c8c9315a207f8", "subnet-0101c89782bed53be", "subnet-0f52ad3011c093e0c"],
        securityGroups=["sg-0001d3ac435b86000"],
        assignPublicIp="ENABLED"
    }'

```


10. Your api is deployed. Check the url in cluster->task-details
```
aws ecs list-tasks \
    --cluster rust-api \
    --desired-status RUNNING

```

## Single bash script
```
#!/bin/bash

# Variables
AWS_REGION="us-east-1"
REPOSITORY_NAME="cherubgyre-dev"
IMAGE_TAG="latest"
CLUSTER_NAME="cherubgyre-dev"
TASK_DEFINITION_FAMILY="cherubgyre-dev"
SUBNETS='["subnet-0ac341ecb24bee027", "subnet-0c0d9667c504aa776", "subnet-002bf7f9fbe43da9a", "subnet-06f2c8c9315a207f8", "subnet-0101c89782bed53be", "subnet-0f52ad3011c093e0c"]'
SECURITY_GROUP="sg-0001d3ac435b86000"

# Create ECR Repository
echo "Creating ECR repository..."
REPOSITORY_URI=$(aws ecr create-repository --repository-name $REPOSITORY_NAME --region $AWS_REGION \
    --query 'repository.repositoryUri' --output text)

echo "Repository URI: $REPOSITORY_URI"

# Authenticate Docker with ECR
echo "Authenticating Docker with ECR..."
aws ecr get-login-password --region $AWS_REGION | docker login --username AWS --password-stdin $REPOSITORY_URI

# Clone the repository
echo "Cloning the repository..."
git clone https://github.com/davidemerson/cherubgyre
cd cherubgyre || exit

# Build Docker Image
echo "Building Docker image..."
docker build -t $REPOSITORY_NAME .

# Tag Docker Image
echo "Tagging Docker image..."
docker tag $REPOSITORY_NAME:$IMAGE_TAG $REPOSITORY_URI:$IMAGE_TAG

# Push Docker Image to ECR
echo "Pushing Docker image to ECR..."
docker push $REPOSITORY_URI:$IMAGE_TAG

# Go back to initial directory
cd ..

# Create ECS Cluster
echo "Creating ECS cluster..."
aws ecs create-cluster --cluster-name $CLUSTER_NAME --region $AWS_REGION

# Register Task Definition
echo "Registering ECS Task Definition..."
cat <<EOF > task-definition.json
{
    "containerDefinitions": [
        {
            "name": "$REPOSITORY_NAME",
            "image": "$REPOSITORY_URI:$IMAGE_TAG",
            "cpu": 0,
            "portMappings": [
                {
                    "name": "rust-api-container-8080-tcp",
                    "containerPort": 8080,
                    "hostPort": 8080,
                    "protocol": "tcp",
                    "appProtocol": "http"
                }
            ],
            "essential": true,
            "logConfiguration": {
                "logDriver": "awslogs",
                "options": {
                    "awslogs-group": "/ecs/$CLUSTER_NAME",
                    "awslogs-region": "$AWS_REGION",
                    "awslogs-create-group": "true",
                    "awslogs-stream-prefix": "ecs"
                }
            }
        }
    ],
    "family": "$TASK_DEFINITION_FAMILY",
    "executionRoleArn": "arn:aws:iam::[youraccount]:role/ecsTaskExecutionRole",
    "networkMode": "awsvpc",
    "requiresCompatibilities": [
        "FARGATE"
    ],
    "cpu": "512",
    "memory": "1024",
    "runtimePlatform": {
        "cpuArchitecture": "X86_64",
        "operatingSystemFamily": "LINUX"
    }
}
EOF

aws ecs register-task-definition --cli-input-json file://task-definition.json

# Run Task
echo "Running ECS Task..."
TASK_ARN=$(aws ecs run-task \
    --cluster $CLUSTER_NAME \
    --launch-type FARGATE \
    --task-definition $TASK_DEFINITION_FAMILY \
    --network-configuration "awsvpcConfiguration={subnets=$SUBNETS,securityGroups=[$SECURITY_GROUP],assignPublicIp='ENABLED'}" \
    --query 'tasks[0].taskArn' --output text)

echo "Task ARN: $TASK_ARN"

# Wait for Task to Start
echo "Waiting for task to start..."
aws ecs wait tasks-running --cluster $CLUSTER_NAME --tasks $TASK_ARN

# Get Task Details
TASK_DETAILS=$(aws ecs describe-tasks --cluster $CLUSTER_NAME --tasks $TASK_ARN --query 'tasks[0].attachments[0].details')
PUBLIC_IP=$(echo $TASK_DETAILS | jq -r '.[] | select(.name=="networkInterfaceId") | .value' | xargs -I{} aws ec2 describe-network-interfaces --network-interface-ids {} --query 'NetworkInterfaces[0].Association.PublicIp' --output text)

echo "API is now running at http://$PUBLIC_IP:8080"

```


## toolchain setup for local development
1. Install rust, following instructions at https://rustup.rs
2. Clone this repo
3. Install RustRover from JetBrains (register for an account, free for non-commercial use)
4. Install lld linker for faster compile times
    ```
    brew install llvm
    ```
    or
    ```
    apt install llvm lld clang
    ```
    
5. Install cargo-watch to trigger commands when source code changes. Chain some commands so cargo watch runs check, (if successful) then test, (if successful) then run:
    ```
    cargo install cargo-watch
    cargo watch -x check -x test -x run
    ```
    
6. Install cargo-llvm-cov to measure code coverage, and run cargo llvm-cov to compute code coverage for the application.
   ```
   rustup component add llvm-tools-preview
   cargo install cargo-llvm-cov
   cargo llvm-cov
   ```
   
7. Make sure the linter "clippy" is installed. Run it to fail the pipeline if there are warnings.
   ```
   rustup component add clippy
   cargo clippy -- -D warnings
   ```
   
8. Add rustfmt for automatic code formatting. Run it using cargo fmt (or `cargo fmt -- ---check` if you'd prefer a formatting step for caution. I don't.)
   ```
   rustup component add rustfmt
   cargo fmt
   ```
   
9. Add cargo-audit for security audits. Run it to scan your dependency tree.
   ```
   cargo install cargo-audit
   cargo audit
   ```

### notes
- Check out `.github/workflows/general.yaml` in this repository: it will run some of the above fmt and clippy checks on every push to main.
- Check out `.github/workflows/audit.yaml` in this repository: it will run audits on every push to main.
- Tests will be in `tests/` here because it is preferable to externalize tests from the source for the purposes of visibility and security. We don't want to give tests any privileged access to the code.
