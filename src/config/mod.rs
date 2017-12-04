/**
	@author anlaakso (Andrew)
	@license GNU/GPLv3
*/


#[derive(Debug, Clone)]
pub struct ArangoConfig<'a> {
    pub host: &'a str,
    pub port: u32
}

impl<'a> Default for ArangoConfig<'a> {
    fn default() -> ArangoConfig<'a> {
        ArangoConfig {
            host: &"localhost",   
            port: 8529,
        }
    }
}
