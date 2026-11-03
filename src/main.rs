use base64::prelude::{BASE64_URL_SAFE_NO_PAD, Engine as _};
use clap::{Parser, ValueEnum};
use clicmd::{Command, commands::Verbosity, error::CliError, cli_main, messages::{Message, Note}};
use colorchoice_clap::Color;
use std::process::ExitCode;
use uuid::{Uuid, Variant}; 
use tracing::{info, debug};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[command(flatten)]
    verbosity: Verbosity,

    #[command(flatten)]
    color: Color,

    /// The number of UUIDs to generate.
    #[arg(long, short = 'c', default_value_t = 1)]
    count: u16,

    /// The format to use for displaying UUIDs.
    #[arg(long, short = 'f', value_enum, default_value_t = Format::DashesOnly)]
    format: Format,

    /// The UUID version to use.
    #[arg(long, short = 'n', value_enum, default_value_t = Version::V4)]
    version_number: Version,

    /// The namespace ID to use for name-based UUIDs (v3 and v5 only).
    #[arg(long, short = 'i')]
    namespace_id: Option<NamespaceId>,

    #[arg(requires = "namespace_id")]
    namespace_name: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
    #[default]
    DashesOnly,
    BracesOnly,
    DashesAndBraces,
    Plain,
    Compact,
    Urn,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Version {
    V1,
    V3,
    #[default]
    V4,
    V5,
    V6,
    V7,
    V8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum NamespaceId {
    Dns, Url, Oid, X500,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Command for Cli {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        self.verbosity.execute()?;
        self.color.execute();

        info!("Generating {} UUID(s) of version {:?}, as {:?}", self.count, self.version_number, self.format);

        if (self.version_number == Version::V3 || self.version_number == Version::V5) && self.namespace_id.is_none() {
            missing_namespace_id(self.version_number).print();
             Ok(ExitCode::FAILURE)
        } else {
            // The node ID is unique for this process.
            let mut node_id = [0u8; 6];
            rand::fill(&mut node_id[..]);

            let namespace_id = self.namespace_id.map(|id| match id {
                NamespaceId::Dns => Uuid::NAMESPACE_DNS,
                NamespaceId::Url => Uuid::NAMESPACE_URL,
                NamespaceId::Oid => Uuid::NAMESPACE_OID,
                NamespaceId::X500 => Uuid::NAMESPACE_X500,
            });
            let namespace_name = self.namespace_name.unwrap_or_default();
            debug!("Namespace ID: {:?}, Namespace Name: {}", namespace_id, namespace_name);

            for _ in 0..self.count {
                let uuid = match self.version_number {
                    Version::V1 => {
                        Uuid::now_v1(&node_id)
                    },
                    Version::V3 => Uuid::new_v3(&namespace_id.unwrap(), namespace_name.as_bytes()),
                    Version::V4 => Uuid::new_v4(),
                    Version::V5 => Uuid::new_v5(&namespace_id.unwrap(), namespace_name.as_bytes()),
                    Version::V6 => {
                        Uuid::now_v6(&node_id)
                    },
                    Version::V7 => Uuid::now_v7(),
                    Version::V8 => {
                        let mut buf = [0u8; 16];
                        rand::fill(&mut buf[..]);
                        Uuid::new_v8(buf)
                    },
                };
                debug!("UUID version: {:?}, variant: {:?}", uuid.get_version_num(), uuid.get_variant());
                if self.format == Format::Urn
                    && !((uuid.get_version_num() == 4 || uuid.get_version_num() == 7)
                    && uuid.get_variant() == Variant::RFC4122)
                {
                    invalid_uuid_for_urn(uuid).print();
                    return Ok(ExitCode::FAILURE);
                } else {
                    match self.format {
                        Format::DashesOnly => println!("{}", uuid.hyphenated()),
                        Format::BracesOnly => println!("{{{}}}", uuid.simple()),
                        Format::DashesAndBraces => println!("{}", uuid.braced()),
                        Format::Plain => println!("{}", uuid.simple()),
                        Format::Compact => println!("{}", BASE64_URL_SAFE_NO_PAD.encode(uuid.as_bytes())),
                        Format::Urn => println!("urn:uuid:{}", uuid.hyphenated()),
                    }
                }
            }
            Ok(ExitCode::SUCCESS)
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Main
// ------------------------------------------------------------------------------------------------

fn main() -> ExitCode {
    cli_main::<Cli>()
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn invalid_uuid_for_urn(uuid: Uuid) -> Message {
    Message::error("Invalid UUID for URN format")
        .with_context(format!("UUID has version `{}` and variant `{}`", uuid.get_version_num(), uuid.get_variant()))
        .with_note(Note::help(format!(
            "only UUIDs of version 4 or 7 with variant RFC4122 are valid for URN format"
        )))
}

fn missing_namespace_id(version: Version) -> Message {
    Message::error("Missing namespace ID/value for name-based UUID")
        .with_context(format!("requested UUID version `{:?}`", version))
        .with_note(Note::help(format!(
            "name-based UUIDs (v3 and v5) require a namespace ID and value; add the `--namespace-id` and value arguments"
        )))
}
