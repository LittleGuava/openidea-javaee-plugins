/// An `asadmin` subcommand the plugin runs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AsadminCmd {
    /// The installed version, without contacting a running server.
    LocalVersion,
}

/// The arguments for `asadmin` (the program itself is resolved by the host).
pub fn asadmin_argv(cmd: &AsadminCmd) -> Vec<String> {
    match cmd {
        AsadminCmd::LocalVersion => vec!["version".into(), "--local".into()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_version_does_not_need_a_running_server() {
        assert_eq!(
            asadmin_argv(&AsadminCmd::LocalVersion),
            ["version", "--local"]
        );
    }
}
