fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: only set if not set already...
    let db_url = "postgres://postgres:mypassword@localhost:5432/postgres";
    println!("cargo:rustc-env=DATABASE_URL={db_url}");

    Ok(())
}
