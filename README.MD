# Document

## Config Environment

```bash
cp .env.example .env
```

## CreateDB

```bash
docker compose up -d
```

## How To Migrate

```bash
diesel migration generate "table"
```

```bash
diesel migration run
```

## How to run

```bash
cargo watch -x run
```
