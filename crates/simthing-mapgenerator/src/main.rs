use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use simthing_mapgenerator::{
    apply_cli_shape_params, build_generation_report, generate_galaxy_with_structure,
    generate_success_galaxy_with_preview, structure_options_from_params,
    success_galaxy_1000_params, visual_spiral_1500_params, write_generation_report_json,
    ArbitraryHyperlaneSourceMode, GalaxyPreviewOptions, GenerationMode, HyperlanePreviewFilter,
    MapGeneratorParams, OutputFormat, PartitionMethod, ReportArtifacts, ScenarioEmitter,
    ScenarioEmitterConfig, ShapeRegistry, ValidationError, DEFAULT_HYPERLANE_RGBA,
    GALAXY_PREVIEW_PNG_SIZE,
};

#[derive(Debug, Parser)]
#[command(
    name = "mapgen",
    about = "MapGeneratorCLI producer — declarative galaxy generation + optional preview PNG"
)]
struct Cli {
    /// Load full parameter JSON from a file (overrides individual flags when present).
    #[arg(long)]
    params: Option<PathBuf>,

    /// Use the proven PR11 1000-star elliptical success-galaxy preset.
    #[arg(long)]
    success_galaxy: bool,

    /// Use the visual remediation 1500-star spiral_4 preset (300×300 lattice).
    #[arg(long)]
    spiral_visual: bool,

    #[arg(long, value_enum, default_value_t = CliMode::Procedural)]
    mode: CliMode,

    #[arg(long)]
    shape: Option<String>,

    /// Shape-specific tuning params, repeatable: `--shape-param arm_width=12 --shape-param jitter=2`.
    #[arg(long = "shape-param", value_name = "KEY=VALUE")]
    shape_params: Vec<String>,

    #[arg(long, alias = "stars")]
    num_stars: Option<u32>,

    #[arg(long)]
    radius: Option<f64>,

    #[arg(long)]
    core_radius: Option<f64>,

    #[arg(long, alias = "lattice-edge")]
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

    /// Target number of base hyperlanes (starlanes) to select (sets num_hyperlanes_default).
    #[arg(long, alias = "num-hyperlanes")]
    num_hyperlanes_target: Option<u32>,

    #[arg(long)]
    random_hyperlanes: Option<bool>,

    /// Add minimal bridges so the galaxy is one connected network (no island clusters). On by default;
    /// this flag is now redundant (kept for back-compat). Use `--allow-disconnected` to opt out.
    #[arg(long)]
    connect_galaxy: bool,

    /// Opt out of the connected-galaxy guarantee — permit orphaned/island systems in the generated map.
    #[arg(long)]
    allow_disconnected: bool,

    /// Skip galaxy partition/bridge generation (clustering still runs).
    #[arg(long)]
    no_partitions: bool,

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

    /// Write a preview PNG of the generated galaxy.
    #[arg(long, alias = "render-png")]
    preview_png: Option<PathBuf>,

    /// Preview PNG edge length in pixels.
    #[arg(long, default_value_t = GALAXY_PREVIEW_PNG_SIZE)]
    png_size: u32,

    /// Which hyperlane couplings to draw in the preview PNG.
    #[arg(long, value_enum, default_value_t = CliHyperlanePreview::Base)]
    hyperlanes: CliHyperlanePreview,

    /// Colour of starlane segments in the preview PNG.
    #[arg(long, value_enum, default_value_t = CliLaneColor::Faint)]
    hyperlane_color: CliLaneColor,

    /// Apply deterministic within-cell star jitter in the preview PNG.
    #[arg(long, default_value_t = true)]
    jitter_stars: bool,

    /// Suppress grid/core-mask debug painting in the preview PNG.
    #[arg(long, default_value_t = true)]
    no_grid: bool,

    /// Draw nebula fields in the preview PNG (off by default).
    #[arg(long)]
    draw_nebulas: bool,

    /// Draw a bright galactic-core glow over the inaccessible core void.
    #[arg(long)]
    draw_core: bool,

    /// Validate and print parameter summary without generation or emission.
    #[arg(long)]
    dry_run: bool,

