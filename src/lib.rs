#![forbid(unsafe_code)]

use error::{CollectBlocksFromTemplateError, DecodeStringError};

pub mod error;

pub fn decode_string<'a>(
    text: impl AsRef<str>,
    resolve_variable: impl Fn(&str) -> Option<&'a str>,
) -> Result<String, DecodeStringError> {
    let text = text.as_ref();

    let blocks = collect_blocks_from_string(text)?;

    let mut ret = String::new();

    for block in blocks {
        if block.is_between_double_curly {
            ret.push_str(resolve_variable(block.value).ok_or_else(|| {
                DecodeStringError::CouldNotResolveVariable {
                    variable_name: block.value.to_string(),
                }
            })?);
        } else {
            ret.push_str(&block.value.replace("{{{", "{{").replace("}}}", "}}"));
        }
    }

    Ok(ret)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Block<'a> {
    index: usize,
    value: &'a str,
    is_between_double_curly: bool,
}

pub fn collect_blocks_from_string(
    value: &str,
) -> Result<Vec<Block<'_>>, CollectBlocksFromTemplateError> {
    let mut ret = Vec::new();

    let mut is_between_double_curly = false;
    let mut current_block_start_offset = 0;

    let mut index = 0;
    while index < value.len() {
        if is_between_double_curly {
            if value[index..].starts_with("}}") {
                ret.push(Block {
                    index: current_block_start_offset,
                    value: &value[current_block_start_offset..index],
                    is_between_double_curly: true,
                });

                is_between_double_curly = false;
                current_block_start_offset = index + 2;

                index += 2;
            } else {
                index += 1;
            }
        } else if value[index..].starts_with("{{{") {
            index += 3;
        } else if value[index..].starts_with("{{") {
            if current_block_start_offset != index {
                ret.push(Block {
                    index: current_block_start_offset,
                    value: &value[current_block_start_offset..index],
                    is_between_double_curly: false,
                });
            }

            is_between_double_curly = true;
            current_block_start_offset = index + 2;

            index += 2;
        } else if value[index..].starts_with("}}}") {
            index += 3;
        } else if value[index..].starts_with("}}") {
            return Err(CollectBlocksFromTemplateError::ThereIsNoOpenedBlock {
                block_end_offset: index,
            });
        } else {
            index += 1;
        }
    }

    if is_between_double_curly {
        Err(CollectBlocksFromTemplateError::OpenedBlockIsNotClosed {
            block_start_offset: current_block_start_offset,
        })
    } else {
        if current_block_start_offset != index {
            ret.push(Block {
                index: current_block_start_offset,
                value: &value[current_block_start_offset..],
                is_between_double_curly: false,
            });
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        decode_string,
        error::{CollectBlocksFromTemplateError, DecodeStringError},
        Block,
    };

    #[test]
    fn collect_blocks_from_string() {
        let blocks = super::collect_blocks_from_string("foo{{HOME}}bar").unwrap();
        assert_eq!(blocks.len(), 3);
        assert_eq!(
            blocks[0],
            Block {
                index: 0,
                value: "foo",
                is_between_double_curly: false,
            },
        );
        assert_eq!(
            blocks[1],
            Block {
                index: 5,
                value: "HOME",
                is_between_double_curly: true,
            },
        );
        assert_eq!(
            blocks[2],
            Block {
                index: 11,
                value: "bar",
                is_between_double_curly: false,
            }
        );

        let blocks = super::collect_blocks_from_string("{{HOME}}foobar").unwrap();
        assert_eq!(blocks.len(), 2);
        assert_eq!(
            blocks[0],
            Block {
                index: 2,
                value: "HOME",
                is_between_double_curly: true,
            }
        );
        assert_eq!(
            blocks[1],
            Block {
                index: 8,
                value: "foobar",
                is_between_double_curly: false,
            }
        );

        let blocks = super::collect_blocks_from_string("foobar{{HOME}}").unwrap();
        assert_eq!(blocks.len(), 2);
        assert_eq!(
            blocks[0],
            Block {
                index: 0,
                value: "foobar",
                is_between_double_curly: false,
            }
        );
        assert_eq!(
            blocks[1],
            Block {
                index: 8,
                value: "HOME",
                is_between_double_curly: true,
            }
        );

        let blocks =
            super::collect_blocks_from_string("{{A}}f{{B}}o{{C}}o{{D}}b{{E}}a{{F}}r{{G}}").unwrap();
        assert_eq!(blocks.len(), 13);
        assert_eq!(
            blocks[0],
            Block {
                index: 2,
                value: "A",
                is_between_double_curly: true,
            },
        );
        assert_eq!(
            blocks[1],
            Block {
                index: 5,
                value: "f",
                is_between_double_curly: false,
            },
        );
        assert_eq!(
            blocks[2],
            Block {
                index: 8,
                value: "B",
                is_between_double_curly: true,
            },
        );
        assert_eq!(
            blocks[3],
            Block {
                index: 11,
                value: "o",
                is_between_double_curly: false,
            },
        );
        assert_eq!(
            blocks[4],
            Block {
                index: 14,
                value: "C",
                is_between_double_curly: true,
            },
        );
        assert_eq!(
            blocks[5],
            Block {
                index: 17,
                value: "o",
                is_between_double_curly: false,
            },
        );
        assert_eq!(
            blocks[6],
            Block {
                index: 20,
                value: "D",
                is_between_double_curly: true,
            },
        );
        assert_eq!(
            blocks[7],
            Block {
                index: 23,
                value: "b",
                is_between_double_curly: false,
            },
        );
        assert_eq!(
            blocks[8],
            Block {
                index: 26,
                value: "E",
                is_between_double_curly: true,
            },
        );
        assert_eq!(
            blocks[9],
            Block {
                index: 29,
                value: "a",
                is_between_double_curly: false,
            },
        );
        assert_eq!(
            blocks[10],
            Block {
                index: 32,
                value: "F",
                is_between_double_curly: true,
            },
        );
        assert_eq!(
            blocks[11],
            Block {
                index: 35,
                value: "r",
                is_between_double_curly: false,
            },
        );
        assert_eq!(
            blocks[12],
            Block {
                index: 38,
                value: "G",
                is_between_double_curly: true,
            },
        );

        let blocks = super::collect_blocks_from_string("{{VAR{N}AME}}").unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks[0],
            Block {
                index: 2,
                value: "VAR{N}AME",
                is_between_double_curly: true,
            }
        );

        let blocks = super::collect_blocks_from_string("{foobar").unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks[0],
            Block {
                index: 0,
                value: "{foobar",
                is_between_double_curly: false,
            }
        );

        assert!(matches!(
            super::collect_blocks_from_string("{{VAR{N}}}AME}}"),
            Err(CollectBlocksFromTemplateError::ThereIsNoOpenedBlock {
                block_end_offset: 13
            })
        ));

        assert!(matches!(
            super::collect_blocks_from_string("{{foobar"),
            Err(CollectBlocksFromTemplateError::OpenedBlockIsNotClosed {
                block_start_offset: 2
            })
        ));
        assert!(matches!(
            super::collect_blocks_from_string("{{foobar}"),
            Err(CollectBlocksFromTemplateError::OpenedBlockIsNotClosed {
                block_start_offset: 2
            })
        ));
        assert!(matches!(
            super::collect_blocks_from_string("foo{{bar"),
            Err(CollectBlocksFromTemplateError::OpenedBlockIsNotClosed {
                block_start_offset: 5
            })
        ));
        assert!(matches!(
            super::collect_blocks_from_string("foo{{bar}"),
            Err(CollectBlocksFromTemplateError::OpenedBlockIsNotClosed {
                block_start_offset: 5
            })
        ));
        assert!(matches!(
            super::collect_blocks_from_string("foobar{{"),
            Err(CollectBlocksFromTemplateError::OpenedBlockIsNotClosed {
                block_start_offset: 8
            })
        ));
        assert!(matches!(
            super::collect_blocks_from_string("foobar{{}"),
            Err(CollectBlocksFromTemplateError::OpenedBlockIsNotClosed {
                block_start_offset: 8
            })
        ));
        assert!(matches!(
            super::collect_blocks_from_string("foobar{}}"),
            Err(CollectBlocksFromTemplateError::ThereIsNoOpenedBlock {
                block_end_offset: 7,
            })
        ));
    }

    #[test]
    fn decode_greeting() {
        let resolve_variable = |varname: &str| match varname {
            "name" => Some("Jane"),
            _ => None,
        };

        assert_eq!(
            decode_string("Hello, {{name}}", resolve_variable).unwrap(),
            "Hello, Jane".to_string(),
        );

        assert_eq!(
            decode_string("Hello, {{{name}}}", resolve_variable).unwrap(),
            "Hello, {{name}}".to_string(),
        );

        assert_eq!(
            decode_string("Hello, {{{{name}}}}", resolve_variable).unwrap(),
            "Hello, {{{name}}}".to_string(),
        );

        assert_eq!(
            decode_string("Hello, {{{{{name}}}}}", resolve_variable).unwrap(),
            "Hello, {{Jane}}".to_string(),
        );

        assert_eq!(
            decode_string("Hello, {{{{{{name}}}}}}", resolve_variable).unwrap(),
            "Hello, {{{{name}}}}".to_string(),
        );
    }

    #[test]
    fn decode() {
        let resolve_variable = |varname: &str| match varname {
            "HOME" => Some("__HOME__"),
            "ROOTHOME" => Some("__ROOT__"),
            _ => None,
        };

        assert_eq!(
            decode_string("foo{{HOME}}bar{{ROOTHOME}}", resolve_variable).unwrap(),
            "foo__HOME__bar__ROOT__".to_string(),
        );
        assert_eq!(
            decode_string("{{HOME}}foobar{{ROOTHOME}}", resolve_variable).unwrap(),
            "__HOME__foobar__ROOT__".to_string(),
        );
        assert_eq!(
            decode_string("foo{bar{{HOME}}", resolve_variable).unwrap(),
            "foo{bar__HOME__".to_string(),
        );
        assert_eq!(
            decode_string("f{{{o}o{{HOME}}bar", resolve_variable).unwrap(),
            "f{{o}o__HOME__bar".to_string(),
        );
        assert_eq!(
            decode_string("f{o}}}o{{HOME}}bar", resolve_variable).unwrap(),
            "f{o}}o__HOME__bar".to_string(),
        );
        assert_eq!(
            decode_string("f{{{o}}}o{{HOME}}bar", resolve_variable).unwrap(),
            "f{{o}}o__HOME__bar".to_string(),
        );
        assert_eq!(
            decode_string("f{{{{o}}}}o{{HOME}}bar", resolve_variable).unwrap(),
            "f{{{o}}}o__HOME__bar".to_string(),
        );
        assert_eq!(
            decode_string("f{{o}o{{HOME}}bar", resolve_variable)
                .err()
                .unwrap(),
            DecodeStringError::CouldNotResolveVariable {
                variable_name: "o}o{{HOME".to_string(),
            },
        );
        assert_eq!(
            decode_string("foo{{INVALID_VARNAME}}bar", resolve_variable)
                .err()
                .unwrap(),
            DecodeStringError::CouldNotResolveVariable {
                variable_name: "INVALID_VARNAME".to_string(),
            },
        );

        assert_eq!(
            decode_string("{", resolve_variable).unwrap(),
            "{".to_string(),
        );
        assert_eq!(
            decode_string("{{{", resolve_variable).unwrap(),
            "{{".to_string(),
        );
        assert_eq!(
            decode_string("{{{{", resolve_variable).unwrap(),
            "{{{".to_string(),
        );
        assert_eq!(
            decode_string("{{{{{{", resolve_variable).unwrap(),
            "{{{{".to_string(),
        );
        assert_eq!(
            decode_string("{{{{{{{", resolve_variable).unwrap(),
            "{{{{{".to_string(),
        );
        assert_eq!(
            decode_string("{{{{{{{{{", resolve_variable).unwrap(),
            "{{{{{{".to_string(),
        );
    }
}
