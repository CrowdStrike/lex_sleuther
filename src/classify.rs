use std::{collections::HashMap, path::PathBuf};

use anyhow::bail;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "classify files like the `file` cli tool",
    arg_required_else_help = true
)]
pub(crate) struct ClassifyArgs {
    /// text files to identify
    #[arg(required(true), value_hint = clap::ValueHint::FilePath)]
    input_files: Vec<PathBuf>,
    /// prints a summary of the results
    #[arg(long, short)]
    summary: bool,
    /// number of top classifications to print in order
    #[arg(long, short, default_value_t = 1)]
    top: usize,
    /// additional information to print along with the matching classification verdict
    #[arg(long, short, default_value_t = VerdictMode::Bare, value_enum)]
    info: VerdictMode,
}

#[derive(ValueEnum, Clone, Debug)]
enum VerdictMode {
    /// only the name
    Bare,
    /// the name and raw score
    Score,
    /// the name and estimated probability
    Probability,
}

pub(crate) fn classify(args: ClassifyArgs) -> anyhow::Result<()> {
    let (input_paths, missing_paths): (Vec<_>, Vec<_>) = args
        .input_files
        .into_iter()
        .partition(|p| p.exists());

    if !missing_paths.is_empty() {
        bail!(
            "the following input paths do not exist: {:?}",
            missing_paths
        );
    }

    // ignore dirs and other non-files
    let input_files: Vec<_> = input_paths.into_iter().filter(|p| p.is_file()).collect();

    let baked_model = lex_sleuther::baked_model();
    let classifications = baked_model.classify_files(&input_files);

    // by default, print the classifications just like the GNU `file` cli does
    for (sample_idx, classification) in classifications.iter().enumerate() {
        let verdicts: Vec<_> = classification
            .verdicts
            .iter()
            .take(args.top)
            .map(|verdict| match args.info {
                VerdictMode::Bare => verdict.label.to_owned(),
                VerdictMode::Score => format!("{} ({:.2})", verdict.label, verdict.score),
                VerdictMode::Probability => {
                    format!("{} ({:.2}%)", verdict.label, verdict.probability * 100.0)
                }
            }).collect();
        println!(
            "{}: {}",
            input_files[sample_idx].display(),
            verdicts.join(", ")
        );
    }

    // print a summary OPTIONALLY
    if args.summary {
        let total_files = input_files.len();
        let totals = classifications
            .iter()
            .map(|classification| classification.verdicts.first().unwrap())
            .fold(HashMap::new(), |mut accum: HashMap<&str, usize>, curr| {
                let count = accum.get(&curr.label).unwrap_or(&0).to_owned();
                accum.insert(curr.label, count + 1);
                accum
            });

        for (class_label, count) in totals {
            println!(
                "{}: {}/{} = {:.2}%",
                class_label,
                count,
                total_files,
                count as f64 / total_files as f64 * 100.0
            );
        }
    }

    Ok(())
}
