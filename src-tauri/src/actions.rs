use std::path::{Path, PathBuf};
use std::process::Command;

use crate::terminal::hide_console_window;

pub fn launch_executable(path: &str) -> Result<(), String> {
    let cleaned = path.trim().trim_matches('"').to_string();
    let target = Path::new(&cleaned);
    if !target.exists() {
        return Err(format!("Fichier introuvable: {cleaned}"));
    }

    launch_path(&cleaned, target.parent())
}

#[cfg(not(windows))]
fn launch_path(path: &str, working_dir: Option<&Path>) -> Result<(), String> {
    let target = Path::new(path);
    let mut command = Command::new(path);
    if let Some(parent) = working_dir.or_else(|| target.parent()) {
        command.current_dir(parent);
    }
    hide_console_window(&mut command);

    command
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("Lancement impossible: {error}"))
}

#[cfg(windows)]
fn launch_path(path: &str, working_dir: Option<&Path>) -> Result<(), String> {
    shell_execute("open", path, None, working_dir)
        .map_err(|error| format!("Lancement impossible: {error}"))
}

pub fn open_in_explorer(path: &str) -> Result<(), String> {
    let cleaned = path.trim().trim_matches('"').to_string();
    if cleaned.is_empty() {
        return Err("Chemin vide.".to_string());
    }

    let target = PathBuf::from(&cleaned);
    if !target.exists() {
        return Err(format!("Chemin introuvable: {cleaned}"));
    }

    let mut command = Command::new("explorer.exe");
    if target.is_file() {
        command.arg(format!("/select,{}", cleaned));
    } else {
        command.arg(&cleaned);
    }
    hide_console_window(&mut command);

    command
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("Ouverture Explorer impossible: {error}"))
}

pub fn run_uninstall(uninstall_string: &str) -> Result<(), String> {
    let trimmed = uninstall_string.trim();
    if trimmed.is_empty() {
        return Err("Commande de desinstallation vide.".to_string());
    }

    let (program, args) = split_command_line(trimmed);
    if program.is_empty() {
        return Err("Commande de desinstallation invalide.".to_string());
    }

    run_program_detached(&program, &args)
        .map_err(|error| format!("Desinstallation impossible: {error}"))
}

#[cfg(not(windows))]
fn run_program_detached(program: &str, args: &[String]) -> Result<(), String> {
    let mut command = Command::new(&program);
    command.args(args);
    hide_console_window(&mut command);

    command
        .spawn()
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[cfg(windows)]
fn run_program_detached(program: &str, args: &[String]) -> Result<(), String> {
    let parameters = join_shell_args(args);
    shell_execute(
        "open",
        program,
        (!parameters.is_empty()).then_some(parameters.as_str()),
        None,
    )
}

#[cfg(windows)]
fn shell_execute(
    verb: &str,
    file: &str,
    parameters: Option<&str>,
    directory: Option<&Path>,
) -> Result<(), String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows::core::PCWSTR;
    use windows::Win32::UI::Shell::ShellExecuteW;
    use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

    fn wide(value: &str) -> Vec<u16> {
        OsStr::new(value)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }

    let verb = wide(verb);
    let file = wide(file);
    let parameters = parameters.map(wide);
    let directory = directory
        .and_then(|path| path.to_str())
        .filter(|path| !path.trim().is_empty())
        .map(wide);

    let result = unsafe {
        ShellExecuteW(
            None,
            PCWSTR(verb.as_ptr()),
            PCWSTR(file.as_ptr()),
            parameters
                .as_ref()
                .map(|value| PCWSTR(value.as_ptr()))
                .unwrap_or_else(PCWSTR::null),
            directory
                .as_ref()
                .map(|value| PCWSTR(value.as_ptr()))
                .unwrap_or_else(PCWSTR::null),
            SW_SHOWNORMAL,
        )
    };

    let code = result.0 as isize;
    if code > 32 {
        Ok(())
    } else {
        Err(shell_execute_error(code))
    }
}

#[cfg(windows)]
fn shell_execute_error(code: isize) -> String {
    match code {
        0 => "Windows n'a pas assez de ressources pour lancer l'action.".to_string(),
        2 => "Fichier introuvable.".to_string(),
        3 => "Chemin introuvable.".to_string(),
        5 => "Acces refuse ou elevation refusee.".to_string(),
        8 => "Memoire insuffisante.".to_string(),
        26 => "Erreur de partage Windows.".to_string(),
        27 => "Association de fichier incomplete.".to_string(),
        29 => "Erreur DDE Windows.".to_string(),
        30 => "Erreur DDE Windows.".to_string(),
        31 => "Aucune application associee a cette action.".to_string(),
        _ => format!("ShellExecute a echoue (code {code})."),
    }
}

fn split_command_line(input: &str) -> (String, Vec<String>) {
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in input.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
            }
            c if c.is_whitespace() && !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            c => current.push(c),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }

    if tokens.is_empty() {
        (String::new(), Vec::new())
    } else {
        let program = tokens.remove(0);
        (program, tokens)
    }
}

fn join_shell_args(args: &[String]) -> String {
    args.iter()
        .map(|arg| quote_shell_arg(arg))
        .collect::<Vec<_>>()
        .join(" ")
}

fn quote_shell_arg(arg: &str) -> String {
    if arg.chars().any(|ch| ch.is_whitespace() || ch == '"') {
        format!("\"{}\"", arg.replace('"', "\\\""))
    } else {
        arg.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{join_shell_args, split_command_line};

    #[test]
    fn splits_simple_command() {
        let (program, args) = split_command_line("C:\\App\\unins000.exe /SILENT");
        assert_eq!(program, "C:\\App\\unins000.exe");
        assert_eq!(args, vec!["/SILENT".to_string()]);
    }

    #[test]
    fn splits_quoted_program_with_args() {
        let (program, args) =
            split_command_line("\"C:\\Program Files\\App\\unins000.exe\" /SILENT /NORESTART");
        assert_eq!(program, "C:\\Program Files\\App\\unins000.exe");
        assert_eq!(args, vec!["/SILENT".to_string(), "/NORESTART".to_string()]);
    }

    #[test]
    fn handles_msiexec() {
        let (program, args) =
            split_command_line("MsiExec.exe /X{00000000-0000-0000-0000-000000000000}");
        assert_eq!(program, "MsiExec.exe");
        assert_eq!(
            args,
            vec!["/X{00000000-0000-0000-0000-000000000000}".to_string()]
        );
    }

    #[test]
    fn returns_empty_for_blank_input() {
        let (program, args) = split_command_line("   ");
        assert!(program.is_empty());
        assert!(args.is_empty());
    }

    #[test]
    fn joins_shell_args_with_quotes() {
        let args = vec!["/S".to_string(), "C:\\Program Files\\App".to_string()];
        assert_eq!(join_shell_args(&args), "/S \"C:\\Program Files\\App\"");
    }
}
