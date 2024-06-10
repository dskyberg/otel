# Rust + OTEL + Grafana
Sandbox for learning Open Telemetry using Grafana (and Grafana Tempo)

## Running

1. Run the docker containers:

```bash
docker-compose up -d
```

2. Run the Rust based web service

```bash
cd micro_rs
cargo run 
```

3. Ping the web service

```bash
curl localhost:8080/
```

4. Explore [Grafana](http://localhost:3000/)
