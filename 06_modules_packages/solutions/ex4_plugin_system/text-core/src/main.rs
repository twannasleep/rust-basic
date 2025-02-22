use anyhow::{Result, Context};
use clap::{Parser, Subcommand};
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use text_plugin_api::{TextProcessor, CreatePlugin, PLUGIN_CREATOR_FUNCTION};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available plugins
    List,
    
    /// Process text using a specific plugin
    Process {
        /// Name of the plugin to use
        #[arg(short, long)]
        plugin: String,
        
        /// Text to process
        #[arg(short, long)]
        text: String,
    },
}

struct PluginManager {
    plugins: HashMap<String, (Library, Box<dyn TextProcessor>)>,
}

impl PluginManager {
    fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    fn load_plugins(&mut self, plugin_dir: &Path) -> Result<()> {
        // Clear existing plugins
        self.plugins.clear();

        // Find all dynamic libraries in the plugin directory
        for entry in WalkDir::new(plugin_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().map_or(false, |ext| {
                    ext == std::env::consts::DLL_EXTENSION
                })
            })
        {
            self.load_plugin(entry.path())?;
        }

        Ok(())
    }

    fn load_plugin(&mut self, path: &Path) -> Result<()> {
        unsafe {
            let library = Library::new(path)
                .with_context(|| format!("Failed to load plugin from {:?}", path))?;

            let creator: Symbol<CreatePlugin> = library
                .get(PLUGIN_CREATOR_FUNCTION.as_bytes())
                .with_context(|| format!("Plugin {:?} doesn't have a create_plugin function", path))?;

            let plugin = creator();
            let name = plugin.name().to_string();

            self.plugins.insert(name, (library, plugin));
        }

        Ok(())
    }

    fn list_plugins(&self) {
        if self.plugins.is_empty() {
            println!("No plugins loaded.");
            return;
        }

        println!("Available plugins:");
        for plugin in self.plugins.values() {
            println!("- {} ({})", plugin.1.name(), plugin.1.description());
        }
    }

    fn process_text(&self, plugin_name: &str, text: &str) -> Result<String> {
        let (_, plugin) = self.plugins.get(plugin_name)
            .ok_or_else(|| anyhow::anyhow!("Plugin '{}' not found", plugin_name))?;
        
        Ok(plugin.process(text))
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        // Plugins must be dropped before their libraries
        self.plugins.clear();
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Create plugin manager and load plugins
    let mut manager = PluginManager::new();
    let plugin_dir = get_plugin_dir()?;
    manager.load_plugins(&plugin_dir)?;

    match cli.command {
        Commands::List => {
            manager.list_plugins();
        }
        Commands::Process { plugin, text } => {
            match manager.process_text(&plugin, &text) {
                Ok(result) => println!("Result: {}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    Ok(())
}

fn get_plugin_dir() -> Result<PathBuf> {
    let exe_path = std::env::current_exe()?;
    let plugin_dir = exe_path.parent()
        .ok_or_else(|| anyhow::anyhow!("Could not get executable directory"))?
        .join("plugins");
    
    if !plugin_dir.exists() {
        std::fs::create_dir(&plugin_dir)?;
    }
    
    Ok(plugin_dir)
} 