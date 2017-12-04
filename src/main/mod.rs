/**
    @author anlaakso (Andrew)
    @license GNU/GPLv3
*/


use config::ArangoConfig;

#[derive(Debug)]
pub struct Arango<'a> {
	config: ArangoConfig<'a>
}

impl<'a> Arango<'a> {
	pub fn new(config: ArangoConfig<'a>) -> Self {
		Arango::<'a> {
			config
		}
	}
}