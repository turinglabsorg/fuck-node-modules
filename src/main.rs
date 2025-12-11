use clap::Parser;
use console::style;
use dialoguer::Confirm;
use humansize::{format_size, DECIMAL};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to search for node_modules folders
    #[arg(default_value = ".")]
    path: PathBuf,
    
    /// Actually delete the folders (dry run by default)
    #[arg(short, long, default_value_t = false)]
    force: bool,
    
    /// Skip confirmation prompts
    #[arg(short, long, default_value_t = false)]
    yes: bool,
    
    /// Only delete node_modules folders older than N days (default: 30)
    #[arg(short, long, default_value_t = 30)]
    older_than: u64,
}

struct NodeModulesStats {
    count: usize,
    total_size: u64,
    folders: Vec<PathBuf>,
    skipped_recent: usize,
    skipped_size: u64,
}

fn find_node_modules_folders(path: &PathBuf, older_than_days: u64) -> Result<NodeModulesStats, Box<dyn std::error::Error>> {
    let mut stats = NodeModulesStats {
        count: 0,
        total_size: 0,
        folders: Vec::new(),
        skipped_recent: 0,
        skipped_size: 0,
    };

    let cutoff_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        - (older_than_days * 24 * 60 * 60);

    for entry in WalkDir::new(path).follow_links(false) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => {
                // Skip directories we can't access (permission errors, broken symlinks, etc.)
                continue;
            }
        };
        
        if entry.file_name() == "node_modules" {
            let folder_path = entry.path().to_path_buf();
            
            // Skip if this is a nested node_modules (like in pnpm's .pnpm structure)
            if folder_path.parent().and_then(|p| p.parent()).map_or(false, |gp| 
                gp.ends_with(".pnpm") || gp.file_name().and_then(|n| n.to_str()) == Some(".pnpm")) {
                continue;
            }
            
            // Get the parent directory (the project folder containing node_modules)
            let parent_path = match folder_path.parent() {
                Some(parent) => parent,
                None => {
                    // Skip if we can't get parent (shouldn't happen, but be safe)
                    stats.skipped_recent += 1;
                    continue;
                }
            };
            
            // Get the most recent modification time of any file in the parent project
            let most_recent_time = get_most_recent_file_time(parent_path)?;
            
            // Calculate folder size (skip errors)
            let mut folder_size = 0;
            for item in WalkDir::new(&folder_path).follow_links(false) {
                if let Ok(item) = item {
                    if let Ok(metadata) = item.metadata() {
                        folder_size += metadata.len();
                    }
                }
                // Silently skip any errors during size calculation
            }
            
            // Skip if the project has been worked on recently
            if most_recent_time > cutoff_time {
                stats.skipped_recent += 1;
                stats.skipped_size += folder_size;
                continue;
            }

            stats.count += 1;
            stats.total_size += folder_size;
            stats.folders.push(folder_path);
        }
    }

    Ok(stats)
}



fn get_most_recent_file_time(path: &std::path::Path) -> Result<u64, Box<dyn std::error::Error>> {
    let mut most_recent = 0u64;
    
    for entry in WalkDir::new(path).follow_links(false) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue, // Skip inaccessible files/directories
        };
        
        // Skip node_modules folders when checking parent project
        if entry.file_name() == "node_modules" {
            continue;
        }
        
        // Only check files (not directories)
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.duration_since(UNIX_EPOCH) {
                        let file_time = duration.as_secs();
                        if file_time > most_recent {
                            most_recent = file_time;
                        }
                    }
                }
            }
        }
    }
    
    Ok(most_recent)
}

fn delete_folder(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if path.exists() {
        std::fs::remove_dir_all(path)?;
        println!("{} {}", style("✓").green(), path.display());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("{}", style("🔍 Searching for node_modules folders...").bold());
    println!("  Looking for folders older than {} days", style(args.older_than.to_string()).cyan());
    
    let stats = find_node_modules_folders(&args.path, args.older_than)?;
    
    if stats.count == 0 {
        if stats.skipped_recent > 0 {
            println!("🎉 No old node_modules folders found!");
            println!("  {} recent folders were skipped ({})", 
                style(stats.skipped_recent.to_string()).yellow(),
                style(format_size(stats.skipped_size, DECIMAL)).yellow()
            );
        } else {
            println!("🎉 No node_modules folders found!");
        }
        return Ok(());
    }
    
    println!("\n{}", style("📊 FOUND OLD NODE_MODULES FOLDERS:").bold().underlined());
    println!("  {} folders ({})", 
        style(format!("{}", stats.count)).bold().cyan(),
        style(format_size(stats.total_size, DECIMAL)).bold().cyan()
    );
    
    // Prominent size warning if large
    if stats.total_size > 1_000_000_000 { // > 1GB
        println!("  ⚠️  {} of disk space can be reclaimed!", 
            style(format_size(stats.total_size, DECIMAL)).bold().yellow()
        );
    }
    
    if stats.skipped_recent > 0 {
        println!("  🔒 {} recent folders were preserved", 
            style(stats.skipped_recent.to_string()).bold().green()
        );
    }
    
    // Show folder list (with limit for large projects)
    if stats.count <= 10 {
        for folder in &stats.folders {
            println!("  - {}", folder.display());
        }
    } else {
        println!("  - {}", style(format!("Showing first 10 of {} folders...", stats.count)).italic());
        for folder in &stats.folders[..10] {
            println!("  - {}", folder.display());
        }
        println!("  - ... and {} more folders", style((stats.count - 10).to_string()).italic());
    }
    
    // Summary before confirmation
    println!("\n{}", style("📋 SUMMARY:").bold().underlined());
    println!("  🗑️  To be deleted: {} folders ({})", 
        style(stats.count.to_string()).bold().red(),
        style(format_size(stats.total_size, DECIMAL)).bold().red()
    );
    
    if stats.skipped_recent > 0 {
        println!("  🔒  Preserved: {} recent folders ({})", 
            style(stats.skipped_recent.to_string()).bold().green(),
            style(format_size(stats.skipped_size, DECIMAL)).bold().green()
        );
    }
    
    let total_possible = stats.total_size + stats.skipped_size;
    println!("  💾  Total possible savings: {}", 
        style(format_size(total_possible, DECIMAL)).bold().blue()
    );
    
    if args.force || args.yes || Confirm::new()
        .with_prompt(format!("Delete {} folders and reclaim {}?", 
            style(stats.count.to_string()).bold(), 
            style(format_size(stats.total_size, DECIMAL)).bold()))
        .default(false)
        .interact()?
    {
        println!("\n{}", style("🗑️  DELETING OLD NODE_MODULES FOLDERS...").bold());
        
        for folder in &stats.folders {
            delete_folder(folder)?;
        }
        
        println!("\n{}", style("🎉 CLEANUP COMPLETE!").bold().green());
        println!("  ✅ Deleted: {} folders", style(stats.count.to_string()).bold().cyan());
        println!("  💾 Reclaimed: {}", style(format_size(stats.total_size, DECIMAL)).bold().green());
        
        if stats.skipped_recent > 0 {
            println!("  🔒 Preserved: {} recent folders", style(stats.skipped_recent.to_string()).bold().yellow());
        }
        
        // Show potential savings if there were skipped folders
        if stats.skipped_recent > 0 {
            let total_possible = stats.total_size + stats.skipped_size;
            println!("  📊 Could reclaim up to: {}", style(format_size(total_possible, DECIMAL)).bold().blue());
        }
    } else {
        println!("\n{}", style("🚫 Operation cancelled.").yellow());
    }
    
    Ok(())
}