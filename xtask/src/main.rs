use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    match (args.next().as_deref(), args.next()) {
        (Some("architecture"), None) => match architecture() {
            Ok(()) => {
                println!("Architecture contracts and ownership guardrails passed.");
                ExitCode::SUCCESS
            }
            Err(errors) => {
                for error in errors {
                    eprintln!("architecture: {error}");
                }
                ExitCode::FAILURE
            }
        },
        _ => {
            eprintln!("usage: cargo xtask architecture");
            ExitCode::from(2)
        }
    }
}

fn architecture() -> Result<(), Vec<String>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be directly under workspace root");
    let mut errors = Vec::new();

    let workspace = read(root.join("Cargo.toml"), &mut errors);
    let contracts = [
        (
            "ensambria-core-identity-auth-contracts",
            "backend/core-identity-auth-contracts",
        ),
        (
            "ensambria-core-tenancy-contracts",
            "backend/core-tenancy-contracts",
        ),
        (
            "ensambria-business-establishment-catalog-contracts",
            "backend/business-establishment-catalog-contracts",
        ),
        (
            "ensambria-platform-product-catalog-contracts",
            "backend/platform-product-catalog-contracts",
        ),
        (
            "ensambria-platform-tenant-onboarding-contracts",
            "backend/platform-tenant-onboarding-contracts",
        ),
    ];
    for (package_name, path) in contracts {
        if !workspace.contains(path) {
            errors.push(format!("{package_name} is not a workspace member"));
        }
        require_file(root, format!("{path}/Cargo.toml"), &mut errors);
        require_file(root, format!("{path}/src/lib.rs"), &mut errors);
    }

    let forbidden = [
        ("backend/business/Cargo.toml", "ensambria-core ="),
        ("backend/business/Cargo.toml", "ensambria-platform ="),
        ("backend/platform/Cargo.toml", "ensambria-core ="),
        ("backend/platform/Cargo.toml", "ensambria-business ="),
        ("backend/core/Cargo.toml", "ensambria-business ="),
    ];
    for (manifest, dependency) in forbidden {
        let content = read(root.join(manifest), &mut errors);
        let dependencies = dependency_section(&content);
        if dependencies
            .lines()
            .any(|line| line.trim_start().starts_with(dependency))
        {
            errors.push(format!(
                "{manifest} has forbidden implementation dependency {dependency}"
            ));
        }
    }

    let business_forbidden = ["ensambria_core::", "ensambria_platform::"];
    let platform_forbidden = ["ensambria_core::", "ensambria_business::"];
    let core_forbidden = ["ensambria_platform::", "ensambria_business::"];
    for (directory, forbidden_tokens) in [
        ("backend/business/src", business_forbidden.as_slice()),
        ("backend/platform/src", platform_forbidden.as_slice()),
        ("backend/core/src", core_forbidden.as_slice()),
    ] {
        for file in rust_files(root.join(directory), &mut errors) {
            let content = read(file.clone(), &mut errors);
            for token in forbidden_tokens {
                if content.contains(token) {
                    errors.push(format!(
                        "{} imports forbidden implementation path {token}",
                        file.display()
                    ));
                }
            }
        }
    }

    let ownership = read(root.join("architecture/data-ownership.yaml"), &mut errors);
    for schema in [
        "core_identity_auth",
        "core_tenancy",
        "business_establishment_catalog",
        "platform_product_catalog",
        "platform_tenant_onboarding",
    ] {
        if !ownership.contains(&format!("schema: {schema}")) {
            errors.push(format!("ownership inventory is missing schema {schema}"));
        }
    }

    require_file(root, "buf.yaml", &mut errors);
    require_file(root, "buf.gen.yaml", &mut errors);
    for proto in [
        "contracts/protobuf/ensambria/core/identity_authentication/v1/identity_authentication.proto",
        "contracts/protobuf/ensambria/core/tenancy/v1/tenancy.proto",
        "contracts/protobuf/ensambria/business/establishment_catalog/v1/establishment_catalog.proto",
        "contracts/protobuf/platform/product_catalog/v1/product_catalog.proto",
        "contracts/protobuf/ensambria/platform/tenant_onboarding/v1/tenant_onboarding.proto",
    ] {
        require_file(root, proto, &mut errors);
    }

    let outbox = read(
        root.join("backend/core/migrations/20261122000011_core_tenant_created_outbox.sql"),
        &mut errors,
    );
    let inbox = read(
        root.join("backend/platform/migrations/20261122000002_platform_tenant_onboarding_inbox.sql"),
        &mut errors,
    );
    for field in [
        "event_id",
        "aggregate_version",
        "correlation_id",
        "payload",
        "published_at",
    ] {
        if !outbox.contains(field) {
            errors.push(format!("CORE Outbox contract is missing {field}"));
        }
    }
    for field in [
        "event_id",
        "aggregate_version",
        "correlation_id",
        "payload_hash",
        "applied_at",
        "quarantined",
    ] {
        if !inbox.contains(field) {
            errors.push(format!("PLATFORM Inbox contract is missing {field}"));
        }
    }

    let ci = read(
        root.join(".github/workflows/repository-quality.yml"),
        &mut errors,
    );
    if !ci.contains("cargo xtask architecture") {
        errors.push("CI does not execute cargo xtask architecture".to_owned());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn dependency_section(manifest: &str) -> String {
    let mut active = false;
    let mut result = String::new();
    for line in manifest.lines() {
        if line.starts_with('[') {
            active = line == "[dependencies]";
        } else if active {
            result.push_str(line);
            result.push(char::from(10));
        }
    }
    result
}

fn require_file(root: &Path, relative: impl AsRef<Path>, errors: &mut Vec<String>) {
    let path = root.join(relative.as_ref());
    if !path.is_file() {
        errors.push(format!(
            "required file is missing: {}",
            relative.as_ref().display()
        ));
    }
}

fn read(path: PathBuf, errors: &mut Vec<String>) -> String {
    match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) => {
            errors.push(format!("cannot read {}: {error}", path.display()));
            String::new()
        }
    }
}

fn rust_files(path: PathBuf, errors: &mut Vec<String>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let entries = match fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(error) => {
            errors.push(format!("cannot list {}: {error}", path.display()));
            return files;
        }
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(rust_files(path, errors));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
    files
}
