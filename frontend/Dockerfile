# Use the official Rust image as a base
FROM rust:latest AS builder

# Install wasm-pack for building WebAssembly
RUN cargo install wasm-pack

# Set the working directory inside the container
WORKDIR /usr/src/app/frontend

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN wasm-pack build --target web

# Use a minimal base image for the final stage
FROM nginx:alpine

# Set the working directory inside the container
WORKDIR /usr/share/nginx/html

# Copy the built files from the builder stage
COPY --from=builder /usr/src/app/pkg .

# Expose the port your application runs on
EXPOSE 80

# Command to run the application
CMD ["nginx", "-g", "daemon off;"]