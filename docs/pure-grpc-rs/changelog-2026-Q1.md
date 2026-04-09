# pure-grpc-rs Changelog -- 2026 Q1

## Week 2026-W15 (Apr 7 - Apr 9)

This week added comprehensive gRPC load testing infrastructure and benchmark servers.

### Highlights

**Comprehensive streaming load test** (`2e2f483`)

Added `streaming_load_test` binary supporting multiple test scenarios:
- Unary calls with various payload sizes (64B, 1KB, 64KB, 1MB)
- Server streaming (1 request -> N responses)
- Client streaming (N requests -> 1 response)
- Bidirectional streaming (N requests <-> N responses)

Tests both Tonic and pure-grpc prost clients against the same server.

**Prost benchmark server** (`2e2f483`)

Added `prost_benchmark_server` binary - a pure-grpc server implementing
`BenchmarkService` with prost codec, enabling fair comparison between tonic
and pure-grpc-rs prost performance.

### Issues

- Closed: None
- Known: prost unary latency (~41ms) vs tonic (~0.25ms) - investigation pending

### Releases

None

---

## Week 2026-W12 (Mar 16 - Mar 22)

This week saw the initial creation of the pure-grpc-rs framework followed by
three successive code-quality passes that hardened error handling, fixed a
codegen naming collision, and reduced duplication across the workspace.

### Highlights

**Initial framework commit** (`9a8cf97`, `2a75773`)

The repository was initialized and the full gRPC framework landed: all four RPC
patterns (unary, server-streaming, client-streaming, bidirectional), a pluggable
`Codec` trait with both `ProstCodec` (feature-gated) and `FlatBuffersCodec`
implementations, server and client transports built on hyper 1.x / h2 / tower,
health checking, server reflection (v1), and a `grpc-build` build.rs integration.
At approximately 8,800 lines across 10 crates, the framework is roughly one-third
the size of tonic.

**Code quality passes** (`289a84e`, `45d8225`, `17b2de3`)

Three follow-up commits addressed findings from a post-landing audit: duplicated
types were removed, validation logic tightened, unnecessary `.clone()` calls
reduced, a stream-type naming collision in codegen was fixed, and test coverage
was extended. Error handling and input validation were also strengthened in the
final pass.

### Issues

- Opened: None
- Closed: None

### Releases

None

---
