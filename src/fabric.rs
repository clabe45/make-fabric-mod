use std::path::Path;

use crate::{
    code::{language::Language, refactor},
    file, git,
};

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<git::Error> for Error {
    fn from(error: git::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

impl From<refactor::Error> for Error {
    fn from(error: refactor::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

impl From<file::Error> for Error {
    fn from(error: file::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

fn validate_version(version: &str) -> Result<(), Error> {
    if !version.chars().all(|c| c.is_digit(10) || c == '.') {
        return Err(Error {
            message: format!("Invalid version: {}", version),
        });
    }

    let parts = version.split('.');
    if parts.count() != 2 {
        return Err(Error {
            message: format!("Invalid version: {}. Expected 2 parts (e.g. 1.19)", version),
        });
    }

    return Ok(());
}

fn update_mod_config(path: &Path, mod_id: &str, main_class: &str, name: &str) -> Result<(), Error> {
    let config_path = path.join("src/main/resources/fabric.mod.json");
    let mut config: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&config_path)?)?;

    config["id"] = serde_json::Value::String(mod_id.to_string());
    config["name"] = serde_json::Value::String(name.to_string());
    config["description"] = serde_json::Value::String("".to_string());
    config["icon"] = serde_json::Value::String(format!("assets/{}/icon.png", mod_id));
    config["entrypoints"]["main"][0] = serde_json::Value::String(main_class.to_string());
    config["mixins"][0] = serde_json::Value::String(format!("{}.mixins.json", mod_id));

    std::fs::write(config_path, serde_json::to_string_pretty(&config)?)?;
    Ok(())
}

fn update_mixin_config(path: &Path, mod_id: &str, mixin_package: &str) -> Result<(), Error> {
    let config_path = path.join(format!("src/main/resources/{}.mixins.json", mod_id));
    let mut config: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&config_path)?)?;
    config["package"] = serde_json::Value::String(mixin_package.to_string());
    std::fs::write(config_path, serde_json::to_string_pretty(&config)?)?;
    Ok(())
}

fn update_gradle_properties(path: &Path, group: &str, base_name: &str) -> Result<(), Error> {
    let config_path = path.join("gradle.properties");
    let mut config = std::fs::read_to_string(&config_path)?;
    config = config.replace("com.example", group);
    config = config.replace("fabric-example-mod", base_name);
    std::fs::write(config_path, config)?;
    Ok(())
}

fn refactor_module(path: &Path, language: &Language, main_class: &str) -> Result<(), Error> {
    // Rename the package
    let old_package = "net.fabricmc.example";
    let new_package = main_class[..main_class.rfind('.').unwrap()].to_string();
    refactor::rename_package(path, language, &old_package, &new_package)?;

    // Rename the main class (if contained in this module)
    let main_class_exists = path
        .join("src/main")
        .join(language.to_string())
        .join(new_package.replace('.', "/"))
        .join(format!("ExampleMod.{}", language.extension()))
        .exists();

    if main_class_exists {
        let old_class = format!("{}.ExampleMod", &new_package);
        let new_class = main_class;
        refactor::rename_class(path, language, &old_class, &new_class)?;
    }

    Ok(())
}

pub fn create_mod(
    path: &Path,
    mod_id: &str,
    minecraft_version: &str,
    language: &Language,
    main_class: &str,
    name: &str,
) -> Result<(), Error> {
    validate_version(minecraft_version)?;

    // Clone the Kotlin example mod
    let template_url = match language {
        Language::Kotlin => "https://github.com/clabe45/fabric-example-mod-kotlin",
        Language::Java => "https://github.com/FabricMC/fabric-example-mod",
    };
    println!("Cloning {}...", template_url);
    let global = git::Context::new(&None)?;
    global.git(&["clone", "--depth", "1", "--branch", minecraft_version, template_url, path.to_str().unwrap()])
        .map_err(|e| match e.kind() {
            // If git is installed but the command failed, it's probably because
            // the version branch doesn't exist
            git::ErrorKind::GitFailed => Error {
                message: format!("Unsupported Minecraft version: {}", minecraft_version),
            },
            _ => e.into(),
        })?;

    println!("Re-initializing git repository...");

    // Remove the .git directory
    let git_dir = path.join(".git");
    std::fs::remove_dir_all(git_dir)?;

    // Re-initialize the git repository
    let repo = git::Context::new(&Some(path))?;
    repo.git(&["init"])?;

    // Refactor each module. If --kotlin is specified, refactor both the Java
    // and Kotlin modules. The mixins are located in the Java module, and
    // everything else is located in the Kotlin module. If --kotlin is not
    // specified, only refactor the Java module.
    let languages = match language {
        Language::Java => vec![Language::Java],
        Language::Kotlin => vec![Language::Kotlin, Language::Java],
    };
    for language in languages {
        println!("Refactoring {} module...", language.to_string());
        refactor_module(path, &language, main_class)?;

        // Replace all string literals equal to "modid" with the mod ID
        let module_root_path = path.join("src/main").join(language.to_string());
        file::recursive_replace(&module_root_path, "\"modid\"", &format!("\"{}\"", mod_id))?;
    }

    // Move the assets directory to the correct location
    let old_assets_path = path.join("src/main/resources/assets/modid");
    let new_assets_path = path.join(format!("src/main/resources/assets/{}", mod_id));
    std::fs::rename(old_assets_path, new_assets_path)?;

    println!("Updating config files...");

    // Update the mixins config
    std::fs::rename(
        path.join("src/main/resources/modid.mixins.json"),
        path.join(format!("src/main/resources/{}.mixins.json", mod_id)),
    )?;
    let mixin_package = format!(
        "{}.mixin",
        main_class[..main_class.rfind('.').unwrap()].to_string()
    );
    update_mixin_config(path, mod_id, &mixin_package)?;

    // Update the mod config
    update_mod_config(path, mod_id, main_class, name)?;

    // Update gradle.properties
    let package = main_class[..main_class.rfind('.').unwrap()].to_string();
    let group = &package[..package.rfind('.').unwrap()].to_string();
    let base_name = &package[package.rfind('.').unwrap() + 1..].to_string();
    update_gradle_properties(path, &group, &base_name)?;

    println!("Done!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{code::language::Language, fabric};

    #[test]
    fn test_validate_version() {
        assert!(fabric::validate_version("1.17").is_ok());
        assert!(fabric::validate_version("1").is_err());
        assert!(fabric::validate_version("1.17.1").is_err());
    }

    #[rstest]
    #[case(Language::Java)]
    #[case(Language::Kotlin)]
    fn test_unsupported_version(#[case] language: Language) {
        assert!(fabric::create_mod(
            &std::path::Path::new("test"),
            "test",
            "1.16",
            &language,
            "test",
            "test"
        )
        .is_err());
    }

    #[rstest]
    #[case(Language::Java, "1.18")]
    #[case(Language::Kotlin, "1.18")]
    #[case(Language::Java, "1.19")]
    #[case(Language::Kotlin, "1.19")]
    fn test_create_mod_creates_git_repo(#[case] language: Language, #[case] minecraft_version: &str) {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test_create_mod_creates_git_repo");
        fabric::create_mod(
            &path,
            "example-mod",
            minecraft_version,
            &language,
            "net.fabricmc.example.ExampleMod",
            "Example Mod",
        )
        .unwrap();

        let git_dir = path.join(".git");
        assert!(git_dir.exists());
    }

    #[rstest]
    #[case(Language::Java, "1.18")]
    #[case(Language::Kotlin, "1.18")]
    #[case(Language::Java, "1.19")]
    #[case(Language::Kotlin, "1.19")]
    fn test_create_mod_moves_entrypoint(#[case] language: Language, #[case] minecraft_version: &str) {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test_create_mod_moves_entrypoint");
        fabric::create_mod(
            &path,
            "example-mod2",
            minecraft_version,
            &language,
            "net.fabricmc.example2.ExampleMod2",
            "Example Mod 2",
        )
        .unwrap();

        let entrypoint = path
            .join("src/main")
            .join(language.to_string())
            .join("net/fabricmc/example2/ExampleMod2.".to_string() + language.extension());

        assert!(entrypoint.exists());
    }

    #[rstest]
    #[case(Language::Java, "1.18")]
    #[case(Language::Kotlin, "1.18")]
    #[case(Language::Java, "1.19")]
    #[case(Language::Kotlin, "1.19")]
    fn test_create_mod_moves_assets(#[case] language: Language, #[case] minecraft_version: &str) {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test_create_mod_moves_assets");
        fabric::create_mod(
            &path,
            "example-mod2",
            minecraft_version,
            &language,
            "net.fabricmc.example3.ExampleMod2",
            "Example Mod 2",
        )
        .unwrap();

        let assets = path.join("src/main/resources/assets/example-mod2");
        assert!(assets.exists());
    }

    #[rstest]
    #[case(Language::Java, "1.18")]
    #[case(Language::Kotlin, "1.18")]
    #[case(Language::Java, "1.19")]
    #[case(Language::Kotlin, "1.19")]
    fn test_create_mod_renames_mixin_config(#[case] language: Language, #[case] minecraft_version: &str) {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test_create_mod_renames_mixin_config");
        fabric::create_mod(
            &path,
            "example-mod2",
            minecraft_version,
            &language,
            "net.fabricmc.example3.ExampleMod2",
            "Example Mod 2",
        )
        .unwrap();

        let mixin_config = path.join("src/main/resources/example-mod2.mixins.json");
        assert!(mixin_config.exists());
    }

    #[rstest]
    #[case(Language::Java, "1.18")]
    #[case(Language::Kotlin, "1.18")]
    #[case(Language::Java, "1.19")]
    #[case(Language::Kotlin, "1.19")]
    fn test_create_mod_updates_mixin_config(#[case] language: Language, #[case] minecraft_version: &str) {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test_create_mod_updates_mixin_config");
        fabric::create_mod(
            &path,
            "example-mod2",
            minecraft_version,
            &language,
            "net.fabricmc.example2.ExampleMod2",
            "Example Mod 2",
        )
        .unwrap();

        let mixin_config = path.join("src/main/resources/example-mod2.mixins.json");
        let contents = std::fs::read_to_string(mixin_config).unwrap();
        let config: serde_json::Value = serde_json::from_str(&contents).unwrap();
        assert_eq!(
            config["package"],
            serde_json::Value::String("net.fabricmc.example2.mixin".to_string())
        );
    }

    #[rstest]
    #[case(Language::Java, "1.18")]
    #[case(Language::Kotlin, "1.18")]
    #[case(Language::Java, "1.19")]
    #[case(Language::Kotlin, "1.19")]
    fn test_create_mod_updates_mod_config(#[case] language: Language, #[case] minecraft_version: &str) {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test_create_mod_updates_mod_id");
        fabric::create_mod(
            &path,
            "example-mod2",
            minecraft_version,
            &language,
            "net.fabricmc.example2.ExampleMod2",
            "Example Mod 2",
        )
        .unwrap();

        let mod_json = path.join("src/main/resources/fabric.mod.json");
        let contents = std::fs::read_to_string(mod_json).unwrap();
        let config: serde_json::Value = serde_json::from_str(&contents).unwrap();
        assert_eq!(config["id"], "example-mod2");
        assert_eq!(config["name"], "Example Mod 2");
        assert_eq!(config["description"], "");
        assert_eq!(config["icon"], "assets/example-mod2/icon.png".to_string());
        assert_eq!(
            config["entrypoints"]["main"][0],
            "net.fabricmc.example2.ExampleMod2"
        );
        assert_eq!(config["mixins"][0], "example-mod2.mixins.json");
    }

    #[rstest]
    #[case(Language::Java, "1.18")]
    #[case(Language::Kotlin, "1.18")]
    #[case(Language::Java, "1.19")]
    #[case(Language::Kotlin, "1.19")]
    fn test_create_mod_updates_gradle_properties(#[case] language: Language, #[case] minecraft_version: &str) {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir
            .path()
            .join("test_create_mod_updates_gradle_properties");
        fabric::create_mod(
            &path,
            "example-mod2",
            minecraft_version,
            &language,
            "net.fabricmc.example2.ExampleMod2",
            "Example Mod 2",
        )
        .unwrap();

        let gradle_properties = path.join("gradle.properties");
        let contents = std::fs::read_to_string(gradle_properties).unwrap();
        assert!(contents.contains("net.fabricmc"));
        assert!(contents.contains("example2"));
    }
}
