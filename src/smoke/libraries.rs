use crate::commonlib::CommonLibraryApi;
use crate::smoke::{LibrariesTest, ResultBuilder, SmokeTest, TestResult, TestResultBuilder};
use async_trait::async_trait;
use std::time::Instant;

#[async_trait]
impl SmokeTest for LibrariesTest {
    async fn run(&self) -> TestResult {
        let test_result = TestResultBuilder::default()
            .set_name(self.name.clone())
            .set_duration(Instant::now());

        let res = self
            .client
            .get_library("Facility and Project".to_string())
            .await;
        assert_eq!(
            res.iter().len() > 0,
            true,
            "List of libraries should not be empty"
        );

        let facility: Vec<_> = res.iter().filter(|lib| lib.name.eq("Facility")).collect();
        assert_eq!(facility.len(), 1);
        let cablecode: Vec<_> = res.iter().filter(|lib| lib.name.eq("CableCode")).collect();
        assert_eq!(cablecode.is_empty(), true);
        test_result.success()
    }
}
