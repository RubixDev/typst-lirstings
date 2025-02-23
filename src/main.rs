use std::{
    fs::{self, File},
    io::{self, Read as _, Write as _},
    path::PathBuf,
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context as _, Result};
use clap::{builder::TypedValueParser, Parser, ValueHint};
use syntastica::{language_set::SupportedLanguage as _, renderer, Processor};
use syntastica_parsers_git::{Lang, LanguageSetImpl};

#[derive(clap::Parser)]
#[command(author, version, about)]
struct Cli {
    /// The command to run.
    #[command(subcommand)]
    subcommand: SubCommand,
}

/// What to do.
#[derive(clap::Subcommand)]
enum SubCommand {
    /// Compiles an input file into a supported output format.
    #[command(visible_alias = "c")]
    Compile {
        /// Path to input Typst file. Use `-` to read input from stdin.
        #[clap(value_parser = input_value_parser(), value_hint = ValueHint::FilePath)]
        input: Input,

        /// Path to output file (PDF, PNG, SVG, or HTML). Use `-` to write output to
        /// stdout.
        ///
        /// For output formats emitting one file per page (PNG & SVG), a page number
        /// template must be present if the source document renders to multiple
        /// pages. Use `{p}` for page numbers, `{0p}` for zero padded page numbers
        /// and `{t}` for page count. For example, `page-{0p}-of-{t}.png` creates
        /// `page-01-of-10.png`, `page-02-of-10.png`, and so on.
        #[clap(
            required_if_eq("input", "-"),
            value_parser = output_value_parser(),
            value_hint = ValueHint::FilePath,
        )]
        output: Option<Output>,

        /// Other arguments to pass to Typst.
        typst_args: Vec<String>,

        /// Path to the Typst executable.
        #[arg(long)]
        typst_path: Option<PathBuf>,

        /// Configures the project root (for absolute paths).
        #[arg(long, env = "TYPST_ROOT", value_name = "DIR")]
        root: Option<PathBuf>,
    },
    /// Print the Typst package code.
    PrintTyp,
}

/// An input that is either stdin or a real path.
#[derive(Clone)]
pub enum Input {
    /// Stdin, represented by `-`.
    Stdin,
    /// A non-empty path.
    Path(PathBuf),
}

/// The clap value parser used by `SharedArgs.input`
fn input_value_parser() -> impl TypedValueParser<Value = Input> {
    clap::builder::OsStringValueParser::new().try_map(|value| {
        if value.is_empty() {
            Err(clap::Error::new(clap::error::ErrorKind::InvalidValue))
        } else if value == "-" {
            Ok(Input::Stdin)
        } else {
            Ok(Input::Path(value.into()))
        }
    })
}

/// An output that is either stdout or a real path.
#[derive(Debug, Clone)]
pub enum Output {
    /// Stdout, represented by `-`.
    Stdout,
    /// A non-empty path.
    Path(PathBuf),
}

/// The clap value parser used by `CompileCommand.output`
fn output_value_parser() -> impl TypedValueParser<Value = Output> {
    clap::builder::OsStringValueParser::new().try_map(|value| {
        // Empty value also handled by clap for `Option<Output>`
        if value.is_empty() {
            Err(clap::Error::new(clap::error::ErrorKind::InvalidValue))
        } else if value == "-" {
            Ok(Output::Stdout)
        } else {
            Ok(Output::Path(value.into()))
        }
    })
}

#[derive(serde::Deserialize, Debug)]
struct QueryResult {
    lang: Option<String>,
    text: String,
    theme: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.subcommand {
        SubCommand::Compile {
            input,
            output,
            root,
            typst_args,
            typst_path,
        } => {
            let mut stdin = String::new();
            if let Input::Stdin = input {
                io::stdin()
                    .read_to_string(&mut stdin)
                    .context("reading stdin")?;
            }
            let mut cmd = typst_path
                .as_ref()
                .map_or_else(|| Command::new("typst"), Command::new);
            cmd.arg("query").stdout(Stdio::piped());
            match &input {
                Input::Stdin => cmd.arg("-").stdin(Stdio::piped()),
                Input::Path(path) => cmd.arg(path),
            };
            if let Some(root) = &root {
                cmd.arg("--root").arg(root);
            }
            let mut child = cmd
                .arg("<__lirstings>")
                .arg("--field=value")
                .args(&typst_args)
                .spawn()
                .context("spawning typst query process")?;
            if let Input::Stdin = input {
                child
                    .stdin
                    .take()
                    .context("obtaining stdin handle")?
                    .write_all(stdin.as_bytes())
                    .context("writing stdin")?;
            }
            let res = child.wait_with_output().context("running typst query")?;
            let res = serde_json::from_slice::<Vec<QueryResult>>(&res.stdout)
                .context("failed parsing query results")?;

            let set = LanguageSetImpl::new();
            let mut processor = Processor::new(&set);
            let mut highlighted = vec![];
            for raw in &res {
                highlighted.push(
                    raw.lang
                        .as_ref()
                        .and_then(|l| get_lang(l))
                        .map(|lang| {
                            Result::<_>::Ok(renderer::resolve_styles(
                                &processor
                                    .process(&raw.text, lang)
                                    .context("highlighting failed")?,
                                syntastica_themes::from_str(&raw.theme)
                                    .with_context(|| format!("unknown theme '{}'", raw.theme))?,
                            ))
                        })
                        .transpose()?,
                );
            }

            let filename = format!(
                "__lirstings-data-{}.json",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            );
            let file_path = match &root {
                Some(root) => root.join(&filename),
                None => match &input {
                    Input::Stdin => PathBuf::from(&filename),
                    Input::Path(path) => path
                        .parent()
                        .map_or_else(|| PathBuf::from(&filename), |path| path.join(&filename)),
                },
            };
            let file =
                File::create(&file_path).with_context(|| format!("failed to create {filename}"))?;
            serde_json::to_writer(file, &highlighted)
                .with_context(|| format!("failed writing {filename}"))?;

            let mut cmd = typst_path
                .as_ref()
                .map_or_else(|| Command::new("typst"), Command::new);
            cmd.arg("compile");
            match &input {
                Input::Stdin => cmd.arg("-").stdin(Stdio::piped()),
                Input::Path(path) => cmd.arg(path),
            };
            match &output {
                Some(Output::Stdout) => _ = cmd.arg("-"),
                Some(Output::Path(path)) => _ = cmd.arg(path),
                None => {}
            }
            if let Some(root) = &root {
                cmd.arg("--root").arg(root);
            }
            let mut child = cmd
                .arg(format!("--input=__lirstings=/{filename}"))
                .args(&typst_args)
                .spawn()
                .context("spawning typst process")?;
            if let Input::Stdin = input {
                child
                    .stdin
                    .take()
                    .context("obtaining stdin handle")?
                    .write_all(stdin.as_bytes())
                    .context("writing stdin")?;
            }
            child.wait().context("failed compiling typst document")?;

            _ = fs::remove_file(&file_path);
        }
        SubCommand::PrintTyp => print!("{}", include_str!("./lib.typ")),
    }
    Ok(())
}

fn get_lang(name: &str) -> Option<Lang> {
    Lang::for_name(name)
        .ok()
        .or_else(|| Lang::for_injection(name))
}
