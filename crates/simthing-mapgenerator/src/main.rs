use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use simthing_mapgenerator::{
    generate_galaxy_with_structure, generate_success_galaxy_with_preview,
    structure_options_from_params, success_galaxy_1000_params, ArbitraryHyperlaneSourceMode,
    GenerationMode, MapGeneratorParams, OutputFormat, PartitionMethod, ScenarioEmitter,
    ScenarioEmitterConfig, ShapeRegistry, ValidationError, GALAXY_PREVIEW_PNG_SIZE,
};

#[derive(Debug, Parser)]
#[command(
    name = "mapgen",
    about = "MapGeneratorCLI producer — declarative galaxy generation + optional 1000×1000 preview PNG"
)]
struct Cli {
    /// Load full parameter JSON from a file (overrides individual flags when present).
    #[arg(long)]
    params: Option<PathBuf>,

    /// Use the proven PR11 1000-star elliptical success-galaxy preset.
    #[arg(long)]
    success_galaxy: bool,

    #[arg(long, value_enum, default_value_t = CliMode::Procedural)]
    mode: CliMode,

    #[arg(long)]
    shape: Option<String>,

    #[arg(long)]
    num_stars: Option<u32>,

    #[arg(long)]
    radius: Option<f64>,

    #[arg(long)]
    core_radius: Option<f64>,

    #[arg(long)]
    lattice_size: Option<u32>,

    #[arg(long)]
    cluster_count: Option<u32>,

    #[arg(long)]
    cluster_radius: Option<f64>,

    #[arg(long, value_enum, default_value_t = CliPartitionMethod::BreadthFirst)]
    partition_method: CliPartitionMethod,

    #[arg(long)]
    max_hyperlane_distance: Option<f64>,

    #[arg(long)]
    num_hyperlanes_min: Option<u32>,

    #[arg(long)]
    num_hyperlanes_max: Option<u32>,

    #[arg(long)]
    random_hyperlanes: Option<bool>,

    #[arg(long)]
    num_wormhole_pairs: Option<u32>,

    #[arg(long)]
    num_gateways: Option<u32>,

    #[arg(long)]
    num_nebulas: Option<u32>,

    #[arg(long)]
    nebula_size: Option<f64>,

    #[arg(long, value_enum, default_value_t = CliOutputFormat::Clause)]
    output_format: CliOutputFormat,

    #[arg(long)]
    seed: Option<u64>,

    /// Write generated `static_galaxy_scenario` text to this path.
    #[arg(long)]
    output: Option<PathBuf>,

    /// Write a 1000×1000 PNG preview of the generated galaxy.
    #[arg(long)]
    preview_png: Option<PathBuf>,

