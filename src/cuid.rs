use pgx::*;
use cuid;

/// Generate a random cuid UUID
#[pg_extern]
fn idkit_cuid_generate() -> String {
    match cuid::cuid() {
        Err(e) => error!("failed to generate cuid: {}", e),
        Ok(id) => id,
    }
}

//////////
// Test //
//////////

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    /// Basic length test
    #[pg_test]
    fn test_cuid_len() {
        let generated = crate::cuid::idkit_cuid_generate();
        assert_eq!(generated.len(), 25);
    }
}
