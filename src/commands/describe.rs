use clap::Command;
use serde_json::{json, Value};

pub fn describe_commands(root: &Command, path: &[String]) -> Value {
    if path.is_empty() {
        return describe_full(root);
    }

    // Walk the path to find the target command
    let mut cmd = root;
    for part in path {
        match cmd.get_subcommands().find(|s| s.get_name() == part) {
            Some(sub) => cmd = sub,
            None => {
                return json!({
                    "error": format!("Unknown command: {}", path.join(" "))
                });
            }
        }
    }

    describe_command(cmd)
}

fn describe_full(root: &Command) -> Value {
    let global_flags: Vec<Value> = root
        .get_arguments()
        .filter(|a| {
            a.get_id() != "help"
                && a.get_id() != "version"
                && a.get_long().is_some()
                && a.is_global_set()
        })
        .map(describe_arg)
        .collect();

    let commands: serde_json::Map<String, Value> = root
        .get_subcommands()
        .filter(|s| s.get_name() != "help")
        .map(|sub| (sub.get_name().to_string(), describe_command(sub)))
        .collect();

    json!({
        "name": root.get_name(),
        "description": root.get_about().map(|s| s.to_string()),
        "global_flags": global_flags,
        "commands": commands,
    })
}

fn describe_command(cmd: &Command) -> Value {
    let subs: Vec<&Command> = cmd
        .get_subcommands()
        .filter(|s| s.get_name() != "help")
        .collect();

    if subs.is_empty() {
        // Leaf command
        let args: Vec<Value> = cmd
            .get_arguments()
            .filter(|a| a.get_id() != "help" && a.get_id() != "version")
            .filter(|a| !a.is_global_set())
            .map(describe_arg)
            .collect();

        json!({
            "description": cmd.get_about().map(|s| s.to_string()),
            "args": args,
        })
    } else {
        // Group command
        let subcommands: serde_json::Map<String, Value> = subs
            .into_iter()
            .map(|sub| (sub.get_name().to_string(), describe_command(sub)))
            .collect();

        json!({
            "description": cmd.get_about().map(|s| s.to_string()),
            "subcommands": subcommands,
        })
    }
}

fn describe_arg(arg: &clap::Arg) -> Value {
    let name = if let Some(long) = arg.get_long() {
        format!("--{long}")
    } else if let Some(short) = arg.get_short() {
        format!("-{short}")
    } else {
        arg.get_id().to_string()
    };

    let type_name = if arg.get_num_args().is_some_and(|r| r.max_values() == 0) {
        "bool"
    } else {
        "string"
    };

    let required = arg.is_required_set();
    let default_val = arg
        .get_default_values()
        .first()
        .map(|v| v.to_string_lossy().to_string());

    let env_var: Option<String> = arg.get_env().map(|e| e.to_string_lossy().to_string());

    let mut obj = json!({
        "name": name,
        "type": type_name,
        "required": required,
        "description": arg.get_help().map(|s| s.to_string()),
    });

    if let Some(d) = default_val {
        obj["default"] = json!(d);
    }
    if let Some(e) = env_var {
        obj["env"] = json!(e);
    }

    obj
}