    /// Validate and print parameter summary without generation or emission.
    #[arg(long)]
    dry_run: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliMode {
    Procedural,
    ArbitraryStatic,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliPartitionMethod {
    BreadthFirst,
    DepthFirst,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliOutputFormat {
    Clause,
    StaticGalaxy,
    Manifest,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let registry = ShapeRegistry::default();
    let mut params = if cli.success_galaxy {
        success_galaxy_1000_params()
    } else if let Some(ref path) = cli.params {
        let json = std::fs::read_to_string(path)?;
        MapGeneratorParams::from_json_str(&json)?
    } else {
        MapGeneratorParams::default()
    };

    apply_cli_overrides(&mut params, &cli);

    params
        .validate(&registry)
        .map_err(|err| format_validation(err))?;

    if cli.dry_run || params.output.dry_run {
        println!("{}", params.dry_run_summary());
        return Ok(());
    }

    let preview_path = cli.preview_png.clone().or_else(|| {
        if cli.success_galaxy {
            Some(PathBuf::from("success_galaxy.png"))
        } else {
            params.output.output.clone()
        }
    });
    let scenario_path = cli.output.clone();

    let generation = if cli.success_galaxy {
        generate_success_galaxy_with_preview(&registry)?
    } else {
        let (hyperlane, special, partition, cluster) = structure_options_from_params(&params)?;
        generate_galaxy_with_structure(
            &params,
            &registry,
            None,
            &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
            Some(hyperlane),
            Some(special),
            Some(partition),
            Some(cluster),
        )?
    };

    if let Some(ref path) = scenario_path {
        std::fs::write(path, generation.scenario.as_str())?;
        println!(
            "wrote scenario ({} systems) -> {}",
            generation.placement.systems.len(),
            path.display()
        );
    }

    if let Some(ref path) = preview_path {
        let png = generation.render_preview_png()?;
        std::fs::write(path, png)?;
        println!(
            "wrote {}x{} preview PNG -> {}",
            GALAXY_PREVIEW_PNG_SIZE,
            GALAXY_PREVIEW_PNG_SIZE,
            path.display()
        );
    }

    if scenario_path.is_none() && preview_path.is_none() {
        println!(
            "generated {} systems; pass --output and/or --preview-png to write artifacts",
            generation.placement.systems.len()
        );
    }

    Ok(())
}

fn apply_cli_overrides(params: &mut MapGeneratorParams, cli: &Cli) {
    if cli.success_galaxy {
        return;
    }
    params.mode = match cli.mode {
        CliMode::Procedural => GenerationMode::Procedural,
        CliMode::ArbitraryStatic => {
            params.shape.shape = "arbitrary_static".into();
            params.arbitrary.hyperlane_source_mode = ArbitraryHyperlaneSourceMode::AddHyperlane;
            if params.arbitrary.explicit_point_cloud_path.is_none() {
                params.arbitrary.explicit_point_cloud_path =
                    Some(PathBuf::from("placeholder/points.json"));
            }
            GenerationMode::ArbitraryStatic
        }
    };
    if let Some(shape) = &cli.shape {
        params.shape.shape = shape.clone();
    }
    if let Some(v) = cli.num_stars {
        params.scale_core.num_stars = v;
    }
    if let Some(v) = cli.radius {
        params.scale_core.radius = v;
    }
    if let Some(v) = cli.core_radius {
        params.scale_core.core_radius = v;
    }
    if let Some(v) = cli.lattice_size {
        params.scale_core.lattice_size = Some(v);
    }
    if let Some(v) = cli.cluster_count {
        params.clustering.cluster_count = Some(v);
    }
    if let Some(v) = cli.cluster_radius {
        params.clustering.cluster_radius = v;
    }
    params.partitioning.partition_method = match cli.partition_method {
        CliPartitionMethod::BreadthFirst => PartitionMethod::BreadthFirst,
        CliPartitionMethod::DepthFirst => PartitionMethod::DepthFirst,
    };
    if let Some(v) = cli.max_hyperlane_distance {
        params.hyperlane.max_hyperlane_distance = v;
    }
    if let Some(v) = cli.num_hyperlanes_min {
        params.hyperlane.num_hyperlanes_min = v;
    }
    if let Some(v) = cli.num_hyperlanes_max {
        params.hyperlane.num_hyperlanes_max = v;
    }
    if let Some(v) = cli.random_hyperlanes {
        params.hyperlane.random_hyperlanes = v;
    }
    if let Some(v) = cli.num_wormhole_pairs {
        params.special_routes.num_wormhole_pairs = v;
    }
    if let Some(v) = cli.num_gateways {
        params.special_routes.num_gateways = v;
    }
    if let Some(v) = cli.num_nebulas {
        params.nebula.num_nebulas = v;
    }
    if let Some(v) = cli.nebula_size {
        params.nebula.nebula_size = v;
    }
    params.output.output_format = match cli.output_format {
        CliOutputFormat::Clause => OutputFormat::Clause,
        CliOutputFormat::StaticGalaxy => OutputFormat::StaticGalaxy,
        CliOutputFormat::Manifest => OutputFormat::Manifest,
    };
    if let Some(seed) = cli.seed {
        params.seed = seed;
    }
    params.output.dry_run = cli.dry_run;
    if let Some(path) = &cli.preview_png {
        params.output.output = Some(path.clone());
    }
}

fn format_validation(err: ValidationError) -> String {
    format!("validation error: {err}")
}
