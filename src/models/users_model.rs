
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use super::super::schema::users;

#[derive(Queryable, Selectable, Debug, Serialize, Ord, Eq, PartialEq, PartialOrd,Insertable,Deserialize)]
#[diesel(table_name = users)]
pub struct Users{
    pub id:i32,
    pub email:String,
    pub name:String,
}

#[derive(Deserialize)]
pub struct NewUser{
    pub email:String,
    pub name:String,
}

