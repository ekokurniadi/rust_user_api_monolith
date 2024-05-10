use crate::db::PgPool;
use crate::models::users_model::{LoginUser, NewUser, Users};
use crate::schema::users::{self, email, name, password};
use diesel::result::Error::NotFound;
use diesel::{prelude::*, result::Error};

pub struct UserService(pub PgPool);

pub trait IUserRepository {
    fn new(db_pool: PgPool) -> UserService;
    async fn get_users(&self, page: i64, limit: i64) -> (Result<Vec<Users>, Error>, i64);
    async fn create_user(&self, new_user: NewUser) -> Result<Users, Error>;
    async fn update_user(&self, user_id: i32, new_user: NewUser) -> Result<Users, Error>;
    async fn delete_user(&self, user_id: i32) -> Result<bool, Error>;
    async fn login(&self, new_user: LoginUser) -> Result<Users, Error>;
}

impl IUserRepository for UserService {
    fn new(db_pool: PgPool) -> Self {
        UserService(db_pool)
    }

    async fn get_users(&self, page: i64, limit: i64) -> (Result<Vec<Users>, Error>, i64) {
        let mut conn = self.0.get().expect("couldn't get connection from pool");

        let count_user = users::table.count().get_result(&mut conn).unwrap_or(0);

        let offset = (page - 1) * limit;
        let query = users::table
            .offset(offset)
            .limit(limit)
            .order(users::id.asc())
            .load::<Users>(&mut conn);

        let res = match query {
            Ok(val) => Ok(val),
            Err(e) => Err(e),
        };

        (res, count_user)
    }

    async fn create_user(&self, new_user: NewUser) -> Result<Users, Error> {
        let mut conn = self.0.get().expect("couldn't get connection from pool");
        let user = diesel::insert_into(users::table)
            .values((
                name.eq(new_user.name),
                email.eq(new_user.email),
                password.eq(new_user.password),
            ))
            .get_result(&mut conn);

        let res: Result<Users, Error> = match user {
            Ok(val) => Ok(val),
            Err(e) => Err(e),
        };

        res
    }

    async fn update_user(&self, user_id: i32, new_user: NewUser) -> Result<Users, Error> {
        let mut conn = self.0.get().expect("couldn't get connection from pool");
        let user = diesel::update(users::table.find(user_id))
            .set((
                users::name.eq(new_user.name),
                users::email.eq(new_user.email),
                users::password.eq(new_user.password),
            ))
            .get_result(&mut conn);

        let res: Result<Users, Error> = match user {
            Ok(val) => Ok(val),
            Err(e) => Err(e),
        };

        res
    }

    async fn delete_user(&self, user_id: i32) -> Result<bool, Error> {
        let mut conn = self.0.get().expect("couldn't get connection from pool");
        match diesel::delete(users::table.find(user_id)).execute(&mut conn) {
            Ok(_) => Ok(true), // Deletion successful
            Err(_) => Ok(false),
        }
    }

    async fn login(&self, new_user: LoginUser) -> Result<Users, Error> {
        let mut conn = self.0.get().expect("couldn't get connection from pool");
        let user = users::table
            .filter(email.eq(new_user.email))
            .filter(password.eq(new_user.password))
            .select(users::all_columns)
            .first::<Users>(&mut conn);

        let res: Result<Users, Error> = match user {
            Ok(val) => Ok(val),
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    return Err(Error::NotFound);
                }
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        };

        res
    }
}
