use ansi_term::Color;
use dirs_next::home_dir;
use git2::Repository;
use std::fs;
use std::io;
#[cfg(not(target_os = "windows"))]
use std::os::unix::fs::symlink;
#[cfg(target_os = "windows")]
use std::os::windows::fs::symlink_dir as symlink;
use std::path::Path;
use tempfile::TempDir;

use crate::common::{self, TestCommand};

#[test]
fn home_directory() -> io::Result<()> {
    let output = common::render_module("directory")
        .arg("--path=~")
        .use_config(toml::toml! { // Necessary if homedir is a git repo
            [directory]
            truncate_to_repo = false
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("~"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn substituted_truncated_path() -> io::Result<()> {
    let output = common::render_module("directory")
        .arg("--path=/some/long/network/path/workspace/a/b/c/dev")
        .use_config(toml::toml! {
            [directory]
            truncation_length = 4
            [directory.substitutions]
            "/some/long/network/path" = "/some/net"
            "a/b/c" = "d"
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("net/workspace/d/dev"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn strange_substitution() -> io::Result<()> {
    let strange_sub = "/\\/;,!";
    let output = common::render_module("directory")
        .arg("--path=/foo/bar/regular/path")
        .use_config(toml::toml! {
            [directory]
            truncation_length = 0
            fish_style_pwd_dir_length = 2 // Overridden by substitutions
            [directory.substitutions]
            "regular" = strange_sub
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint(format!("/foo/bar/{}/path", strange_sub))
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn directory_in_home() -> io::Result<()> {
    let dir = home_dir().unwrap().join("starship/engine");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("directory")
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("~/starship/engine"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_in_home() -> io::Result<()> {
    let dir = home_dir().unwrap().join("starship/engine/schematics");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("directory")
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("starship/engine/schematics")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn fish_directory_in_home() -> io::Result<()> {
    let dir = home_dir().unwrap().join("starship/engine/schematics");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            truncation_length = 1
            fish_style_pwd_dir_length = 2
        })
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("~/st/en/schematics"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn root_directory() -> io::Result<()> {
    let output = common::render_module("directory")
        .arg("--path=/")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("/"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn test_prefix() -> io::Result<()> {
    let output = common::render_module("directory")
        .arg("--path=/")
        .use_config(toml::toml! {
            [directory]
            prefix = "sample "
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("sample {} ", Color::Cyan.bold().paint("/"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[cfg(not(target_os = "windows"))]
fn directory_in_root() -> io::Result<()> {
    let output = common::render_module("directory")
        .arg("--path=/etc")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("/etc"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
fn directory_in_root() -> io::Result<()> {
    let output = common::render_module("directory")
        .arg("--path=C:\\")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("C:"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_in_root() -> io::Result<()> {
    let dir = Path::new("/tmp/starship/thrusters/rocket");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("directory")
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("starship/thrusters/rocket")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_config_large() -> io::Result<()> {
    let dir = Path::new("/tmp/starship/thrusters/rocket");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            truncation_length = 100
        })
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("/tmp/starship/thrusters/rocket")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn fish_style_directory_config_large() -> io::Result<()> {
    let dir = Path::new("/tmp/starship/thrusters/rocket");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            truncation_length = 1
            fish_style_pwd_dir_length = 100
        })
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("/tmp/starship/thrusters/rocket")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_config_small() -> io::Result<()> {
    let dir = Path::new("/tmp/starship/thrusters/rocket");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            truncation_length = 2
        })
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("thrusters/rocket"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn fish_directory_config_small() -> io::Result<()> {
    let dir = Path::new("/tmp/starship/thrusters/rocket");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            truncation_length = 2
            fish_style_pwd_dir_length = 1
        })
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("/t/s/thrusters/rocket"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn git_repo_root() -> io::Result<()> {
    // TODO: Investigate why git repo related tests fail when the tempdir is within /tmp/...
    // Temporarily making the tempdir within $HOME
    // #[ignore] can be removed after this TODO is addressed
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    fs::create_dir(&repo_dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("directory")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("rocket-controls"));
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn directory_in_git_repo() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let dir = repo_dir.join("src");
    fs::create_dir_all(&dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("directory")
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("rocket-controls/src"));
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn truncated_directory_in_git_repo() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let dir = repo_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("directory")
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("src/meters/fuel-gauge"));
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn directory_in_git_repo_truncate_to_repo_false() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("above-repo").join("rocket-controls");
    let dir = repo_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            // Don't truncate the path at all.
            truncation_length = 5
            truncate_to_repo = false
        })
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint("above-repo/rocket-controls/src/meters/fuel-gauge")
    );
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn fish_path_directory_in_git_repo_truncate_to_repo_false() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("above-repo").join("rocket-controls");
    let dir = repo_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            // Don't truncate the path at all.
            truncation_length = 5
            truncate_to_repo = false
            fish_style_pwd_dir_length = 1
        })
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint("~/.t/above-repo/rocket-controls/src/meters/fuel-gauge")
    );
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn fish_path_directory_in_git_repo_truncate_to_repo_true() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("above-repo").join("rocket-controls");
    let dir = repo_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            // `truncate_to_repo = true` should display the truncated path
            truncation_length = 5
            truncate_to_repo = true
            fish_style_pwd_dir_length = 1
        })
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint("~/.t/a/rocket-controls/src/meters/fuel-gauge")
    );
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn directory_in_git_repo_truncate_to_repo_true() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("above-repo").join("rocket-controls");
    let dir = repo_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            // `truncate_to_repo = true` should display the truncated path
            truncation_length = 5
            truncate_to_repo = true
        })
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint("rocket-controls/src/meters/fuel-gauge")
    );
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
#[cfg(not(target_os = "windows"))]
fn git_repo_in_home_directory_truncate_to_repo_true() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let dir = tmp_dir.path().join("src/meters/fuel-gauge");
    fs::create_dir_all(&dir)?;
    Repository::init(&tmp_dir).unwrap();

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            // `truncate_to_repo = true` should attmpt to display the truncated path
            truncate_to_repo = true
            truncation_length = 5
        })
        // Set home directory to the temp repository
        .env("HOME", tmp_dir.path())
        .arg("--path")
        .arg(dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("~/src/meters/fuel-gauge")
    );
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn symlinked_git_repo_root() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let symlink_dir = tmp_dir.path().join("rocket-controls-symlink");
    fs::create_dir(&repo_dir)?;
    Repository::init(&repo_dir).unwrap();
    symlink(&repo_dir, &symlink_dir)?;

    let output = common::render_module("directory")
        .arg("--path")
        .arg(symlink_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("rocket-controls-symlink")
    );
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn directory_in_symlinked_git_repo() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let src_dir = repo_dir.join("src");
    let symlink_dir = tmp_dir.path().join("rocket-controls-symlink");
    let symlink_src_dir = symlink_dir.join("src");
    fs::create_dir_all(&src_dir)?;
    Repository::init(&repo_dir).unwrap();
    symlink(&repo_dir, &symlink_dir)?;

    let output = common::render_module("directory")
        .arg("--path")
        .arg(symlink_src_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("rocket-controls-symlink/src")
    );
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn truncated_directory_in_symlinked_git_repo() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let src_dir = repo_dir.join("src/meters/fuel-gauge");
    let symlink_dir = tmp_dir.path().join("rocket-controls-symlink");
    let symlink_src_dir = symlink_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&src_dir)?;
    Repository::init(&repo_dir).unwrap();
    symlink(&repo_dir, &symlink_dir)?;

    let output = common::render_module("directory")
        .arg("--path")
        .arg(symlink_src_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("src/meters/fuel-gauge"));
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn directory_in_symlinked_git_repo_truncate_to_repo_false() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("above-repo").join("rocket-controls");
    let src_dir = repo_dir.join("src/meters/fuel-gauge");
    let symlink_dir = tmp_dir
        .path()
        .join("above-repo")
        .join("rocket-controls-symlink");
    let symlink_src_dir = symlink_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&src_dir)?;
    Repository::init(&repo_dir).unwrap();
    symlink(&repo_dir, &symlink_dir)?;

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            // Don't truncate the path at all.
            truncation_length = 5
            truncate_to_repo = false
        })
        .arg("--path")
        .arg(symlink_src_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint("above-repo/rocket-controls-symlink/src/meters/fuel-gauge")
    );
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn fish_path_directory_in_symlinked_git_repo_truncate_to_repo_false() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("above-repo").join("rocket-controls");
    let src_dir = repo_dir.join("src/meters/fuel-gauge");
    let symlink_dir = tmp_dir
        .path()
        .join("above-repo")
        .join("rocket-controls-symlink");
    let symlink_src_dir = symlink_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&src_dir)?;
    Repository::init(&repo_dir).unwrap();
    symlink(&repo_dir, &symlink_dir)?;

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            // Don't truncate the path at all.
            truncation_length = 5
            truncate_to_repo = false
            fish_style_pwd_dir_length = 1
        })
        .arg("--path")
        .arg(symlink_src_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint("~/.t/above-repo/rocket-controls-symlink/src/meters/fuel-gauge")
    );
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn fish_path_directory_in_symlinked_git_repo_truncate_to_repo_true() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("above-repo").join("rocket-controls");
    let src_dir = repo_dir.join("src/meters/fuel-gauge");
    let symlink_dir = tmp_dir
        .path()
        .join("above-repo")
        .join("rocket-controls-symlink");
    let symlink_src_dir = symlink_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&src_dir)?;
    Repository::init(&repo_dir).unwrap();
    symlink(&repo_dir, &symlink_dir)?;

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            // `truncate_to_repo = true` should display the truncated path
            truncation_length = 5
            truncate_to_repo = true
            fish_style_pwd_dir_length = 1
        })
        .arg("--path")
        .arg(symlink_src_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint("~/.t/a/rocket-controls-symlink/src/meters/fuel-gauge")
    );
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn directory_in_symlinked_git_repo_truncate_to_repo_true() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("above-repo").join("rocket-controls");
    let src_dir = repo_dir.join("src/meters/fuel-gauge");
    let symlink_dir = tmp_dir
        .path()
        .join("above-repo")
        .join("rocket-controls-symlink");
    let symlink_src_dir = symlink_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&src_dir)?;
    Repository::init(&repo_dir).unwrap();
    symlink(&repo_dir, &symlink_dir)?;

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            // `truncate_to_repo = true` should display the truncated path
            truncation_length = 5
            truncate_to_repo = true
        })
        .arg("--path")
        .arg(symlink_src_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint("rocket-controls-symlink/src/meters/fuel-gauge")
    );
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
fn symlinked_directory_in_git_repo() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let dir = repo_dir.join("src");
    fs::create_dir_all(&dir)?;
    Repository::init(&repo_dir).unwrap();
    symlink(&dir, repo_dir.join("src/loop"))?;

    let output = common::render_module("directory")
        .use_config(toml::toml! {
            [directory]
            // `truncate_to_repo = true` should display the truncated path
            truncation_length = 5
            truncate_to_repo = true
        })
        .arg("--path")
        .arg(repo_dir.join("src/loop/loop"))
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("rocket-controls/src/loop/loop")
    );
    assert_eq!(expected, actual);
    tmp_dir.close()
}

#[test]
#[ignore]
#[cfg(not(target_os = "windows"))]
fn symlinked_subdirectory_git_repo_out_of_tree() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("above-repo").join("rocket-controls");
    let src_dir = repo_dir.join("src/meters/fuel-gauge");
    let symlink_dir = tmp_dir.path().join("fuel-gauge");
    fs::create_dir_all(&src_dir)?;
    Repository::init(&repo_dir).unwrap();
    symlink(&src_dir, &symlink_dir)?;

    let output = common::render_module("directory")
        // Set home directory to the temp repository
        .env("HOME", tmp_dir.path())
        .arg("--path")
        .arg(symlink_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("~/fuel-gauge"));
    assert_eq!(expected, actual);
    tmp_dir.close()
}
