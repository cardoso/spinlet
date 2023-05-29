use std::collections::HashMap;
use std::path::Path;

use cap_primitives::ambient_authority;
use spinlet::Config;
use spinlet::Context;
use spinlet::Loader;
use spinlet::Spinlet;
use spinlet::spin;
use spinlet_manifest::Manifest;
use tokio::process::Command;
use wasmtime_wasi::Dir;

fn main() {
    human_panic::setup_panic!();
    let pid = std::process::id();
    eprintln!("PID: {}", pid);

    let cwd = Dir::open_ambient_dir(".", ambient_authority()).unwrap_or_else(|error| panic!("Failed to open current working directory: {error}"));
    let root = Path::new(".spinlet");
    
    let config_file = root.join("config.toml");
    let raw_config = std::fs::read_to_string(config_file).unwrap_or_else(|error| panic!("Failed to read config: {error}"));
    let config: Config = toml::from_str(&raw_config).unwrap_or_else(|error| panic!("Failed to parse config: {error}"));

    let spin = if let Ok(spin) = spin::bin_path() {
        spin
    } else {
        "spin".to_string()
    };

    let mut args = std::env::args();

    let _spinlet = args.next().unwrap_or(std::env::current_exe().unwrap().to_str().unwrap().to_string());

    let command = args.next().unwrap_or(config.child().name().to_string());

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|error| panic!("Failed to build runtime: {error}"))
        .block_on(async {
            if let Some(alias) = config.parent().alias(&command) {
                let etc = config.etc().read_dir().unwrap_or_else(|error| panic!("Failed to read spinlets: {error}"));
                let mut manifests = HashMap::<String, Manifest>::new();
                let mut before_hooks = Vec::<String>::new();
                let mut after_hooks = Vec::<String>::new();
                for entry in etc {
                    let Ok(entry) = entry else { continue };
                    let entry = entry.path();
                    let Some(name) = entry.file_name().and_then(|name| name.to_str()) else { continue };
                    let Ok(manifest) = tokio::fs::read_to_string(&entry).await else { continue };
                    let Ok(manifest) = toml::from_str::<Manifest>(&manifest) else { continue };
                    match manifest.hook(alias) {
                        Some(hook) => {
                            if hook.before().enabled() {
                                before_hooks.push(name.to_string());
                            }

                            if hook.after().enabled() {
                                after_hooks.push(name.to_string());
                            }

                            manifests.insert(name.to_string(), manifest);
                        }
                        None => continue
                    }
                }
                eprintln!("Running alias: {}", alias);
                let mut before_tasks = Vec::new();
                for hook in before_hooks {
                    let name = hook.to_string();
                    let binary = config.binary(&name);
                    let Ok(cwd) = cwd.open_dir(".") else { continue };
                    let Some(manifest) = manifests.remove(&name) else { continue };
                    let task = tokio::task::spawn(async move {
                        let Ok(binary) = tokio::fs::read(&binary).await else { return };
                        let context = Context::new(&manifest, cwd);
                        let loader = Loader::new(context);
                        let spinlet = Spinlet::new(loader);
                        
                        match spinlet.run(&binary).await {
                            Ok(success) => if success {
                                eprintln!("Before hook ran successfully");
                            } else {
                                eprintln!("Before hook ran unsuccessfully");
                            },
                            Err(error) => {
                                eprintln!("Failed to run before hook: {error}");
                            }
                        }
                    });

                    before_tasks.push(task);
                }

                for task in before_tasks {
                    match task.await {
                        Ok(_) => eprintln!("Before hook ran successfully"),
                        Err(error) => eprintln!("Before hook failed to run: {error}")
                    }
                }

                let mut command = Command::new(spin).arg(alias).args(args).spawn().unwrap_or_else(|error| panic!("Failed to run alias: {error}"));
                let result = command.wait().await.unwrap_or_else(|error| panic!("Failed to run alias: {error}"));
                
                if result.success() {
                    eprintln!("Alias ran successfully");
                } else {
                    eprintln!("Alias ran unsuccessfully");
                }

                let mut after_tasks = Vec::new();
                for hook in after_hooks {
                    let name = hook.to_string();
                    let binary = config.binary(&name);
                    let Ok(cwd) = cwd.open_dir(".") else { continue };
                    let Some(manifest) = manifests.remove(&name) else { continue };
                    
                    let task = tokio::task::spawn(async move {
                        let binary = tokio::fs::read(&binary).await.unwrap_or_else(|error| panic!("Failed to read spinlet: {error}"));
                        let context = Context::new(&manifest, cwd);
                        let loader = Loader::new(context);
                        let spinlet = Spinlet::new(loader);
                        match spinlet.run(&binary).await {
                            Ok(success) => if success {
                                eprintln!("[]");
                            } else {
                                eprintln!("[]");
                            },
                            Err(error) => {
                                eprintln!("{error}");
                            }
                        }
                    });

                    after_tasks.push(task);
                }

                for task in after_tasks {
                    match task.await {
                        Ok(_) => eprintln!("After hook ran successfully"),
                        Err(error) => eprintln!("After hook failed to run: {error}")
                    }
                }

                return;
            };

            eprintln!("Running spinlet: {}", command);
            let manifest = config.manifest(&command);
            let manifest = tokio::fs::read_to_string(manifest).await.unwrap_or_else(|error| panic!("Failed to read manifest: {error}"));
            let manifest: Manifest = toml::from_str(&manifest).unwrap_or_else(|error| panic!("Failed to parse manifest: {error}"));

            let context = Context::new(&manifest, cwd);
            let loader = Loader::new(context);
            let spinlet = Spinlet::new(loader);

            let binary = config.binary(&command);
            let binary = tokio::fs::read(&binary).await.unwrap_or_else(|error| panic!("Failed to read spinlet: {error}"));

            let success = spinlet.run(&binary).await.unwrap_or_else(|error| panic!("Failed to run spinlet: {error}"));
            if success {
                eprintln!("Spinlet ran successfully");
            } else {
                eprintln!("Spinlet failed to run");
            }
        });
}