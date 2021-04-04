//use crate::context::Context;
#[allow(dead_code)]
use std::default::Default;

#[derive(Default)]
pub struct Router {
	routes: Vec<Route>,
}

pub struct Route {
	path: String,
	method: Method,
	//context:	Context
}

enum Method {
	Get,
	Post,
	Patch,
	Put,
	Delete,
	Options,
}

impl Router {}
