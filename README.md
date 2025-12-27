# Redis in Docker

Redis (short for **REmote DIctionary Server**) is a high-performance, in-memory data store designed for extremely fast data access.  
It primarily stores data in RAM for speed, while optionally persisting data to disk for durability.

This project shows how to run **Redis using Docker and Docker Compose**.

---

## 🚀 What is Redis used for?

Redis is commonly used as:

### 🧠 Cache

Store frequently accessed data so applications don’t repeatedly query slower databases.

## Examples

- User sessions
- API responses

---

### 📦 Data Store

Redis supports rich data structures beyond simple key-value pairs:

- Strings
- Lists
- Sets
- Sorted Sets
- Hashes
- Streams
- Bitmaps

---

### ⚡ Message Broker

- Pub/Sub messaging
- Task queues
- Event streaming

---

### ⏱️ Real-Time Features

- Rate limiting
- Leaderboards
- Counters
- Live analytics

---

## ⚙️ Why is Redis fast?

- Data is stored in **RAM**
- Single-threaded core (no locking overhead)
- Highly optimized **C** implementation
- Typical operations complete in **microseconds**

---

## 📦 Requirements

- Docker
- Docker Compose

---

## 🔧 Installation

(Optional) Create a local data directory:

```bash
mkdir data
```

Run Redis with Docker

```bash

docker run -d \
  --name redis-alpine \
  -p 6379:6379 \
  -p 8001:8001 \
  redis:7.4-alpine
```

🐳 Run Redis with Docker Compose
Start Redis

```bash

docker compose up -d
```

Stop Redis

```bash

docker compose down
```

Remove all data and Docker resources

⚠️ This will delete all Redis data

```bash

docker compose down -v
docker system prune -f
```

🔌 Connect to Redis

```bash

docker exec -it redis-alpine redis-cli
```

🧪 Run Commands in Redis CLI

```bash

SET company redis
GET company
```

🔐 Redis Authentication Examples
Authenticate after connecting

```bash

redis-cli -h redis15.localnet.org -p 6390
AUTH myusername mypassword
```

Authenticate using a URI

```bash

redis-cli -u redis://myusername:mypassword@localhost:6379
```
