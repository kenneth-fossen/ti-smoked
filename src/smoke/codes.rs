use crate::commonlib::entities::Code;
use crate::commonlib::CommonLibraryApi;
use crate::smoke::{CodesTest, ResultBuilder, SmokeTest, TestResult, TestResultBuilder};
use async_trait::async_trait;
use chrono::Datelike;
use std::time::Instant;

#[async_trait]
impl SmokeTest for CodesTest {
    async fn run(&self) -> TestResult {
        let test_result = TestResultBuilder::default()
            .set_name(self.name.clone())
            .set_duration(Instant::now());
        // // "CableCode", "CommonLibrary", "AHA"
        let _res = self.client.get_code("Facility".to_string()).await;

        let facility: Vec<&Code> = _res.iter().filter(|code| code.name.eq("JSV")).collect();

        if facility.is_empty() {
            return test_result.failed();
        }

        let jsv = facility.first().unwrap().to_owned();
        assert_eq!(jsv.identity, "JSV", "Should be JSV for the identity");
        assert_eq!(
            jsv.description.as_ref().unwrap(),
            "Johan Sverdrup",
            "Description should be Johan Sverdrup"
        );
        assert!(jsv.is_valid, "JSV Should be valid");
        assert_eq!(jsv.date_created.year(), 2019, "JSV was created in 2019");
        assert!(
            jsv.date_updated.year() > jsv.date_created.year(),
            "Cannot be updated before created"
        );
        let sapplant: Vec<_> = jsv
            .attributes
            .iter()
            .filter(|attrib| attrib.definition_name.eq("SAPPlant"))
            .collect();
        assert_eq!(
            sapplant.first().unwrap().display_value,
            "See subinstallations",
            "JSV.SAPPlant error"
        );
        let tie: Vec<_> = jsv
            .attributes
            .iter()
            .filter(|attrib| attrib.definition_name.eq("IsForTIE"))
            .collect();
        assert_eq!(
            tie.first().unwrap().display_value,
            "True",
            "JSV.IsForTIE error"
        );

        test_result.success()
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
