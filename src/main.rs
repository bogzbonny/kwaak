use std::io;

use anyhow::Result;
use config::Config;
use frontend::App;
use ratatui::{backend::CrosstermBackend, Terminal};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

mod chat_message;
mod commands;
mod config;
mod frontend;
mod indexing;
mod query;
mod repository;
mod storage;
mod tracing;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load().await?;
    let repository = repository::Repository::from_config(config);

    std::fs::create_dir_all(repository.config().cache_dir())?;
    std::fs::create_dir_all(repository.config().log_dir())?;

    crate::tracing::init(&repository)?;

    ::tracing::info!("Loaded configuration: {:?}", repository.config());

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Start the application
    let mut app = App::default();

    if cfg!(feature = "test-markdown") {
        app.ui_tx
            .send(chat_message::ChatMessage::new_system(MARKDOWN_TEST).into())?;
    }
    let handler = commands::CommandHandler::start_with_ui_app(&mut app, repository);

    let res = app.run(&mut terminal).await;
    handler.abort();

    // TODO: Add panic unwind hook to alwqays restore terminal
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

static MARKDOWN_TEST: &str = r#"
## Examples

Indexing a local code project, chunking into smaller pieces, enriching the nodes with metadata, and persisting into [Qdrant](https://qdrant.tech):

```rust
indexing::Pipeline::from_loader(FileLoader::new(".").with_extensions(&["rs"]))
        .with_default_llm_client(openai_client.clone())
        .filter_cached(Redis::try_from_url(
            redis_url,
            "swiftide-examples",
        )?)
        .then_chunk(ChunkCode::try_for_language_and_chunk_size(
            "rust",
            10..2048,
        )?)
        .then(MetadataQACode::default())
        .then(move |node| my_own_thing(node))
        .then_in_batch(Embed::new(openai_client.clone()))
        .then_store_with(
            Qdrant::builder()
                .batch_size(50)
                .vector_size(1536)
                .build()?,
        )
        .run()
        .await?;
```

Querying for an example on how to use the query pipeline:

```rust
query::Pipeline::default()
    .then_transform_query(GenerateSubquestions::from_client(
        openai_client.clone(),
    ))
    .then_transform_query(Embed::from_client(
        openai_client.clone(),
    ))
    .then_retrieve(qdrant.clone())
    .then_answer(Simple::from_client(openai_client.clone()))
    .query("How can I use the query pipeline in Swiftide?")
    .await?;
"#;
