/**
    @author anlaakso (Andrew)
    @license GNU/GPLv3
*/

use connection::verbs::Verbs;

#[derive(Debug)]
pub struct ConnectionParams<'a> {
	pub path: &'a str,
	pub method: Verbs,
	pub json: Option<&'a str>
}