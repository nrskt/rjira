# rjira

This is an example of Hexagonal Architecture built by Rust.
Sample application like about JIRA.

The core features are simple as follows.

- Add the item (Story | Task) to backlog.
- Estimate the item by story point.
- Assign the item to someone.

Users can use the features through REST api or command-line app.

TODO:

- [x] Think of a better method of error handling.
  - [x] and remove the codes using `unwrap`
- [x] Add logging (using tracing crate)
- [ ] More testing (ex: adaptors)

## Architecture

This application is built by multiple crates.

![image-1](./images/architecture-1.png)

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

#### Code sample

Define with default implementations.

```rust
pub trait Estimatable {
    fn mut_point(&mut self) -> &mut Option<StoryPoint>;

    /// estimate it.
    fn estimate(&mut self, point: StoryPoint) {
        *self.mut_point() = Some(point);
    }
}
```

When it implements, I can omit implementation.

```rust
impl Estimatable for Story {
    fn mut_point(&mut self) -> &mut Option<StoryPoint> {
        &mut self.point
    }
}
```

```rust
impl Estimatable for Task {
    fn mut_point(&mut self) -> &mut Option<StoryPoint> {
        &mut self.point
    }
}
```

We can test only the estimate part.

```rust
#[test]
fn test_estimatable() {
    let mut estimatable = TestEstimateable { point: None };
    estimatable.estimate(StoryPoint::new(2).unwrap());
    assert_eq!(estimatable.point, Some(StoryPoint::new(2).unwrap()))
}
```

### ports/***

Ports represent interfaces of application.
I define 2 kinds of interfaces called `Driver` and `Driven`.

#### Driver

`Driver` represents the interface that the actor of the application user.
In other words, `Driver` interface knows
how to call the function that is defined cores/backlog.

#### Driven

`Driven` represent the interface that application use.
Application often use middleware for persistencing and more.
These interfaces know that how to use it.

##### Code sample

Driver

Rust's trait can be defined default implementation.
So, unlike `Driven` interface, it is defined with default implementations.

The interface has dependencies to `Driven` interface.
So When we want to run `Driver` interface,
we can run with something that implement `Driven` interface.

Therefore, When we testing, we can use mock that was implemented `Driven`.

```rust
#[async_trait::async_trait]
pub trait BacklogUseCase: ProvideBacklogRepository {
    async fn get_backlog(&self) -> UseCaseResult<Backlog> {
        let repo = self.provide();
        let backlog = repo.get().await?;
        Ok(backlog)
    }

    /// Add item to backlog
    async fn add_item(&self, cmd: impl AddItemCmd + 'async_trait) -> UseCaseResult<Backlog> {
        let repo = self.provide();
        let mut backlog = repo.get().await?;
        backlog.add_item(cmd.item());
        repo.save(backlog.clone()).await?;
        Ok(backlog)
    }

    /// Assign the specific item to someone.
    async fn assign_item(&self, cmd: impl AssignItemCmd + 'async_trait) -> UseCaseResult<Backlog> {
        let repo = self.provide();
        let mut backlog = repo.get().await?;
        backlog.assign_item(&cmd.id(), cmd.assignee())?;
        repo.save(backlog.clone()).await?;
        Ok(backlog)
    }

    /// Estimate the specific item.
    async fn estimate_item(
        &self,
        cmd: impl EstimateItemCmd + 'async_trait,
    ) -> UseCaseResult<Backlog> {
        let repo = self.provide();
        let mut backlog = repo.get().await?;
        backlog.estimate_item(&cmd.id(), cmd.point())?;
        repo.save(backlog.clone()).await?;
        Ok(backlog)
    }
}
```

We can test it using mock

```rust
#[cfg(test)]
mod test_get_backlog {
    use super::*;

    #[tokio::test]
    async fn test_get_backlog() {
        let mut mock = mock::MockTest::new();
        mock.expect_get().times(1).returning(|| Ok(Backlog::new()));
        mock.get_backlog().await.unwrap();
    }
}
```

Driven

`Driven` interface is defined with `Provide~` trait together.
This pattern like the "Cake Pattern" in Scala.

Define `Driven` interface.

