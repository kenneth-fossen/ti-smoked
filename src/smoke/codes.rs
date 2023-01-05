use std::time::Instant;
use async_trait::async_trait;
use crate::smoke::{SmokeTest, TestResult, TestResultBuilder, ResultBuilder, CodesTest};
use crate::commonlib::CommonLibraryApi;
use crate::commonlib::entities::Code;

#[async_trait]
impl SmokeTest for CodesTest {
    async fn run(&self) -> TestResult {
        let test_result = TestResultBuilder::default()
            .set_name(self.name.clone())
            .set_duration(Instant::now());
        // // "CableCode", "CommonLibrary", "AHA"
        let _res = self.client.get_code("Facility".to_string()).await;

        let facility: Vec<&Code> = _res.iter()
            .filter(|code| code.name.eq("JSV"))
            .collect();

        if facility.is_empty() {
            return test_result.failed();
        }
        let jsv = facility.first().unwrap().to_owned();
        let identity = jsv.identity.eq("JSV");
        let descption = jsv.description.as_ref().unwrap().eq("Johan Sverdrup");
        let valid = jsv.is_valid;


        test_result.failed()
    }
}

/*
            Assert.True(facilities.Count > 0, "List of facilities should not be empty");

                var jsv = facilities.SingleOrDefault(x => x.Name == "JSV");
                Assert.NotNull(jsv, "JSV should be in the list of facilities");

                Assert.Equal(jsv.Identity, "JSV");
                Assert.Equal(jsv.Description, "Johan Sverdrup");
                Assert.True(jsv.IsValid, "JSV should be valid");
                Assert.True(jsv.DateCreated.Value.Year == 2019, "Expected JSV to have been created in 2019");
                Assert.True(jsv.DateUpdated.Value > jsv.DateCreated.Value, "Expected DateUpdated to be after DateCreated");

                var attributes = jsv.Attributes;
                Assert.True(attributes.SingleOrDefault(x => x.DefinitionName == "SAPPlant")?.DisplayValue == "See subinstallations", "JSV.SAPPlant error");
                Assert.True(attributes.SingleOrDefault(x => x.DefinitionName == "IsForTIE")?.DisplayValue == "True", "JSV.IsForTIE error");

            }
 */