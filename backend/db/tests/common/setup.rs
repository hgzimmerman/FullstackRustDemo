use diesel::PgConnection;
//use diesel::database_error::DatabaseResult;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::OptionalExtension;
use diesel::QueryResult;
use diesel::Connection;
use common::query_helper;
use common::database_error::{ DatabaseResult, DatabaseError};
use migrations_internals as migrations;

use std::sync::{MutexGuard, Mutex};


const DATABASE_NAME: &'static str = "weekend";
// TODO, Create a testing db url so tests don't clobber the dev DB.
pub const DATABASE_URL: &'static str = env!("DATABASE_URL");

const DROP_DATABASE_URL: &'static str = env!("DROP_DATABASE_URL");


/// This creates a singleton of the base database connection.
///
/// The base database connection is required, because you cannot drop the other database from itself.
///
/// Because it is wrapped in a mutex, only one test at a time can access it.
/// The setup method will lock it and use it to reset the database.
lazy_static! {
    static ref CONN: Mutex<PgConnection> =
        Mutex::new(PgConnection::establish(DROP_DATABASE_URL).expect("Database not available"));
}


pub trait Fixture {
    fn generate(conn: &PgConnection) -> Self;
}

pub struct EmptyFixture;

impl Fixture for EmptyFixture {
    fn generate(_conn: &PgConnection) -> Self {
        EmptyFixture
    }
}


pub fn setup<Fun, Fix >( mut test_function: Fun )
    where
        Fun: FnMut (&Fix, &PgConnection), // The FnMut adds support for benchers, as they are required to mutate on each iteration.
        Fix: Fixture
{
    let admin_conn: MutexGuard<PgConnection> = CONN.lock().unwrap();
    reset_database(&admin_conn);

    let actual_connection: PgConnection = PgConnection::establish(DATABASE_URL).expect("Database not available.");
    run_migrations(&actual_connection);
    let fixture: Fix = Fix::generate(&actual_connection);
    test_function (&fixture, &actual_connection);
}

fn reset_database(conn: &PgConnection) {
    // TODO instead of dropping, I could instead just revert all migrations.
    drop_database(&conn).expect("Could not drop db");
    let _ = create_database(&conn);
}

fn drop_database(conn: &PgConnection) ->  DatabaseResult<()> {

    if pg_database_exists(&conn, DATABASE_NAME)? {
        println!("Dropping database: {}", DATABASE_NAME);
        query_helper::drop_database(DATABASE_NAME)
            .if_exists()
            .execute(conn)
            .map_err(DatabaseError::from)
            .map(|_|())

    } else {
        Ok(())
    }
}


fn create_database(conn: &PgConnection) ->  DatabaseResult<()> {
    query_helper::create_database(DATABASE_NAME)
        .execute(conn)
        .map_err(DatabaseError::from)
        .map(|_| ())
}


fn run_migrations(conn: &PgConnection) {
    use std::path::Path;
    let migrations_dir = Path::new("migrations");
    migrations::run_pending_migrations_in_directory(conn, migrations_dir, &mut ::std::io::sink())
        .expect("Couldn't run migrations.");
}

table! {
    pg_database (datname) {
        datname -> Text,
        datistemplate -> Bool,
    }
}

fn pg_database_exists(conn: &PgConnection, database_name: &str) -> QueryResult<bool> {
    use self::pg_database::dsl::*;

    pg_database
        .select(datname)
        .filter(datname.eq(database_name))
        .filter(datistemplate.eq(false))
        .get_result::<String>(conn)
        .optional()
        .map(|x| x.is_some())
}