```rust
pub trait ProvideBacklogRepository {
    type Repository: BacklogRepository + Send + Sync;

    fn provide(&self) -> &Self::Repository;
}

#[async_trait::async_trait]
pub trait BacklogRepository {
    /// Get the specific backlog.
    ///
    /// If backlog does not find, return the error.
    async fn get(&self) -> PortsResult<Backlog>;

    /// Save the specific backlog.
    async fn save(&self, backlog: Backlog) -> PortsResult<()>;
}
```

### adaptors/***

Adaptors implement the port interfaces.
For example, If we want to save the backlog to the file system,
we should implement a repository interface for someone that knows
how to use the file system.

`Driven` adaptor implementation.

I defined the struct that knows the file path.
The struct knows how to save the backlog through the `Driven` interface (`BacklogRepository`).

Of cause, If we want to persist backlog to in-memory,
we can use a data structure such as a `HashMap`.

```rust
#[derive(Debug, Clone)]
pub struct FsBacklogRepository {
    path: PathBuf,
}

#[async_trait::async_trait]
impl BacklogRepository for FsBacklogRepository {
    async fn get(&self) -> PortsResult<Backlog> {
        let file = File::open(&self.path)?;
        let backlog = serde_yaml::from_reader(file);
        match backlog {
            Err(_) => Ok(Backlog::new()),
            Ok(backlog) => Ok(backlog),
        }
    }

    async fn save(&self, backlog: Backlog) -> PortsResult<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .unwrap();
        // let file = File::create(&self.path)?;
        serde_yaml::to_writer(file, &backlog)?;
        Ok(())
    }
}
```

On the other hand, Implement `Driver` adaptors.

`RestAdaptor` knows how to save the backlog through
`ProvideBacklogRepository` and `BacklogRepository` interfaces.

```rust
#[derive(Debug, Clone)]
pub struct RestAdaptor {
    fs: FsBacklogRepository,
}

impl ProvideBacklogRepository for RestAdaptor {
    type Repository = FsBacklogRepository;

    fn provide(&self) -> &Self::Repository {
        &self.fs
    }
}
```

And the adaptor implements how to use the `Driver` interface.

```rust
impl BacklogUseCase for RestAdaptor {}
```

### applications/***

The `applications` crates are entrypoint.
I defined 2 crates (REST server and command-line). These include `main` function.

`main` function initialize `Adaptor` and start application.

```rust
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let adaptor = CliAdaptoer::new(args.data());
    args.run(adaptor).await
}
```

### Error handling

`cores/***`, `ports/driven/***`, and `adaptors` for driven define
their own error with `thiserror` crate.
They return the error was defined.

`cores/backlog` define the error.

```rust
#[derive(Debug, Error)]
pub enum BacklogError {
    #[error("TypeError: {0:?}")]
    TypeError(String),
    #[error("NotFound: {0:?}")]
    NotFound(String),
}
```

and it returns with its own logic.

```rust
/// The collection can search a specific item and estimate it.
pub trait EstimatableFromCollection:
    FindFromCollection<Key = Uuid, Ret = Box<dyn BacklogItem>>
{
    /// estimate the specific item.
    fn estimate_item(&mut self, id: &Uuid, point: StoryPoint) -> BacklogResult<()> {
        match self.find_by_id_mut(id) {
            None => Err(BacklogError::not_found(format!(
                "BacklogItem, id: {} does not found",
                id
            ))),
            Some(item) => {
                item.estimate(point);
                Ok(())
            }
        }
    }
}
```

`ports/driven/backlog-repo` define error, `adaptors/fs` use it.

```rust
#[derive(Debug, Error)]
pub enum BacklogRepositoryError {
    #[error("BacklogRepositoryError: not found the resource, {0}")]
    NotFound(String),
    #[error("BacklogRepositoryError: IO occurred something, {0}")]
    Io(#[from] std::io::Error),
    #[error("BacklogRepositoryError: serialize/deserialize yaml occurred something, {0}")]
    Yaml(#[from] serde_yaml::Error),
}
```

