# Philosophy Microservices Plan — Read-Only Demo

## Goal

Build a small read-only microservice demo about philosophy. The system should be simple enough to implement quickly, preferably within a few days, while still demonstrating correct bounded contexts and database-per-service thinking.

The system only needs `GET` endpoints. No create, update, delete, authentication, events, queues, or distributed transactions are required.

---

## Recommended Services

## 1. Catalog Service

The Catalog Service owns the core philosophy catalog data.

### Responsibility

It answers questions such as:

- Who are the philosophers?
- What philosophical schools exist?
- Which philosophers belong to which schools?

### Example Data

```text
philosophers
schools
philosopher_school
```

### Endpoints

```http
GET /health
GET /philosophers
GET /philosophers/{id}
GET /schools
GET /schools/{id}
```

### Example Philosopher

```json
{
  "id": "marcus-aurelius",
  "name": "Marcus Aurelius",
  "birthYear": 121,
  "deathYear": 180,
  "schoolIds": ["stoicism"]
}
```

### Example School

```json
{
  "id": "stoicism",
  "name": "Stoicism",
  "description": "A Hellenistic school of philosophy focused on virtue, reason, and living according to nature."
}
```

---

## 2. Quote Service

The Quote Service owns philosophical quotes.

### Responsibility

It answers questions such as:

- What quotes are available?
- Which quotes belong to a given philosopher?
- Can the app show a random quote?

The service stores only the external philosopher ID. It does not own philosopher details.

### Example Data

```text
quotes
```

### Endpoints

```http
GET /health
GET /quotes
GET /quotes/{id}
GET /quotes?philosopherId={id}
GET /quotes/random
```

### Example Quote

```json
{
  "id": "quote-1",
  "philosopherId": "marcus-aurelius",
  "text": "You have power over your mind, not outside events."
}
```

---

## 3. Reading Guide Service

The Reading Guide Service owns prebuilt learning or reading recommendations.

### Responsibility

It answers questions such as:

- What reading guides are available?
- Which guide belongs to a philosophical school?
- Which philosophers should be studied in what order?

The service stores only external IDs such as `schoolId` and `philosopherIds`.

### Example Data

```text
reading_guides
```

### Endpoints

```http
GET /health
GET /guides
GET /guides/{id}
GET /guides?schoolId={id}
```

### Example Reading Guide

```json
{
  "id": "intro-to-stoicism",
  "title": "Introduction to Stoicism",
  "schoolId": "stoicism",
  "philosopherIds": [
    "epictetus",
    "seneca",
    "marcus-aurelius"
  ]
}
```

---

## Service Boundaries

This design uses bounded contexts instead of splitting the system by database tables.

| Service | Business Question |
|---|---|
| Catalog Service | What do we know about philosophers and schools? |
| Quote Service | What quotes belong to philosophers? |
| Reading Guide Service | What should a user read or study? |

The services may reference each other by ID, but they do not share databases and do not use cross-database foreign keys.

---

## Data Ownership

### Catalog Service owns:

```text
philosophers
schools
philosopher_school
```

### Quote Service owns:

```text
quotes
```

### Reading Guide Service owns:

```text
reading_guides
```

The Quote Service and Reading Guide Service can store IDs such as:

```text
philosopherId = "marcus-aurelius"
schoolId = "stoicism"
```

These are external references, not database-level foreign keys.

---

## Communication

For a very small demo, the services do not need to call each other.

The frontend can compose the data:

```text
Catalog Service       -> philosopher and school names
Quote Service         -> quotes
Reading Guide Service -> recommended reading order
```

Example frontend flow:

```text
1. GET /guides/intro-to-stoicism
2. GET /schools/stoicism
3. GET /philosophers/epictetus
4. GET /philosophers/seneca
5. GET /philosophers/marcus-aurelius
6. GET /quotes?philosopherId=marcus-aurelius
```

This keeps the backend very simple.

---

## Implementation Scope

Each service only needs:

- static seed data or one simple database table;
- a repository layer;
- a service layer;
- a controller layer;
- a `/health` endpoint;
- a few read-only `GET` endpoints.

For the fastest version, each service can use a local JSON file instead of a real database.

Example structure:

```text
catalog-service/data.json
quote-service/data.json
reading-guide-service/data.json
```

If the demo needs to show database-per-service, use three small PostgreSQL databases instead.

---

## What to Skip

To keep the project small, skip:

- authentication;
- user accounts;
- POST, PATCH, PUT, DELETE endpoints;
- Kafka or RabbitMQ;
- event sourcing;
- CQRS;
- distributed transactions;
- Kubernetes;
- a separate API Gateway;
- complex validation;
- admin screens.

---

## Suggested 7-Day Plan

## Day 1 — Project Setup

- Create the repository.
- Add three service folders.
- Add Docker Compose.
- Add health endpoints.
- Add static seed data or basic PostgreSQL setup.

## Day 2 — Catalog Service

- Implement philosopher endpoints.
- Implement school endpoints.
- Add sample data for Stoicism, Platonism, and Aristotelianism.

## Day 3 — Quote Service

- Implement quote list endpoint.
- Implement quote-by-ID endpoint.
- Implement quote-by-philosopher endpoint.
- Implement random quote endpoint.

## Day 4 — Reading Guide Service

- Implement guide list endpoint.
- Implement guide-by-ID endpoint.
- Implement guide-by-school endpoint.
- Add 2–3 predefined reading guides.

## Day 5 — Frontend

- Create a very small Angular frontend.
- Show philosophers, schools, quotes, and reading guides.
- Compose data in the frontend using IDs.

## Day 6 — Integration

- Run everything with Docker Compose.
- Add README instructions.
- Add example curl commands.
- Fix small integration issues.

## Day 7 — Deployment and Demo

- Deploy only if needed.
- Record a short demo video.
- Show that each service has its own responsibility and data ownership.

---

## Final Recommended Scope

The smallest useful version is:

```text
Catalog Service
Quote Service
Reading Guide Service
```

With only read-only endpoints:

```text
GET /philosophers
GET /schools
GET /quotes
GET /guides
```

This is small enough to implement quickly, but still demonstrates:

- bounded contexts;
- database-per-service thinking;
- external IDs instead of cross-database foreign keys;
- frontend-side data composition;
- a realistic microservice structure without unnecessary complexity.
