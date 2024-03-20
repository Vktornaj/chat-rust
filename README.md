# Chat API

This project is a Chat API writed in Rust that allows users to send and receive messages in real-time using WebSockets. It provides functionalities for creating chat rooms, joining rooms, sending messages within the rooms, make calls and publish user stories, it uses protocol buffers to serialize data.

## Getting Started

To run the Chat API, follow these steps:

1. Make sure you have Docker installed on your machine.

2. Clone the repository:
    ```bash
    git clone https://github.com/Vktornaj/chat-rust.git
    ```

3. Navigate to the project directory:
    ```bash
    cd chat-rust
    ```

4. Build and run the Docker containers locally:
    ```bash
    docker compose -f ./compose/localhost/compose.yml up --build 
    ```

5. Start using the Chat API!

## Development
1. Open in devcontainer

2. Install the sqlx CLI tool:
    ```bash
    cargo install sqlx-cli
    ```

3. Perform database migrations:
    ```bash
    sqlx migrate info --source ./entry/migrations
    sqlx migrate run --source ./entry/migrations
    ```

4. Run the Chat API:
    ```bash
    cargo run
    ```

## Usage

Once the Chat API is up and running, you can find the docuentation at:

- Replace `$ENVIRONMENT` with the environment you are running the API on (e.g. `dev`, `prod`) and `$MODULE` with the module you want to access (e.g. `auth`, `contact`, `profile`, `message`).
  ```bash
  ./compose/$ENVIRONMENT/nginx/public/docs/$MODULE/openapi.yml
  ```

## Architecture
This project uses a Modular Monolithic architchture, that improves the scalability compared with a simple monolith architecture and maintainability compared with a microservices architecture. The project is divided into the following modules:
- Auth: Manages the authentication and authorization of users.
- Contact: Manages the relationships between users.
- Profile: Manages the user's profile information.
- Message: Manages the individual messages between users.
- MessageRoom: Manages the chat rooms and the messages within the rooms.
- Call: Manages the calls between users.
- Story: Manages the stories of the users.
- Notification: Manages the notifications of the users.
- Common: Contains the common functionalities used by the other modules.
- Eentry: Contains the entry point of the application.

### Inside the modules
Each module is divided into the following layers:
- Domain: Contains the business logic of the module.
- Application: Contains the use cases of the module and the traits.
- Adapter: Contains the implementation of the traits and the external dependencies.

## Deployment
The Chat API is deployed using Docker containers. The project contains some docker compose files that can be used to deploy the API in different environments. This project is optimized to be deployed in a cloud server (AWS).

- Push the Docker image to the registry:
  ```bash
  ./local-push-container.sh
  ```
- Retrieve the Docker image from the registry and deploy it to the cloud server:
  ```bash
  ./cloud-pull-container.sh
  ```

## Technologies
- Rust
- Axum
- SQLx
- Redis
- Postgres
- Docker
- OpenAPI
- JWT
- Websockets
- MongoDB
- Prometheus
- Grafana
- Nginx
- Protocol Buffers

## License

This project is proprietary software and is protected by copyright law. All rights are reserved. Unauthorized copying, modification, distribution, public performance, or other exploitation of this software is prohibited.