    /// Write a deterministic machine-readable generation report (JSON).
    #[arg(long)]
    report_json: Option<PathBuf>,
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

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
enum CliHyperlanePreview {
    #[default]
    Base,
    All,
}

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
enum CliLaneColor {
    /// Faint grey-blue (default; matches prior preview output).
    #[default]
    Faint,
    Blue,
    Cyan,
    White,
}

impl CliLaneColor {
    fn rgba(self) -> [u8; 4] {
        match self {
            CliLaneColor::Faint => DEFAULT_HYPERLANE_RGBA,
            CliLaneColor::Blue => [64, 132, 240, 205],
            CliLaneColor::Cyan => [70, 200, 235, 200],
            CliLaneColor::White => [235, 240, 255, 160],
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let registry = ShapeRegistry::default();
    let mut params = if cli.success_galaxy {
        success_galaxy_1000_params()
    } else if is_visual_spiral_request(&cli) {
        visual_spiral_1500_params()
    } else if let Some(ref path) = cli.params {
        let json = std::fs::read_to_string(path)?;
        MapGeneratorParams::from_json_str(&json)?
    } else {
        MapGeneratorParams::default()
    };

    apply_cli_overrides(&mut params, &cli)?;

    params
        .validate(&registry)
        .map_err(|err| format_validation(err))?;

    if cli.dry_run || params.output.dry_run {
        println!("{}", params.dry_run_summary());
        return Ok(());
    }

    let preview_path = cli.preview_png.clone().or_else(|| {
        if is_visual_spiral_request(&cli) {
            Some(PathBuf::from(
                "docs/tests/mapgenerator_cli_visual_spiral_1500.png",
            ))
        } else if cli.success_galaxy {
            Some(PathBuf::from("success_galaxy.png"))
        } else {
            params.output.output.clone()
        }
    });
    let scenario_path = cli.output.clone();

    let generation = if cli.success_galaxy {
        generate_success_galaxy_with_preview(&registry)?
    } else if is_visual_spiral_request(&cli) {
        let (hyperlane, special, partition, _cluster) = structure_options_from_params(&params)?;
        generate_galaxy_with_structure(
            &params,
            &registry,
            None,
            &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
            Some(hyperlane),
            Some(special),
            Some(partition),
            None,
        )?
    } else {
        let (hyperlane, special, partition, cluster) = structure_options_from_params(&params)?;
        generate_galaxy_with_structure(
            &params,
            &registry,
            None,
            &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
            Some(hyperlane),
            Some(special),
            if cli.no_partitions {
                None
            } else {
                Some(partition)
            },
            Some(cluster),
        )?
    };

    if let Some(c) = generation.connectivity {
        println!(
            "connectivity: {} component(s) after {} bridge(s) (was {}; longest bridge {} cells){}",
            c.components_after,
            c.bridges_added,
            c.components_before,
            c.max_bridge_chebyshev,
            if c.components_after == 1 {
                " — one interconnected galaxy, no island clusters"
            } else {
                ""
            }
        );
    }

    if let Some(ref path) = scenario_path {
        std::fs::write(path, generation.scenario.as_str())?;
        println!(
            "wrote scenario ({} systems) -> {}",
            generation.placement.systems.len(),
            path.display()
        );
    }

    if let Some(ref path) = preview_path {
        let preview_options = GalaxyPreviewOptions {
            seed: generation.seed,
            png_size: cli.png_size,
            jitter_stars: cli.jitter_stars,
            draw_nebulas: cli.draw_nebulas,
            draw_core_mask: !cli.no_grid,
            hyperlane_filter: match cli.hyperlanes {
                CliHyperlanePreview::Base => HyperlanePreviewFilter::BaseOnly,
                CliHyperlanePreview::All => HyperlanePreviewFilter::AllCouplings,
            },
            // When connectivity is on (the default), draw every base lane (including the connectivity
            // bridges that tie clusters together); otherwise keep the render distance filter.
            max_hyperlane_chebyshev: if params.hyperlane.ensure_connected {
                None
            } else {
                Some(params.hyperlane.max_hyperlane_distance.round().max(1.0) as u32)
            },
            hyperlane_rgba: cli.hyperlane_color.rgba(),
            draw_core_glow: cli.draw_core,
        };
        let png = generation.render_preview_png_with_options(preview_options)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, png)?;
        println!(
            "wrote {}x{} preview PNG ({} base hyperlane segments) -> {}",
            cli.png_size,
            cli.png_size,
            generation.base_hyperlane_edges.len(),
            path.display()
        );
    }

    if scenario_path.is_none() && preview_path.is_none() && cli.report_json.is_none() {
        println!(
            "generated {} systems; pass --output, --preview-png, and/or --report-json to write artifacts",
            generation.placement.systems.len()
        );
    }

    let mut artifacts = ReportArtifacts::new();
    artifacts.scenario_path = scenario_path.as_deref();
    artifacts.png_path = preview_path.as_deref();
    artifacts.report_path = cli.report_json.as_deref();

    if let Some(ref path) = cli.report_json {
        let report = build_generation_report(&params, &generation, artifacts);
        write_generation_report_json(&report, path)?;
        println!("wrote generation report -> {}", path.display());
    }

    Ok(())
}

fn apply_cli_overrides(
    params: &mut MapGeneratorParams,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    if cli.success_galaxy {
        return Ok(());
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
    apply_cli_shape_params(&mut params.shape, &cli.shape_params).map_err(|err| err.to_string())?;
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
    if let Some(v) = cli.num_hyperlanes_target {
        params.hyperlane.num_hyperlanes_default = v;
    }
    if let Some(v) = cli.random_hyperlanes {
        params.hyperlane.random_hyperlanes = v;
    }
    // Connected-by-default (params default ON). `--connect-galaxy` is redundant but explicit;
    // `--allow-disconnected` is the opt-out and wins if both are given.
    if cli.connect_galaxy {
        params.hyperlane.ensure_connected = true;
    }
    if cli.allow_disconnected {
        params.hyperlane.ensure_connected = false;
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
    Ok(())
}

fn format_validation(err: ValidationError) -> String {
    format!("validation error: {err}")
}

fn is_visual_spiral_request(cli: &Cli) -> bool {
    cli.spiral_visual
        || (cli.shape.as_deref() == Some("spiral_4")
            && cli.num_stars == Some(1500)
            && cli.lattice_size == Some(300))
}
