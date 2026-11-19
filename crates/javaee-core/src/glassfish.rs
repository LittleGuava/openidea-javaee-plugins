/// An `asadmin` subcommand the plugin runs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AsadminCmd {
    /// The installed version, without contacting a running server.
    LocalVersion,
}

impl AsadminCmd {
    /// The subcommand behind one of the plugin's command ids.
    pub fn from_command_id(id: &str) -> Option<Self> {
        match id {
            "version" => Some(Self::LocalVersion),
            _ => None,
        }
    }
}

/// The arguments for `asadmin` (the program itself is resolved by the host).
pub fn asadmin_argv(cmd: &AsadminCmd) -> Vec<String> {
    match cmd {
        AsadminCmd::LocalVersion => vec!["version".into(), "--local".into()],
    }
}

/// What a finished `asadmin` run means for the user: its output, or why it failed.
pub fn asadmin_result(status: Option<i32>, stdout: &str, stderr: &str) -> Result<String, String> {
    let Some(code) = status else {
        return Err("asadmin was interrupted before it finished".into());
    };
    if code == 0 {
        return Ok(stdout.trim_end().to_string());
    }
    let detail = [stderr.trim(), stdout.trim()]
        .into_iter()
        .find(|s| !s.is_empty());
    Err(match detail {
        Some(detail) => format!("asadmin exited with code {code}: {detail}"),
        None => format!("asadmin exited with code {code}"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_ids_map_to_subcommands() {
        assert_eq!(
            AsadminCmd::from_command_id("version"),
            Some(AsadminCmd::LocalVersion)
        );
        assert_eq!(AsadminCmd::from_command_id("deploy-everything"), None);
    }

    #[test]
    fn success_shows_the_output_without_trailing_blank_lines() {
        let out = "Version = Eclipse GlassFish 7.0.25 (build 7.0.25)\nCommand version executed successfully.\n\n";
        assert_eq!(
            asadmin_result(Some(0), out, ""),
            Ok("Version = Eclipse GlassFish 7.0.25 (build 7.0.25)\nCommand version executed successfully.".into())
        );
    }

    #[test]
    fn failure_names_the_exit_code_and_stderr() {
        let err = asadmin_result(
            Some(1),
            "",
            "Remote server does not listen for requests on [localhost:4848].\n",
        );
        assert_eq!(
            err,
            Err("asadmin exited with code 1: Remote server does not listen for requests on [localhost:4848].".into())
        );
    }

    #[test]
    fn failure_without_stderr_falls_back_to_stdout() {
        let err = asadmin_result(Some(1), "Command deploy failed.\n", "  \n");
        assert_eq!(
            err,
            Err("asadmin exited with code 1: Command deploy failed.".into())
        );
    }

    #[test]
    fn failure_without_any_output_still_says_what_happened() {
        assert_eq!(
            asadmin_result(Some(2), "", ""),
            Err("asadmin exited with code 2".into())
        );
    }

    #[test]
    fn a_killed_process_is_reported_as_interrupted() {
        assert_eq!(
            asadmin_result(None, "partial", ""),
            Err("asadmin was interrupted before it finished".into())
        );
    }
}
