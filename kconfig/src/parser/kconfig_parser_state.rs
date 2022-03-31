pub trait KconfigParserState {}

pub struct Building {}

impl KconfigParserState for Building {}

pub struct Parsing {}

impl KconfigParserState for Parsing {}

pub struct Done {}

impl KconfigParserState for Done {}
