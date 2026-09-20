use anyhow::{Result, bail};
use iepm_core::{
    ExecutionReadiness, Installer, InstallerArgument, InstallerLauncher, LockedPackage, Lockfile,
    WeiDUComponent,
};
use std::collections::{BTreeMap, BTreeSet};

/// Render the first A5 deliverable: a non-mutating, auditable execution plan.
/// It never fetches, materializes, starts a process, or touches a game tree.
pub fn render_plan(lockfile: &Lockfile) -> Result<String> {
    if lockfile.schema != 3 {
        bail!(
            "lockfile schema {} cannot enter A5 planning; re-resolve it to schema 3",
            lockfile.schema
        );
    }
    if lockfile.execution_readiness != ExecutionReadiness::Executable {
        let reasons = if lockfile.blocking_reasons.is_empty() {
            "the lockfile did not provide causal blocking reasons".to_owned()
        } else {
            lockfile
                .blocking_reasons
                .iter()
                .map(|reason| format!("- {reason}"))
                .collect::<Vec<_>>()
                .join("\n")
        };
        bail!("cannot create an executable plan from an analysis-only lockfile:\n{reasons}");
    }
    let weidu = lockfile
        .toolchain
        .weidu
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("executable lockfile has no WeiDU toolchain version"))?;
    let packages = lockfile
        .packages
        .iter()
        .map(|package| {
            (
                (package.environment.as_str(), package.package.as_str()),
                package,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut output = vec![
        "IEPM execution plan (non-mutating)".to_owned(),
        format!("Registry revision: {}", lockfile.registry_revision),
        format!("WeiDU version: {weidu}"),
        "No artifact fetches, materialization, subprocesses, or filesystem mutations occur."
            .to_owned(),
    ];
    let mut action = 1_usize;
    let mut checked_baselines = BTreeSet::new();
    for node in &lockfile.execution {
        let package = packages
            .get(&(node.environment.as_str(), node.package.as_str()))
            .ok_or_else(|| anyhow::anyhow!("execution node {} has no locked package", node.id))?;
        let environment = lockfile
            .environments
            .get(&node.environment)
            .ok_or_else(|| anyhow::anyhow!("execution node {} has no environment", node.id))?;
        output.push(String::new());
        output.push(format!(
            "Environment: {} ({})",
            node.environment, environment.target
        ));
        if checked_baselines.insert(node.environment.as_str()) && !environment.baseline.is_empty() {
            output.push("  Preflight baseline (verify; do not reinstall):".to_owned());
            for entry in &environment.baseline {
                output.push(format!(
                    "    ~{}~ #{} #{}",
                    entry.tp2, entry.language, entry.component
                ));
            }
        }
        if !node.predecessors.is_empty() {
            output.push(format!("  Requires: {}", node.predecessors.join(", ")));
        }
        output.push(format!(
            "{action}. Materialize {}@{}",
            package.package, package.release_id
        ));
        output.push(format!("   Artifact: {}", artifact_description(package)?));
        output.push(format!("   {}", materialization_description(package)));
        action += 1;

        let language = package
            .language
            .as_deref()
            .or(environment.language.as_deref())
            .ok_or_else(|| anyhow::anyhow!("{} has no locked WeiDU language", node.id))?;
        for (tp2, components) in components_by_tp2(package)? {
            let installer = package
                .installers
                .iter()
                .find(|installer| installer.tp2 == tp2)
                .expect("preflighted installer route");
            let language_id = installer
                .languages
                .iter()
                .find(|candidate| candidate.name == language)
                .map(|candidate| candidate.id)
                .ok_or_else(|| anyhow::anyhow!("{tp2} has no language mapping for {language}"))?;
            let numbers = components
                .iter()
                .map(|component| {
                    component
                        .number
                        .expect("preflighted numeric selector")
                        .to_string()
                })
                .collect::<Vec<_>>();
            output.push(format!("{action}. Run WeiDU for {}", package.package));
            output.push(format!("   Installer TP2: {tp2}"));
            output.push(format!(
                "   Command: {}",
                render_command(installer, package, lockfile, language_id, &numbers)?
            ));
            if !package.installer_inputs.is_empty() {
                output.push("   Portable inputs:".to_owned());
                for (name, value) in &package.installer_inputs {
                    output.push(format!("     {name} = {value}"));
                }
            }
            action += 1;
        }
    }
    Ok(format!("{}\n", output.join("\n")))
}

fn render_command(
    installer: &Installer,
    package: &LockedPackage,
    lockfile: &Lockfile,
    language_id: u32,
    components: &[String],
) -> Result<String> {
    let mut command = match installer.launcher {
        InstallerLauncher::Bundled => vec![format!(
            "\"{}\"",
            installer
                .program
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("{} has no executable program", installer.tp2))?
        )],
        InstallerLauncher::Toolchain => vec!["$IEPM_WEIDU".to_owned(), quote(&installer.tp2)],
    };
    for component in components {
        command.push("--force-install".to_owned());
        command.push(component.clone());
    }
    command.push("--language".to_owned());
    command.push(language_id.to_string());
    for argument in &installer.arguments {
        match argument {
            InstallerArgument::Literal { value } => command.push(quote(value)),
            InstallerArgument::EnvironmentInput { input } => {
                let environment = package.installer_inputs.get(input).ok_or_else(|| {
                    anyhow::anyhow!(
                        "{} requires installer input {} for its command",
                        package.package,
                        input
                    )
                })?;
                if !lockfile.environments.contains_key(environment) {
                    bail!(
                        "{} binds {} to unknown environment {}",
                        package.package,
                        input,
                        environment
                    );
                }
                command.push(format!("<bound-environment:{environment}>"));
            }
        }
    }
    Ok(command.join(" "))
}

fn quote(value: &str) -> String {
    format!("\"{value}\"")
}

fn artifact_description(package: &LockedPackage) -> Result<String> {
    let artifact = package
        .artifact
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("{} has no locked artifact", package.package))?;
    Ok(format!("sha256:{} ({})", artifact.sha256, artifact.url))
}

