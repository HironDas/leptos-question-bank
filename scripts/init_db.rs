#!/usr/bin/env cargo
//!```cargo
//! [dependencies]
//! xshell = "0.2.7"
//! anyhow = "1.0.100"
//! ```

use xshell::{cmd, Shell};
use std::time::Duration;
use std::thread;

fn main()->anyhow::Result<()>{
    let sh = Shell::new()?;
    sh.set_var("XSHELL_ECHO", "1");

    //  Check if required binaries are installed
    if cmd!(sh, "psql --version").quiet().run().is_err(){
        eprintln!("Error: psql is not installed");
        std::process::exit(1);
    }

    if cmd!(sh, "sqlx --version").quiet().run().is_err(){
        eprintln!("Error: sqlx is not installed.");
        std::process::exit(1);
    }

    let args: Vec<String> = std::env::args().collect();
    let skip_docker_flag = args.contains(&"--skip-docker".to_string());

    let db_user = sh.var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
    let db_password = sh.var("POSTGRES_PASSWORD").unwrap_or_else(|_| "password".to_string());
    let db_name = sh.var("POSTGRES_DB").unwrap_or_else(|_| "question_bank".to_string());
    let db_port = sh.var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
    let skip_docker = skip_docker_flag || sh.var("SKIP_DOCKER").is_ok();

    println!("SKIP_DOCKER: {skip_docker}");

    if !skip_docker {
        cmd!(sh,
            "docker run
            -e POSTGRES_USER={db_user}
            -e POSTGRES_PASSWORD={db_password}
            -e POSTGRES_DB={db_name}
            -p {db_port}:5432
            --restart=always
            --name question_bank_postgres
            -d postgres:17
            postgres -N 1000"
        ).run()?
    }

    // Wait for Postgres to start
    // Set PGPASSWORD
    sh.set_var("PGPASSWORD", &db_password);

    println!("Waiting for postgres to start...");
    loop{
        let check_db = cmd!(sh,"psql -h localhost -U {db_user} -p {db_port} -d postgres -c \\q")
            .ignore_stderr()
            .run();

        if check_db.is_ok(){
            break;
        }

        eprintln!("Postgres is still unavailable - sleeping");
        thread::sleep(Duration::from_secs(2));
    }

    println!("Postgres is up and running on port {db_port}!");

    // Run migration
    let database_url = format!("postgres://{db_user}:{db_password}@localhost:{db_port}/{db_name}");
    sh.set_var("DATABASE_URL", database_url);

    cmd!(sh, "sqlx database create").run()?;
    cmd!(sh, "sqlx migrate run").run()?;

    Ok(())
}
