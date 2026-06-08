//! CLI (Command Line Interface) components.
//!
//! Part of the Interface (Presentation Layer).
//! For command-line argument parsing, help text, version output, and simple
//! text-based interactions without a full TUI.
//!
//! **Taxonomy Classification**: Interface (CLI).

pub mod scaffold;
pub mod doctor;

pub use scaffold::{
    CliCommand, CliCommandDef, CliOptionDef, CliParser, ParsedArgs, ScaffoldAction,
    parse_cli_args, is_help_arg, is_version_arg, is_doctor_arg, is_install_arg, is_debug_arg,
    is_no_color_arg, is_json_arg, is_high_contrast_arg, is_accessible_arg, is_tui_arg, is_cli_arg,
};
pub use doctor::{run_doctor, run_doctor_with_custom};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let args1 = vec!["--version".to_string()];
        let args2 = vec!["-v".to_string()];
        let args3 = vec!["version".to_string()];

        assert!(is_version_arg(&args1[0]));
        assert_eq!(parse_cli_args(&args1), CliCommand::Version);
        assert_eq!(parse_cli_args(&args2), CliCommand::Version);
        assert_eq!(parse_cli_args(&args3), CliCommand::Version);
    }

    #[test]
    fn test_parse_custom_command() {
        let parser = CliParser::new("test", "desc")
            .command("mycommand", "run my command");
        let args = vec!["test".to_string(), "mycommand".to_string()];
        let parsed = parser.parse(&args).unwrap();
        assert_eq!(parsed.command, Some("mycommand".to_string()));
    }

    #[test]
    fn test_parse_options() {
        let parser = CliParser::new("test", "desc")
            .option('p', "port", "port number", true)
            .option('d', "debug", "debug flag", false);
        let args = vec![
            "test".to_string(),
            "--port".to_string(),
            "8080".to_string(),
            "-d".to_string(),
        ];
        let parsed = parser.parse(&args).unwrap();
        assert_eq!(parsed.options.get("port"), Some(&"8080".to_string()));
        assert_eq!(parsed.flags.get("debug"), Some(&true));
    }

    #[test]
    fn test_doctor() {
        run_doctor(); // Should not panic.
    }

    #[test]
    fn test_determine_scaffold_action() {
        let parser = CliParser::new("testapp", "a test application")
            .logo("LOGO")
            .command("run", "Run application");

        // Help flag
        let args = vec!["testapp".to_string(), "--help".to_string()];
        assert_eq!(parser.determine_scaffold_action(&args), ScaffoldAction::PrintHelp);

        // Help command
        let args = vec!["testapp".to_string(), "help".to_string()];
        assert_eq!(parser.determine_scaffold_action(&args), ScaffoldAction::PrintHelp);

        // Version flag
        let args = vec!["testapp".to_string(), "-v".to_string()];
        assert_eq!(parser.determine_scaffold_action(&args), ScaffoldAction::PrintVersion);

        // Doctor command
        let args = vec!["testapp".to_string(), "doctor".to_string()];
        assert_eq!(parser.determine_scaffold_action(&args), ScaffoldAction::RunDoctor);

        // Continue command
        let args = vec!["testapp".to_string(), "run".to_string()];
        let action = parser.determine_scaffold_action(&args);
        match action {
            ScaffoldAction::Continue(parsed) => {
                assert_eq!(parsed.command, Some("run".to_string()));
            }
            _ => panic!("Expected ScaffoldAction::Continue"),
        }
    }
}