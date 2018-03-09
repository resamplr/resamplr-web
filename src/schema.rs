use diesel;
use diesel::prelude::*;

mod schema {	
	infer_schema!("dotenv:DATABASE_URL");
}