/**
    @author anlaakso (Andrew)
    @license GNU/GPLv3
*/

#[macro_export]
macro_rules! arango {
    (
        $($key:ident : $value:expr),*
    ) => {{
        use arango::config::ArangoConfig;
        use arango::main::Arango;

        let config = ArangoConfig {
            $($key: $value,)* 
            .. ArangoConfig::default()
        };

        Arango::new(config)
    }};
}
