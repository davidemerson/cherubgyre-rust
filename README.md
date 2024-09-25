# cherubgyre
cherubgyre is an anonymous community defense social network

https://cherubgyre.com is under construction, but it's got some links.

https://api.cherubgyre.com has api docs.

# structure
```
/cherubgyre
│
├── /src
│   ├── main.rs
│   └── handlers.rs  # Lambda function handlers
├── /target          # Build artifacts
├── /template.yaml   # AWS SAM Template for API Gateway, Lambda, and DynamoDB
└── /Cargo.toml      # Rust dependencies
```

# function overview
Each event in the Lambda function corresponds to a specific API route in the API Gateway (e.g., /users/invite, /users/{user_id}/duress).

- RustApiFunction: Single Lambda function that handles all API requests, mapped to individual API Gateway paths.

- get_followers:
    Retrieves the list of followers for a given user from the DynamoDB followers table.

- duress_alert:
    Handles duress alerts, stores them in the DynamoDB duress_status table, and triggers notifications (you'll need to implement the notification logic).

- cancel_duress:
    Cancels the active duress status for a user by verifying the normal pin code and deleting the duress status from the DynamoDB duress_status table.

- enable_test_mode:
    Enables test mode for the user and stores the activation time in the test_mode table. This test mode will remain active for 5 minutes.

- get_map_data:
    Retrieves the map data (last check-in location and duress status) for users that the requesting user follows from the followed_users table.

# database
- users table: Stores user info, including the normal pin and duress pin.
- followers table: Maps users to their followers.
- duress_status table: Stores the active duress alerts for users.
- test_mode table: Tracks when a user has enabled test mode.
- followed_users table: Tracks the last check-in and duress status of followed users.

# deployment method
## build the lambda in rust
```
cargo build --release --target x86_64-unknown-linux-gnu
zip -j target/lambda/bootstrap target/x86_64-unknown-linux-gnu/release/cherubgyre

```
## deploy using AWS SAM
```
sam build
sam deploy --guided

```
## testing the API
### invite code generation
```
curl -X POST https://<api-gateway-url>/users/invite
```
### register user
```
curl -X POST https://<api-gateway-url>/users/register \
  -H "Content-Type: application/json" \
  -d '{ "invite_code": "ABC123DEF", "normal_pin": "1234", "duress_pin": "4321" }'
```
### trigger duress alert
```
curl -X POST https://<api-gateway-url>/users/{user_id}/duress \
  -H "Content-Type: application/json" \
  -d '{ "duress_type": "pin_code", "message": "Help!", "timestamp": "2024-09-25T12:00:00Z" }'
```
### get followers
```
curl -X GET https://<api-gateway-url>/users/{user_id}/followers
```
### cancel duress
```
curl -X POST https://<api-gateway-url>/users/{user_id}/duress/cancel \
  -H "Content-Type: application/json" \
  -d '{ "normal_pin": "1234", "confirm": true }'
```