use diesel::PgConnection;
//use diesel::database_error::DatabaseResult;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::OptionalExtension;
use diesel::QueryResult;
use diesel::Connection;
use query_helper;
use database_error::{ DatabaseResult, DatabaseError};
use migrations_internals as migrations;
use rocket::local::Client;
use server::{Config, init_rocket};

use std::sync::{MutexGuard, Mutex};


const DATABASE_NAME: &'static str = "weekend_test";
pub const DATABASE_URL: &'static str = env!("TEST_DATABASE_URL");

const DROP_DATABASE_URL: &'static str = env!("DROP_DATABASE_URL");


/// This creates a singleton of the base database connection.
///
/// The base database connection is required, because you cannot drop the other database from itself.
///
/// Because it is wrapped in a mutex, only one test at a time can access it.
/// The setup method will lock it and use it to reset the database.
///
/// It is ok if a test fails and poisons the mutex, as the one place where it is used disregards the poison.
/// Disregarding the poison is fine because code using the mutexed value never modifies the value,
/// so there is no indeterminate state to contend with.
lazy_static! {
    static ref CONN: Mutex<PgConnection> =
        Mutex::new(PgConnection::establish(DROP_DATABASE_URL).expect("Database not available"));
}


/// The Fixture trait should be implemented for collections of data used in testing.
/// Because it can be instantiated using just a connection to the database,
/// it allows the creation of the type in question and allows data generated at row insertion time
/// (UUIDs) to be made available to the body of tests.
pub trait Fixture {
    fn generate(conn: &PgConnection) -> Self;
}

/// Because some tests may not require any initial database state, but still utilize the connection,
/// This Fixture is provided to meet that need.
pub struct EmptyFixture;

impl Fixture for EmptyFixture {
    fn generate(_conn: &PgConnection) -> Self {
        EmptyFixture
    }
}


/// Sets up the database and runs the provided closure where the test code should be present.
/// By running your tests using this method, you guarantee that the database only contains the rows
/// created in the fixture's `generate()` function, and the thread will block if another test using
/// this function is currently running, preventing side effects from breaking other tests.
pub fn setup<Fun, Fix >( mut test_function: Fun )
    where
        Fun: FnMut (&Fix, &PgConnection), // The FnMut adds support for benchers, as they are required to mutate on each iteration.
        Fix: Fixture
{
    let admin_conn: MutexGuard<PgConnection> = match CONN.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // Don't care if the mutex is poisoned
    };
    reset_database(&admin_conn);

    let actual_connection: PgConnection = PgConnection::establish(DATABASE_URL).expect("Database not available.");
    run_migrations(&actual_connection);
    let fixture: Fix = Fix::generate(&actual_connection);
    test_function (&fixture, &actual_connection);
}


pub fn setup_client<Fun, Fix >( mut test_function: Fun )
    where
        Fun: FnMut (&Fix, Client), // The FnMut adds support for benchers, as they are required to mutate on each iteration.
        Fix: Fixture
{
    let admin_conn: MutexGuard<PgConnection> = match CONN.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // Don't care if the mutex is poisoned
    };
    reset_database(&admin_conn);

    let actual_connection: PgConnection = PgConnection::establish(DATABASE_URL).expect("Database not available.");
    run_migrations(&actual_connection);
    let fixture: Fix = Fix::generate(&actual_connection);

    let mut config = Config::default();
    config.db_url = DATABASE_URL.to_string(); // set the testing db url
    let rocket = init_rocket(config);
    let client = Client::new(rocket).expect(
        "Valid rocket instance",
    );

    test_function (&fixture, client);
}

/// Drops the database and then recreates it.
/// The guarantee that this function provides is that the test database will be in a default
/// state, without any run migrations after this ran.
fn reset_database(conn: &PgConnection) {
    // TODO instead of dropping, I could instead just revert all migrations.
    drop_database(&conn).expect("Could not drop db");
    create_database(&conn).expect("Could not create Database");
}

/// Drops the database
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

/// Recreates the database
fn create_database(conn: &PgConnection) ->  DatabaseResult<()> {
    let db_result = query_helper::create_database(DATABASE_NAME)
        .execute(conn)
        .map_err(DatabaseError::from)
        .map(|_| ());
    println!("Created database");
    db_result
}

/// Creates tables
fn run_migrations(conn: &PgConnection) {
    use std::path::Path;
    // TODO this is a hack to make running the tests possible in both the db directory and the server directory
    let migrations_dir = Path::new("../db/migrations");
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