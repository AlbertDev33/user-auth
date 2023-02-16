use std::env::set_var;

pub fn constants() -> String {
    set_var(
        "RUST_LOG",
        "user-auth-server=debug,actix_web=info,actix_server=info",
    );
    
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
    
    return database_url;
    
}