```rust
#[async_trait::async_trait]
impl BacklogRepository for FsBacklogRepository {
    async fn get(&self) -> BacklogRepositoryResult<Backlog> {
        OpenOptions::new()
            .create(true)
            // If I use .write(false), I get the error that mean "InvalidInput".
            .write(true)
            .truncate(false)
            .open(&self.path)?;
        let file = std::fs::File::open(&self.path)?;
        let backlog = serde_yaml::from_reader(file);
        match backlog {
            Err(_) => Ok(Backlog::new()),
            Ok(backlog) => Ok(backlog),
        }
    }

    async fn save(&self, backlog: Backlog) -> BacklogRepositoryResult<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;
        serde_yaml::to_writer(file, &backlog)?;
        Ok(())
    }
}
```

On the other hand, `ports/driver/backlog-service` defines 3 kinds of errors.

- IncommingError
- OutcommingError
- BusinessLogicError

IncommingError represents input error.
When calling the service, we often validate the input value. If something happens,
we raise the IncommingError.  

OutcommingError represents driven error.
When the service uses some driven interfaces, if something happens,
we raise the  OutcommingError.

BusinessLogicError represents that something happens in `cores/backlog`.

So, When implementing the `backlog-service`, we treat 3 types of errors.

I decided to use the `eyre` crate how to treat the errors in `backlog-service`.
The usage is as follows.

But, I have problems that was difficult to use the 3 types of errors properly.

**Problem 1**

Can't use backtrace in stable.

If we get the error, we want to find where is wrong.
My first codes.

```rust
enum UseCaseError {
    ...etc
}

fn something() -> Result<(), UseCaseError> {
    do()?;
}
```

This code works correctly, but it does not tell us where is wrong.
So, I decide to use `eyre` or `anyhow` for error reporting.

**Problem 2**

I want to handle Incomming/Outcomming/BusinessLogic.

I decided to use `eyre` for error handling. I write as follows.

```rust
repo.save(backlog.clone()).await
    .wrap_err("fail to save the backlog")?;
```

`wrap_err` provide `eyre::WrapErr` trait.
This code has backtrace feature.
So we can get the file name and line number that was happened.

But `repo.save()` return `BacklogRepositoryError`.
I want to cast to `OutcommingError` because the error handler
in `rest` or `cli` becomes complicated.

Why it's complicated. If it does not cast the error, the handler needs to know
many error types. This case is simple but in the future, we need many driven interfaces.
At that time, the handler must know all error types. I want to avoid it.

**Solution**

I think that I should cast the error before calling the `.wrap_err()` method.

```rust
repo.save(backlog.clone()).await
    .map_err(OutcommingError::from)
    .wrap_err("fail to save the backlog")?;
```

And for shortening this boilerplate I implemented `WrapErrExt` trait.

```rust
// without context message
repo.save(backlog.clone()).await.wrap::<OutcommingError>()?;
// with context message
repo.save(backlog.clone()).await
    .wrap_msg::<OutcommingError>("fail to save the backlog")?;
```

### For example

Overall

```rust
async fn estimate_item(
    &self,
    cmd: impl EstimateItemCmd + 'async_trait,
) -> eyre::Result<Backlog> {
    // IncommingError handling
    let id = cmd.id().wrap_err("fail to get item id")?;
    let point = cmd.point().wrap_err("fail to get story point")?;

    let repo = self.provide();

    // OutcommingError handling
    let mut backlog = repo
        .get()
        .await
        .wrap_msg::<OutcommingError>("fail to get backlog")?;
    
    // BusinessLogicError handling
    backlog
        .estimate_item(&id, point)
        .wrap::<BusinessLogicError>()?;

    // OutcommingError handling
    repo.save(backlog.clone()).await.wrap::<OutcommingError>()?;
    Ok(backlog)
}
```

Error handler in `rest`

```rust
let (status, msg) = if let Some(_) = err.downcast_ref::<IncommingError>() {
    (StatusCode::BAD_REQUEST, format!("{:?}", err))
} else if let Some(_) = err.downcast_ref::<OutcommingError>() {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err))
} else if let Some(_) = err.downcast_ref::<BusinessLogicError>() {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err))
} else {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("unexpected error"),
    )
};
```

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
