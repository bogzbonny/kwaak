use crate::repository::Repository;
use crate::storage;
use anyhow::Result;
use swiftide::indexing::loaders;
use swiftide::indexing::transformers;
use swiftide::integrations;
use swiftide::traits::EmbeddingModel;
use swiftide::traits::SimplePrompt;

pub async fn index_repository(repository: &Repository) -> Result<()> {
    let extensions = repository.config().language.file_extensions();
    let loader = loaders::FileLoader::new(repository.path()).with_extensions(extensions);
    // NOTE: Parameter to optimize on
    let chunk_size = 100..2048;

    let indexing_provider: Box<dyn SimplePrompt> =
        repository.config().indexing_provider().try_into()?;
    let embedding_provider: Box<dyn EmbeddingModel> =
        repository.config().embedding_provider().try_into()?;
    let lancedb = storage::build_lancedb(repository)?;

    swiftide::indexing::Pipeline::from_loader(loader)
        .then_chunk(transformers::ChunkCode::try_for_language_and_chunk_size(
            repository.config().language,
            chunk_size,
        )?)
        .then(transformers::MetadataQACode::new(indexing_provider))
        .then_in_batch(transformers::Embed::new(embedding_provider))
        .then_store_with(lancedb)
        .run()
        .await
}
