use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Setup deps via Docker
    println!("cargo::rerun-if-changed=docker-compose.yml");
    
    let output = Command::new("docker")
        .args(["compose", "-f", "../docker-compose.yml", "up", "-d"])
        .spawn()?
        .wait_with_output()?;

    if !output.status.success() {
        return Err("docker-compose failed".into());
    }

    let db_url = "postgres://postgres:mypassword@localhost:5432/postgres";
    println!("cargo:rustc-env=DATABASE_URL={db_url}");

    Ok(())
}