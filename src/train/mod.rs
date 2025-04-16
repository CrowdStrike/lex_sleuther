use lex_sleuther::{
    lex,
    regression::{from_adhoc, Regression},
};
use log::info;
use ndarray::{Array1, Array2};
use std::{fs, io::Write, path::PathBuf};

use anyhow::bail;
use chrono::offset::Utc;
use clap::Parser;

/// contains basic data science routines that ndarray lacks
mod stats;

#[derive(Parser, Debug)]
#[command(about = "generate new training data for classification")]
pub(crate) struct TrainArgs {
    /// path to output model file to
    #[arg(short, long, value_hint = clap::ValueHint::FilePath, required = true)]
    output_path: PathBuf,

    /// directories containing samples, each directory is a distinct sample set
    #[arg(value_hint = clap::ValueHint::FilePath, required = true)]
    sample_dirs: Vec<String>,

    /// names to use as class labels, defaults to sample directory names
    #[arg(short, long)]
    labels: Vec<String>,

    /// number of cross-validations to perform when optimizing alpha
    #[arg(short, long, default_value_t = 4)]
    k: usize,

    /// seed to use when randomly shuffling the dataset
    #[arg(short, long, default_value_t = 0x88)]
    seed: u64,
}

pub(crate) fn train(args: TrainArgs) -> anyhow::Result<()> {
    // check existance
    if !args.output_path.parent().is_some_and(|p| p.exists()) {
        bail!(
            "the parent directory of output path {:?} does not exist, not creating",
            args.output_path
        );
    }

    let (sample_paths, missing_paths): (Vec<_>, Vec<_>) = args
        .sample_dirs
        .iter()
        .cloned()
        .map(PathBuf::from)
        .partition(|p| p.exists());

    if !missing_paths.is_empty() {
        bail!(
            "the following sample set paths do not exist: {:?}",
            missing_paths
        );
    }

    if !args.labels.is_empty() && args.labels.len() != sample_paths.len() {
        bail!(
            "you must pass the same number of classification labels ({}), as sample directories ({})",
            args.labels.len(),
            sample_paths.len()
        );
    }

    // filter out non-directories
    let sample_sets: Vec<_> = sample_paths.into_iter().filter(|p| p.is_dir()).collect();

    let class_labels = if args.labels.is_empty() {
        // if necessary, derive class labels from the sample set directory names
        sample_sets
            .iter()
            .map(|p| {
                p.file_name()
                    // use the whole path if the basename isn't usable for some reason
                    .unwrap_or(p.as_os_str())
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect()
    } else {
        args.labels
    };

    info!("weights path = {:?}", args.output_path);
    info!("sample_sets = {:?}", sample_sets);
    info!("class labels = {:?}", class_labels);

    let regression = generate_regression(&sample_sets, args.k, args.seed);

    // write baked model src
    write_baked_rs(&regression, &class_labels, args.output_path)?;

    Ok(())
}

fn generate_regression(sample_sets: &[PathBuf], k: usize, seed: u64) -> impl Regression + use<> {
    // list of every sample file (and the set they belong to)
    let sample_files: Vec<_> = sample_sets
        .iter()
        .enumerate()
        .flat_map(|(i, set_path)| {
            fs::read_dir(set_path)
                .unwrap()
                .flatten()
                .map(move |e| (i, e.path()))
        })
        .collect();
    let num_classes = sample_sets.len();

    // matrix where each row is the frequency of each token in a sample
    let frequency_matrix =
        lex::get_frequency_matrix_from_files(&sample_files.iter().cloned().map(|f| f.1).collect());

    // the ideal solution vectors are where the result is 1.0 for the correct set and 0.0 otherwise
    let actual_classifications: Array1<usize> =
        sample_files.iter().map(|(idx, _)| idx).copied().collect();
    let ideal_vec = actual_classifications
        .iter()
        .flat_map(|&idx| {
            (0..num_classes).map(move |set_idx| if set_idx == idx { 1.0 } else { 0.0 })
        })
        .collect();
    let ideal_matrix =
        Array2::from_shape_vec((actual_classifications.len(), sample_sets.len()), ideal_vec)
            .unwrap();

    // determine the optimal alpha to use with ridge regression
    // for now, the list of alphas to try is baked
    let alphas = [0.0, 0.1, 0.5, 1.0, 5.0, 10.0];
    let best_alpha = stats::determine_ideal_alpha(
        frequency_matrix.view(),
        ideal_matrix.view(),
        k,
        num_classes,
        seed,
        &alphas,
    );

    info!("best alpha value = {}", best_alpha);

    // ridge regression
    let weight_matrix =
        stats::ridge_regression_iter(frequency_matrix.view(), ideal_matrix.view(), &[best_alpha])
            .next()
            .unwrap();
    let regression = from_adhoc(weight_matrix);

    // compute residuals
    let regression_results = regression.results(frequency_matrix.view());
    let closest_fit_vec: Vec<f64> = regression_results
        .iter()
        .flat_map(|res| res.into_scores())
        .copied()
        .collect();
    let closest_fit_matrix =
        Array2::from_shape_vec(ideal_matrix.raw_dim(), closest_fit_vec).unwrap();

    let residual_matrix = &closest_fit_matrix - ideal_matrix;

    // what is the resulting classification of each sample
    let set_classifications: Vec<_> = regression_results
        .iter()
        .map(|res| res.sorted_classes().get(0).unwrap().to_owned())
        .collect();

    // find incorrect results (outliers) and report them (they might be malformed)
    let (correct, _): (Vec<_>, Vec<_>) = set_classifications
        .into_iter()
        .enumerate()
        .partition(|(sample_idx, set_idx)| sample_files[*sample_idx].0 == *set_idx);

    // estimate efficacy
    info!(
        "total efficacy: {} correct / {} total = {:.2}%",
        correct.len(),
        sample_files.len(),
        correct.len() as f64 * 100.0 / sample_files.len() as f64
    );

    // print residuals for the interested
    for (i, set) in sample_sets.iter().enumerate() {
        // print residuals for the interested
        let residual = residual_matrix.column(i).dot(&residual_matrix.column(i));
        info!("{:?} residual = {}", set, residual);
    }

    regression
}

/// Writes our weights and classifications as a Rust source code file that can be included.
/// This is the easiest way to embed the default weights into the main binaries.
fn write_baked_rs<R: Regression>(
    regression: &R,
    class_labels: &Vec<String>,
    output_path: PathBuf,
) -> anyhow::Result<()> {
    let mut weight_file = fs::File::create(&output_path)?;

    let weight_matrix = regression.weights();

    writeln!(
        weight_file,
        "// WARNING: this file is autogenerated by lex_sleuther::train on {}",
        Utc::now()
    )?;
    writeln!(
        weight_file,
        "pub const CLASS_LABELS: [&str; {}] = {:?};",
        class_labels.len(),
        class_labels
    )?;
    writeln!(
        weight_file,
        "pub static WEIGHT_ARRAY: [f64; {}] = {:#?};",
        weight_matrix.len(),
        weight_matrix.as_slice().unwrap()
    )?;

    info!(
        "wrote includable Rust source code to {}",
        output_path.display()
    );

    Ok(())
}
