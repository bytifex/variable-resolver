use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CollectBlocksFromTemplateError {
    #[error("opened block is note closed, block start offset = {block_start_offset}")]
    OpenedBlockIsNotClosed { block_start_offset: usize },

    #[error("there is no block opened, block end offset = {block_end_offset}")]
    ThereIsNoOpenedBlock { block_end_offset: usize },
}

#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Error)]
pub enum DecodeStringError {
    #[error("could not resolve variable, variable name = {variable_name}")]
    CouldNotResolveVariable { variable_name: String },

    #[error("could not collect blocks from template, error = {0}")]
    CollectBlocksFromTemplateError(
        #[from]
        #[source]
        CollectBlocksFromTemplateError,
    ),
}
