use javaee_core::glassfish::{AsadminCmd, asadmin_argv, asadmin_result};
use openidea_plugin::{Error, Invocation, Plugin, host, register_plugin};

const ASADMIN: &str = "asadmin";

#[derive(Default)]
pub struct Glassfish;

impl Plugin for Glassfish {
    fn run_command(
        &mut self,
        id: &str,
        _ctx: &Invocation,
        _arg: Option<&str>,
    ) -> Result<Option<String>, Error> {
        let cmd = AsadminCmd::from_command_id(id).ok_or_else(|| format!("Unknown command {id}"))?;
        let out = host::run_process(ASADMIN, &asadmin_argv(&cmd), None)?;
        Ok(Some(asadmin_result(out.status, &out.stdout, &out.stderr)?))
    }
}

register_plugin!(Glassfish);

#[cfg(test)]
mod tests {
    use super::*;
    use openidea_plugin::{ProcessOutput, mock::MockHost};

    const CAPS: &[&str] = &["process:asadmin"];

    fn asadmin_answers(mock: &MockHost, status: Option<i32>, stdout: &str, stderr: &str) {
        let (stdout, stderr) = (stdout.to_string(), stderr.to_string());
        mock.on_process(ASADMIN, move |_, _| {
            Ok(ProcessOutput {
                status,
                stdout: stdout.clone(),
                stderr: stderr.clone(),
            })
        });
    }

    #[test]
    fn version_shows_what_asadmin_prints_without_needing_a_project() {
        let mock = MockHost::install().with_capabilities(CAPS);
        asadmin_answers(&mock, Some(0), "Version = Eclipse GlassFish 7.0.25\n", "");

        let shown = Glassfish
            .run_command("version", &mock.invocation(), None)
            .unwrap();

        assert_eq!(shown.as_deref(), Some("Version = Eclipse GlassFish 7.0.25"));
        let runs = mock.process_runs();
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].args, ["version", "--local"]);
        assert_eq!(runs[0].cwd, None, "runs outside any project");
    }

    #[test]
    fn version_reports_why_asadmin_failed() {
        let mock = MockHost::install().with_capabilities(CAPS);
        asadmin_answers(&mock, Some(1), "Command version failed.\n", "");

        let err = Glassfish
            .run_command("version", &mock.invocation(), None)
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            "asadmin exited with code 1: Command version failed."
        );
    }

    #[test]
    fn unknown_command_is_refused_without_running_asadmin() {
        let mock = MockHost::install().with_capabilities(CAPS);

        let err = Glassfish
            .run_command("format-disk", &mock.invocation(), None)
            .unwrap_err();

        assert!(err.to_string().contains("format-disk"), "{err}");
        assert!(mock.process_runs().is_empty());
    }
}
