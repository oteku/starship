use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::nodejs::NodejsConfig;
use crate::utils;

/// Creates a module with the current Node.js version
///
/// Will display the Node.js version if any of the following criteria are met:
///     - Current directory contains a `.js`, `.mjs` or `.cjs` file
///     - Current directory contains a `.ts` file
///     - Current directory contains a `package.json` or `.node-version` file
///     - Current directory contains a `node_modules` directory
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_js_project = context
        .try_begin_scan()?
        .set_files(&["package.json", ".node-version"])
        .set_extensions(&["js", "mjs", "cjs", "ts"])
        .set_folders(&["node_modules"])
        .is_match();

    let is_esy_project = context
        .try_begin_scan()?
        .set_folders(&["esy.lock"])
        .is_match();

    if !is_js_project || is_esy_project {
        return None;
    }

    let node_version = utils::exec_cmd("node", &["--version"])?.stdout;

    let mut module = context.new_module("nodejs");
    let config: NodejsConfig = NodejsConfig::try_load(module.config);

    module.set_style(config.style);

    let formatted_version = node_version.trim();
    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(formatted_version));

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::{self, File};
    use std::io;

    #[test]
    fn folder_without_node_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("nodejs", dir.path(), None);
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_package_json() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;

        let actual = render_module("nodejs", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_package_json_and_esy_lock() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;
        let esy_lock = dir.path().join("esy.lock");
        fs::create_dir_all(&esy_lock)?;

        let actual = render_module("nodejs", dir.path(), None);
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_node_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".node-version"))?.sync_all()?;

        let actual = render_module("nodejs", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_js_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.js"))?.sync_all()?;

        let actual = render_module("nodejs", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mjs_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.mjs"))?.sync_all()?;

        let actual = render_module("nodejs", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cjs_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.cjs"))?.sync_all()?;

        let actual = render_module("nodejs", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_ts_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.ts"))?.sync_all()?;

        let actual = render_module("nodejs", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_node_modules() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let node_modules = dir.path().join("node_modules");
        fs::create_dir_all(&node_modules)?;

        let actual = render_module("nodejs", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
