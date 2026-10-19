<p align="center">
  <img src="assets/logo.png" alt="Cloudws E2e RPN Rust logo" width="180">
</p>

# RPN Scientific Calculator

A REST API service that evaluates scientific calculator expressions written in
[Reverse Polish Notation (RPN)](https://en.wikipedia.org/wiki/Reverse_Polish_notation).
Built with **Rust** and **Actix Web**.

## What it does

The service exposes a single endpoint, `POST /calculate`, which accepts a JSON
body containing a space-separated RPN expression and returns the computed
result (or a descriptive error).

### Supported operators

| Operator | Meaning                     | Arity  |
|----------|------------------------------|--------|
| `+`      | Addition                     | binary |
| `-`      | Subtraction                  | binary |
| `*`      | Multiplication                | binary |
| `/`      | Division                     | binary |
| `^`      | Power                         | binary |
| `sqrt`   | Square root                   | unary  |
| `sin`    | Sine (radians)                | unary  |
| `cos`    | Cosine (radians)              | unary  |
| `tan`    | Tangent (radians)             | unary  |
| `log`    | Natural logarithm (base e)   | unary  |

Numeric tokens (integers or floats, including negative numbers) are pushed
onto the stack. Operators pop their operands off the stack, compute a result,
and push it back — standard RPN/postfix evaluation.

### Error handling

The service returns HTTP `400 Bad Request` with an `{"error": "..."}` body for:

- **Stack underflow** — not enough operands for an operator
- **Division by zero**
- **Unknown operators** or **non-numeric tokens**
- **Invalid expressions** — e.g. leftover values on the stack, empty input
- **Domain errors** — `sqrt` of a negative number, `log` of a non-positive number

## Build

Requires a stable Rust toolchain (edition 2021) and Cargo.

```bash
cargo build --release
```

## Run

```bash
cargo run
```

By default the service listens on `http://0.0.0.0:8080`.

## Example requests

Successful calculation — `(3 + 4) * 2 - 1`:

```bash
curl -X POST http://localhost:8080/calculate \
  -H 'Content-Type: application/json' \
  -d '{"expression": "3 4 + 2 * 1 -"}'
# => {"result":13.0}
```

Square root:

```bash
curl -X POST http://localhost:8080/calculate \
  -H 'Content-Type: application/json' \
  -d '{"expression": "9 sqrt"}'
# => {"result":3.0}
```

Trigonometry (radians) and natural log:

```bash
curl -X POST http://localhost:8080/calculate \
  -H 'Content-Type: application/json' \
  -d '{"expression": "0 cos 1 +"}'
# => {"result":2.0}
```

Error — division by zero:

```bash
curl -X POST http://localhost:8080/calculate \
  -H 'Content-Type: application/json' \
  -d '{"expression": "5 0 /"}'
# => HTTP 400
# => {"error":"division by zero"}
```

Error — stack underflow:

```bash
curl -X POST http://localhost:8080/calculate \
  -H 'Content-Type: application/json' \
  -d '{"expression": "1 +"}'
# => HTTP 400
# => {"error":"stack underflow: not enough operands for operator '+'"}
```

## Testing

Unit tests cover the RPN evaluator (`src/rpn.rs`) — happy path, operator
precedence expressed via RPN ordering, every supported operator, and error
cases (stack underflow, division by zero, unknown operators, non-numeric
tokens, domain errors) — plus integration tests for the HTTP layer
(`src/main.rs`) verifying the `200`/`400` response contract.

```bash
cargo test
```