fn materialization_description(package: &LockedPackage) -> String {
    match (
        &package.materialization.source_root,
        package.materialization.include.is_empty(),
    ) {
        (Some(root), true) => format!("Materialization root: {root}"),
        (Some(root), false) => format!(
            "Materialization root: {root}; include: {}",
            package.materialization.include.join(", ")
        ),
        (None, false) => format!(
            "Materialize: {}",
            package.materialization.include.join(", ")
        ),
        (None, true) => "Materialize: package root".to_owned(),
    }
}

fn components_by_tp2(package: &LockedPackage) -> Result<BTreeMap<&str, Vec<&WeiDUComponent>>> {
    let mut groups = BTreeMap::new();
    for component in &package.components {
        let selector = component.weidu.as_ref().ok_or_else(|| {
            anyhow::anyhow!("{}:{} has no WeiDU selector", package.package, component.id)
        })?;
        if selector.number.is_none() {
            bail!(
                "{}:{} has no numeric WeiDU invocation",
                package.package,
                component.id
            );
        }
        if !package
            .installers
            .iter()
            .any(|installer| installer.tp2 == selector.tp2)
        {
            bail!(
                "{}:{} references undeclared installer {}",
                package.package,
                component.id,
                selector.tp2
            );
        }
        groups
            .entry(selector.tp2.as_str())
            .or_insert_with(Vec::new)
            .push(selector);
    }
    if groups.is_empty() {
        bail!("{} has no executable WeiDU components", package.package);
    }
    Ok(groups)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lockfile(readiness: &str) -> Lockfile {
        serde_json::from_str(&format!(
            r#"{{
                "schema": 3,
                "environments": {{"target": {{"target": "bg2ee", "language": "English"}}}},
                "registry_revision": "fixture",
                "toolchain": {{"iepm": "test", "weidu": "24600"}},
                "packages": [{{
                    "package": "fixture", "release_id": "release-1", "version": "One",
                    "environment": "target", "phase": "eet",
                    "artifact": {{"url": "https://example.invalid/fixture.zip", "sha256": "{}"}},
                    "materialization": {{"source_root": "fixture", "include": ["mod"]}},
                    "installers": [{{"tp2": "mod/setup-fixture.tp2", "program": "setup-fixture.exe", "languages": [{{"id": 0, "name": "English"}}]}}],
                    "components": [{{"id": "main", "weidu": {{"tp2": "mod/setup-fixture.tp2", "label": "main", "number": 0}}}}],
                    "language": "English", "provenance": "verified"
                }}],
                "execution": [{{"id": "target::fixture", "package": "fixture", "environment": "target", "phase": "eet"}}],
                "execution_readiness": "{}",
                "blocking_reasons": ["fixture reason"]
            }}"#,
            "a".repeat(64), readiness
        ))
        .unwrap()
    }

    #[test]
    fn renders_an_auditable_non_mutating_plan() {
        let plan = render_plan(&lockfile("executable")).unwrap();
        assert!(plan.contains("IEPM execution plan (non-mutating)"));
        assert!(plan.contains("Materialize fixture@release-1"));
        assert!(plan.contains("\"setup-fixture.exe\" --force-install 0 --language 0"));
        assert!(plan.contains("No artifact fetches"));
    }

    #[test]
    fn refuses_analysis_only_lockfiles_with_reasons() {
        let error = render_plan(&lockfile("analysis-only")).unwrap_err();
        assert!(error.to_string().contains("fixture reason"));
    }

    #[test]
    fn legacy_lockfiles_default_to_analysis_only_and_require_reresolution() {
        let mut value = serde_json::to_value(lockfile("executable")).unwrap();
        let object = value.as_object_mut().unwrap();
        object.insert("schema".to_owned(), serde_json::json!(2));
        object.remove("execution_readiness");
        object.remove("blocking_reasons");
        let legacy: Lockfile = serde_json::from_value(value).unwrap();
        assert_eq!(legacy.execution_readiness, ExecutionReadiness::AnalysisOnly);
        assert!(
            render_plan(&legacy)
                .unwrap_err()
                .to_string()
                .contains("re-resolve it to schema 3")
        );
    }

    #[test]
    fn renders_a_toolchain_route_and_baseline_without_machine_paths() {
        let lock: Lockfile = serde_json::from_value(serde_json::json!({
            "schema": 3,
            "environments": {
                "bgee-source": {"target": "bgee"},
                "eet-target": {
                    "target": "eet", "language": "English",
                    "baseline": [{
                        "tp2": "eefixpack/setup-eefixpack.tp2",
                        "language": 0,
                        "component": 0
                    }]
                }
            },
            "registry_revision": "fixture",
            "toolchain": {"iepm": "test", "weidu": "24600"},
            "packages": [{
                "package": "eet", "release_id": "release-1", "version": "One",
                "environment": "eet-target", "phase": "eet-import",
                "artifact": {"url": "https://example.invalid/eet.zip", "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"},
                "installers": [{
                    "tp2": "EET/EET.tp2", "launcher": "toolchain",
                    "languages": [{"id": 0, "name": "English"}],
                    "inputs": [{"name": "source-environment", "required": true}],
                    "arguments": [
                        {"kind": "literal", "value": "--args-list"},
                        {"kind": "literal", "value": "sp"},
                        {"kind": "environment-input", "input": "source-environment"}
                    ]
                }],
                "components": [{"id": "core", "weidu": {"tp2": "EET/EET.tp2", "number": 0}}],
                "language": "English",
                "installer_inputs": {"source-environment": "bgee-source"},
                "provenance": "derived"
            }],
            "execution": [{"id": "eet-target::eet", "package": "eet", "environment": "eet-target", "phase": "eet-import"}],
            "execution_readiness": "executable"
        }))
        .unwrap();
        let plan = render_plan(&lock).unwrap();
        assert!(plan.contains("$IEPM_WEIDU \"EET/EET.tp2\" --force-install 0 --language 0 \"--args-list\" \"sp\" <bound-environment:bgee-source>"));
        assert!(plan.contains("Preflight baseline (verify; do not reinstall):"));
        assert!(!plan.contains("C:\\"));
    }
}
