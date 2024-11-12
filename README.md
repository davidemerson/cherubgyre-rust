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

6. You will now go to the AWS Elastic Container Service (ECS) console and create a cluster for your deployment. We'll use Fargate to keep things serverless.
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

(to do) >> steps 8 & 9 (create task and create service) need to be converted to aws-cli / cloudshell steps so they don't rely on UI options as much (see below, it can all be one script really)

(to do) >> add a step which demonstrates a test against the /v1/health endpoint so the user can validate that their server is healthy. You can use Postman for this, or even just curl/wget

(to do) >> organize this all in a single cloudshell script for provisioning, something like this (not complete, just an example):

```
#!/bin/bash

# Variables
VPC_ID=$(aws ec2 describe-vpcs --query "Vpcs[0].VpcId" --output text)
SUBNET_IDS=$(aws ec2 describe-subnets --filters Name=vpc-id,Values=$VPC_ID --query "Subnets[*].SubnetId" --output text | tr '\t' ',')

SERVICE_NAME="service-cherubgyre"
CLUSTER_NAME="cluster-cherubgyre"
TASK_DEFINITION="cg-task" # Replace with your task definition name and revision

# Create Elastic IP
EIP_ALLOC_ID=$(aws ec2 allocate-address --query "AllocationId" --output text)

# Create Target Group for Fargate service
TARGET_GROUP_ARN=$(aws elbv2 create-target-group \
    --name fargate-target-group \
    --protocol HTTP \
    --port 8080 \
    --vpc-id $VPC_ID \
    --target-type ip \
    --query "TargetGroups[0].TargetGroupArn" \
    --output text)

# Create Load Balancer
LOAD_BALANCER_ARN=$(aws elbv2 create-load-balancer \
    --name fargate-lb \
    --subnets $SUBNET_IDS \
    --security-groups $(aws ec2 describe-security-groups --filters Name=vpc-id,Values=$VPC_ID --query "SecurityGroups[0].GroupId" --output text) \
    --query "LoadBalancers[0].LoadBalancerArn" \
    --output text)

# Create Listener for Load Balancer
aws elbv2 create-listener \
    --load-balancer-arn $LOAD_BALANCER_ARN \
    --protocol HTTP \
    --port 8080 \
    --default-actions Type=forward,TargetGroupArn=$TARGET_GROUP_ARN

# Create ECS Cluster (if not already created)
aws ecs create-cluster --cluster-name $CLUSTER_NAME

# Create Fargate Service and Associate with Load Balancer
aws ecs create-service \
    --cluster $CLUSTER_NAME \
    --service-name $SERVICE_NAME \
    --task-definition $TASK_DEFINITION \
    --load-balancers targetGroupArn=$TARGET_GROUP_ARN,containerName=your-container-name,containerPort=80 \
    --desired-count 1 \
    --network-configuration "awsvpcConfiguration={subnets=[$SUBNET_IDS],securityGroups=[$(aws ec2 describe-security-groups --filters Name=vpc-id,Values=$VPC_ID --query "SecurityGroups[0].GroupId" --output text)],assignPublicIp=ENABLED}" \
    --launch-type FARGATE

# Associate Elastic IP to the Load Balancer
NETWORK_INTERFACE_ID=$(aws elbv2 describe-load-balancers --load-balancer-arns $LOAD_BALANCER_ARN --query "LoadBalancers[0].NetworkInterfaces[0].NetworkInterfaceId" --output text)
aws ec2 associate-address --allocation-id $EIP_ALLOC_ID --network-interface-id $NETWORK_INTERFACE_ID

echo "Elastic Load Balancer with Elastic IP is now available for the Fargate service."
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
