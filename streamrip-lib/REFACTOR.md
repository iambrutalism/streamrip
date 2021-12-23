# Streamrip Refactor

## Goals

- Improve performance. Right now, when using Python's concurrency facilities,
CPU usage is far too high. Startup time is also too high. Such a simple program
should be very lightweight.

- Improve maintainability: Although the python script is modularized, the dependencies
dependency graph is very complicated.

- Properly use interfaces: Currently there are too many `if client.source == x`. The
clients should convert the responses into a universal form right when they recieve
the data. There should not be patches littered thoughout the program in the form
of `if` statements.

- Better support for asynchronous code

## Design

### Enums

```rust
enum MediaType {
    Album,
    Artist,
    Label,
    Playlist,
    Track,
    Video,
}
```

**Client**

The client trait defines:
- `authenticate(auth_info: AuthenticationInfo)`: do any necessary authentication
- `search(query: String)`: search for a query
- `get_metadata(id: String, type: Media)`
