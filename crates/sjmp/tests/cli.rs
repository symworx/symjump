use std::fs;
use std::io::Write;
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_sjmp"))
}

fn write_cfg(dir: &std::path::Path) -> std::path::PathBuf {
    let path = dir.join("favorites.toml");
    let mut f = fs::File::create(&path).unwrap();
    writeln!(f, "[[favorites]]\nlabel = \"symworx\"\npath = \"~/worx/symworx\"\nkeys = \"s\"\n\n[[verbs.toolbox]]\nlabel = \"python\"\nname = \"dev-python\"\nkeys = \"p\"\n\n[[verbs.agent]]\nlabel = \"codex\"\nkeys = \"c\"\ncmd = \"codex -C {{path}}\"").unwrap();
    path
}

fn scratch() -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!("sjmp-cli-{}", std::process::id()));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn jump_list_exec_toolbox() {
    let tmp = scratch();
    let cfg = write_cfg(&tmp);
    let c = cfg.to_str().unwrap();
    let list = bin().args(["--config", c, "list"]).env("HOME", "/home/nate").output().unwrap();
    assert!(list.status.success());
    assert!(String::from_utf8_lossy(&list.stdout).contains("s\tsymworx\t"));
    let jump = bin().args(["--config", c, "jump", "s"]).env("HOME", "/home/nate").output().unwrap();
    assert_eq!(String::from_utf8_lossy(&jump.stdout).trim(), "/home/nate/worx/symworx");
    let bad = bin().args(["--config", c, "jump", "zzz"]).env("HOME", "/h").output().unwrap();
    assert!(!bad.status.success());
    let ex = bin().args(["--config", c, "exec", "s", "--cmd", "grok"]).env("HOME", "/home/nate").output().unwrap();
    assert!(String::from_utf8_lossy(&ex.stdout).contains("cd /home/nate/worx/symworx && grok"));
    let tb = bin().args(["--config", c, "toolbox", "enter", "p"]).output().unwrap();
    assert_eq!(String::from_utf8_lossy(&tb.stdout).trim(), "toolbox enter dev-python");
    let va = bin().args(["--config", c, "verb", "agent", "c", "s"]).env("HOME", "/home/nate").output().unwrap();
    assert!(String::from_utf8_lossy(&va.stdout).contains("codex -C /home/nate/worx/symworx"));
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn init_bash() {
    let out = bin().args(["init", "bash"]).output().unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success());
    assert!(s.contains("INSIDE_EMACS"));
    assert!(s.contains(r"\ep"));
    assert!(s.contains(r"\ex"));
}

#[test]
fn pin_then_jump() {
    let tmp = scratch();
    let cfg = tmp.join("favorites.toml");
    let dest = tmp.join("lab");
    fs::create_dir(&dest).unwrap();
    let pin = bin().args(["--config", cfg.to_str().unwrap(), "pin", dest.to_str().unwrap(), "--label", "lab", "--keys", "l"]).output().unwrap();
    assert!(pin.status.success(), "{}", String::from_utf8_lossy(&pin.stderr));
    let out = bin().args(["--config", cfg.to_str().unwrap(), "jump", "l"]).output().unwrap();
    assert!(String::from_utf8_lossy(&out.stdout).contains(dest.to_string_lossy().as_ref()));
    let _ = fs::remove_dir_all(&tmp);
}
