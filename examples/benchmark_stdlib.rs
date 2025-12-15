use std::time::Instant;
use syster::project::StdLibLoader;
use syster::semantic::Workspace;

fn main() {
    let mut workspace = Workspace::new();
    let loader = StdLibLoader::new();

    println!("Benchmarking standard library loading...");
    println!("==========================================\n");

    // Measure file collection
    let start = Instant::now();
    let file_count = std::fs::read_dir("sysml.library")
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path().is_file()
                        && e.path()
                            .extension()
                            .map(|ext| ext == "sysml" || ext == "kerml")
                            .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0);
    let collect_time = start.elapsed();
    println!("File collection: {:?} ({} files)", collect_time, file_count);

    // Measure total load time
    let start = Instant::now();
    if let Err(e) = loader.load(&mut workspace) {
        eprintln!("Load error: {}", e);
        return;
    }
    let load_time = start.elapsed();
    println!("Total load time: {:?}", load_time);
    println!("Files loaded: {}", workspace.file_count());
    if workspace.file_count() > 0 {
        println!(
            "Average per file: {:?}",
            load_time / workspace.file_count() as u32
        );
    }

    // Measure symbol population
    let start = Instant::now();
    if let Err(e) = workspace.populate_all() {
        eprintln!("Population error: {}", e);
        return;
    }
    let populate_time = start.elapsed();
    println!("\nSymbol population: {:?}", populate_time);
    if workspace.file_count() > 0 {
        println!(
            "Average per file: {:?}",
            populate_time / workspace.file_count() as u32
        );
    }

    // Memory usage estimate
    let symbol_count = workspace.symbol_table().all_symbols().len();
    println!("\nSymbol table size: {} symbols", symbol_count);
    println!("Index size: {} entries", workspace.file_count());
}
