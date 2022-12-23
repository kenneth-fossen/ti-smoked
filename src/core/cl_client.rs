use super::TestTarget;

pub struct ClClient {
    appkey: String,
    baseurl: String,
    tokenprovider: String,
}

impl ClClient {
    fn build(&self, test_target: TestTarget) -> ClClient {
        let appkey = test_target.get_config_value("CommonLibraryAppId");
        let baseurl = test_target.get_config_value("CommonLibraryApiBaseAddress");
        let tokenprovider = test_target.get_config_value("TokenProviderConnectionString");
        ClClient {
            appkey: appkey,
            baseurl: baseurl,
            tokenprovider: tokenprovider,
        }
    }
    
}
