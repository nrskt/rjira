# rjira

This is an example of Hexagonal Architecture built by Rust.
Sample application like about JIRA.

The core features are simple as follows.

- Add the item (Story | Task) to backlog.
- Estimate the item by story point.
- Assign the item to someone.

Users can use the features through REST api or command-line app.

TODO:

- [ ] Think of a better method of error handling.
- [ ] Add logging (using tracing crate)
- [ ] More testing (ex: adaptors)

## Architecture

This application is built by multiple crates.

### cores/backlog

Cores represent the business domain.
In this case, the core has knowledge of backlog and backlog items.

- What is backlog item?
- What behavior does backlog item have?
- What is backlog?
- What behavior does backlog have?

I try to define data and behavior as separated using trait.

Users treat 2 kind of items (Story and Task).
These items are strictly different but have similar behavior.
These are estimated, assigned, ...etc.

Therefore, I define some traits that represent behavior.

For example, `Estimatable` trait means that something can be estimated.
I gave this trait to `Story` and `Task`.

By separating the behavior and the data gave me to be simple one by one.

(I often used trait object in this application.
However, it is possible to make similar expressing using enum.
trait object sometimes confused me more than necessary.)

### ports/***

### adaptors/***

### applications/***

## How to use

### rest-server

Start server

```sh
❯ cargo run --bin rest-server
```

Add item

```sh
curl --location --request POST 'localhost:3000/backlog/items' \
--header 'Content-Type: application/json' \
--data-raw '{
    "item_type": "Task",
    "title": "test"
}'
```

Estimate item

```sh
curl --location --request PUT 'localhost:3000/backlog/items/<item_id>' \
--header 'Content-Type: application/json' \
--data-raw '{
    "point": 1,
}'
```

Assign item

```sh
curl --location --request PUT 'localhost:3000/backlog/items/<item_id>' \
--header 'Content-Type: application/json' \
--data-raw '{
    "assignee": "someone",
}'
```

### command-line

Show help

```sh
❯ cargo run --bin rjira -- --help
```

Add item

```sh
❯ cargo run --bin rjira -- add-item Story test
```

Estimate item

```sh
❯ cargo run --bin rjira -- estimate-item <ID> <POINT>
```

Assign item

```sh
❯ cargo run --bin rjira -- assign-item <ID> <ASSIGNEE>
```
