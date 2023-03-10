use crate::commonlib::CommonLibraryApi;
use crate::smoke::{ResultBuilder, SmokeTest, TestResult, TestResultBuilder, ViewsTest};
use async_trait::async_trait;
use std::time::Instant;

#[async_trait]
impl SmokeTest for ViewsTest {
    async fn run(&self) -> TestResult {
        let test_result = TestResultBuilder::default()
            .set_name(self.name.clone())
            .set_details("Good".to_string())
            .set_duration(Instant::now());

        let view_definition = self
            .client
            .get_genericview_definition("Facility".to_string())
            .await;
        let name_column: Vec<_> = view_definition
            .columns
            .iter()
            .filter(|col| {
                if let Some(name) = col.column_name.as_ref() {
                    name.eq(&"Name")
                } else {
                    false
                }
            })
            .collect();

        assert!(!name_column.is_empty(), "Facility.Name column is missing");
        let tie: Vec<_> = view_definition
            .columns
            .iter()
            .filter(|col| {
                if let Some(name) = col.column_name.as_ref() {
                    name.eq(&"IsForTIE")
                } else {
                    false
                }
            })
            .collect();
        assert!(!tie.is_empty(), "Facility.IsForTIE column is missing");

        test_result.success()
    }
}
