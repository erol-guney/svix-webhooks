// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

#[derive(Args, Clone)]
pub struct StreamingSinkListOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// The sorting order of the returned items
    #[arg(long)]
    pub order: Option<Ordering>,
}

impl From<StreamingSinkListOptions> for svix::api::StreamingSinkListOptions {
    fn from(value: StreamingSinkListOptions) -> Self {
        let StreamingSinkListOptions {
            limit,
            iterator,
            order,
        } = value;
        Self {
            limit,
            iterator,
            order,
        }
    }
}

#[derive(Args, Clone)]
pub struct StreamingSinkCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamingSinkCreateOptions> for svix::api::StreamingSinkCreateOptions {
    fn from(value: StreamingSinkCreateOptions) -> Self {
        let StreamingSinkCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct StreamingSinkRotateSecretOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamingSinkRotateSecretOptions> for svix::api::StreamingSinkRotateSecretOptions {
    fn from(value: StreamingSinkRotateSecretOptions) -> Self {
        let StreamingSinkRotateSecretOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct StreamingSinkArgs {
    #[command(subcommand)]
    pub command: StreamingSinkCommands,
}

#[derive(Subcommand)]
pub enum StreamingSinkCommands {
    /// List of all the stream's sinks.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n",
            "Example:   svix streaming sink list strm_000000000000000000000\n",
            "\n",
            "{all-args}{after-help}",
        ))]
    List {
        stream_id: String,
        #[clap(flatten)]
        options: StreamingSinkListOptions,
    },
    /// Creates a new sink.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n",
            "Example:   svix streaming sink create strm_000000000000000000000\n",
            "\n",
            "{all-args}{after-help}",
        ))]
    Create {
        stream_id: String,
        stream_sink_in: Option<crate::json::JsonOf<StreamSinkIn>>,
        #[clap(flatten)]
        options: StreamingSinkCreateOptions,
    },
    /// Get a sink by id or uid.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n",
            "Example:   svix streaming sink get strm_000000000000000000000 sink_000000000000000000000\n",
            "\n",
            "{all-args}{after-help}",
        ))]
    Get { stream_id: String, sink_id: String },
    /// Update a sink.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n",
            "Example:   svix streaming sink update strm_000000000000000000000 sink_000000000000000000000\n",
            "\n",
            "{all-args}{after-help}",
        ))]
    Update {
        stream_id: String,
        sink_id: String,
        stream_sink_in: Option<crate::json::JsonOf<StreamSinkIn>>,
    },
    /// Delete a sink.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n",
            "Example:   svix streaming sink delete strm_000000000000000000000 sink_000000000000000000000\n",
            "\n",
            "{all-args}{after-help}",
        ))]
    Delete { stream_id: String, sink_id: String },
    /// Partially update a sink.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n",
            "Example:   svix streaming sink patch strm_000000000000000000000 sink_000000000000000000000\n",
            "\n",
            "{all-args}{after-help}",
        ))]
    Patch {
        stream_id: String,
        sink_id: String,
        stream_sink_patch: Option<crate::json::JsonOf<StreamSinkPatch>>,
    },
    /// Get the sink's signing secret (only supported for http sinks)
    ///
    /// This is used to verify the authenticity of the delivery.
    ///
    /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n",
            "Example:   svix streaming sink get-secret strm_000000000000000000000 sink_000000000000000000000\n",
            "\n",
            "{all-args}{after-help}",
        ))]
    GetSecret { stream_id: String, sink_id: String },
    /// Rotates the signing secret (only supported for http sinks).
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n",
            "Example:   svix streaming sink rotate-secret strm_000000000000000000000 sink_000000000000000000000\n",
            "\n",
            "{all-args}{after-help}",
        ))]
    RotateSecret {
        stream_id: String,
        sink_id: String,
        endpoint_secret_rotate_in: Option<crate::json::JsonOf<EndpointSecretRotateIn>>,
        #[clap(flatten)]
        options: StreamingSinkRotateSecretOptions,
    },
    /// Set or unset the transformation code associated with this sink.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n",
            "Example:   svix streaming sink transformation-partial-update strm_000000000000000000000 sink_000000000000000000000\n",
            "\n",
            "{all-args}{after-help}",
        ))]
    TransformationPartialUpdate {
        stream_id: String,
        sink_id: String,
        sink_transform_in: Option<crate::json::JsonOf<SinkTransformIn>>,
    },
}

impl StreamingSinkCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { stream_id, options } => {
                let resp = client
                    .streaming()
                    .sink()
                    .list(stream_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                stream_id,
                stream_sink_in,
                options,
            } => {
                let resp = client
                    .streaming()
                    .sink()
                    .create(
                        stream_id,
                        stream_sink_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { stream_id, sink_id } => {
                let resp = client.streaming().sink().get(stream_id, sink_id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                stream_id,
                sink_id,
                stream_sink_in,
            } => {
                let resp = client
                    .streaming()
                    .sink()
                    .update(
                        stream_id,
                        sink_id,
                        stream_sink_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { stream_id, sink_id } => {
                client.streaming().sink().delete(stream_id, sink_id).await?;
            }
            Self::Patch {
                stream_id,
                sink_id,
                stream_sink_patch,
            } => {
                let resp = client
                    .streaming()
                    .sink()
                    .patch(
                        stream_id,
                        sink_id,
                        stream_sink_patch.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetSecret { stream_id, sink_id } => {
                let resp = client
                    .streaming()
                    .sink()
                    .get_secret(stream_id, sink_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::RotateSecret {
                stream_id,
                sink_id,
                endpoint_secret_rotate_in,
                options,
            } => {
                let resp = client
                    .streaming()
                    .sink()
                    .rotate_secret(
                        stream_id,
                        sink_id,
                        endpoint_secret_rotate_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::TransformationPartialUpdate {
                stream_id,
                sink_id,
                sink_transform_in,
            } => {
                let resp = client
                    .streaming()
                    .sink()
                    .transformation_partial_update(
                        stream_id,
                        sink_id,
                        sink_transform_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
