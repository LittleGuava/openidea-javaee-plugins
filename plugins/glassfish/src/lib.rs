use javaee_core::glassfish::{AsadminCmd, asadmin_argv};
use openidea_plugin::{Error, Invocation, Plugin, register_plugin};

const ASADMIN: &str = "asadmin";

#[derive(Default)]
pub struct Glassfish;

impl Plugin for Glassfish {
    fn run_command(
        &mut self,
        id: &str,
        ctx: &Invocation,
        _arg: Option<&str>,
    ) -> Result<Option<String>, Error> {
        let project = ctx.project().ok_or("Open a project first")?;
        let cmd = match id {
            "version" => AsadminCmd::LocalVersion,
            other => return Err(format!("Unknown command {other}").into()),
        };
        let out = project.run_process(ASADMIN, &asadmin_argv(&cmd), None)?;
        if out.status != Some(0) {
            return Err(format!("asadmin failed: {}", out.stderr.trim()).into());
        }
        Ok(Some(out.stdout.trim().to_string()))
    }
}

register_plugin!(Glassfish);

#[cfg(test)]
mod tests {
    use super::*;
    use openidea_plugin::{ProcessOutput, mock::MockHost};

    const CAPS: &[&str] = &["project.read", "process:asadmin"];

    #[test]
    fn version_shows_what_asadmin_prints() {
        let mock = MockHost::install().with_capabilities(CAPS);
        mock.on_process("asadmin", |_, _| {
            Ok(ProcessOutput {
                status: Some(0),
                stdout: "Version = GlassFish 7.0.25\n".into(),
                stderr: String::new(),
            })
        });

        let shown = Glassfish
            .run_command("version", &mock.invocation(), None)
            .unwrap();

        assert_eq!(shown.as_deref(), Some("Version = GlassFish 7.0.25"));
        let runs = mock.process_runs();
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].args, ["version", "--local"]);
    }

    #[test]
    fn version_reports_asadmin_failure() {
        let mock = MockHost::install().with_capabilities(CAPS);
        mock.on_process("asadmin", |_, _| {
            Ok(ProcessOutput {
                status: Some(1),
                stdout: String::new(),
                stderr: "Command version failed.\n".into(),
            })
        });

        let err = Glassfish
            .run_command("version", &mock.invocation(), None)
            .unwrap_err();

        assert!(err.to_string().contains("Command version failed."), "{err}");
    }
}
