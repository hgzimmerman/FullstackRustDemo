use diesel::PgConnection;
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
use testing_fixtures::Fixture;
use pool::Pool;

use std::sync::{MutexGuard, Mutex};


pub const DATABASE_NAME: &'static str = "weekend_test";

/// Points to the database that tests will be performed on.
/// The database schema will be destroyed and recreated before every test.
/// It absolutely should _never_ point to a production database,
/// as tests ran using it will likely create an admin account that has known login credentials.
pub const DATABASE_URL: &'static str = env!("TEST_DATABASE_URL");

/// Should point to the base postgres account.
const DROP_DATABASE_URL: &'static str = env!("DROP_DATABASE_URL");



/// This creates a singleton of the base database connection.
///
/// The base database connection is required, because you cannot drop the other database from itself.
///
/// Because it is wrapped in a mutex, only one test at a time can access it.
/// The setup method will lock it and use it to reset the database.
///
/// It is ok if a test fails and poisons the mutex, as the one place where it is used disregards the poison.
/// Disregarding the poison is fine because code using the mutex-ed value never modifies the value,
/// so there is no indeterminate state to contend with if a prior test has panicked.
lazy_static! {
    static ref CONN: Mutex<PgConnection> =
        Mutex::new(PgConnection::establish(DROP_DATABASE_URL).expect("Database not available"));
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
    // Sleep-wait for the one connection to the administration account database connection to become available.
    let admin_conn: MutexGuard<PgConnection> = match CONN.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // Don't care if the mutex is poisoned
    };
    reset_database(&admin_conn);

    // Create a connection to the test database.
    let conn: PgConnection = PgConnection::establish(DATABASE_URL).expect("Database not available.");
    run_migrations(&conn);
    let fixture: Fix = Fix::generate(&conn);
    test_function (&fixture, &conn);
}

/// Sets up the provided test fixture much like the `setup()` funciton,
/// except that it provides a Rocket client instead of the raw database connection.
pub fn setup_client<Fun, Fix >( mut test_function: Fun )
    where
        Fun: FnMut (&Fix, Client), // The FnMut adds support for `benchers`, as they are required to mutate on each iteration.
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


pub fn setup_warp<Fun, Fix> (mut test_function: Fun)
where
    Fun: FnMut(&Fix, Pool),
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

    // Establish a pool, this will be passed in as part of the State object when simulating the api.
    let testing_pool = pool::init_pool(DATABASE_URL);
    test_function(&fixture, testing_pool)
}

/// Drops the database and then recreates it.
/// The guarantee that this function provides is that the test database will be in a default
/// state, without any run migrations after this ran.
fn reset_database(conn: &PgConnection) {
    // TODO instead of dropping, I could instead just revert all migrations.
    drop_database(&conn).expect("Could not drop db");
    create_database(&conn).expect("Could not create Database");
}

/// Drops the database, completely removing every table (and therefore every row) in the database.
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
    println!("Created database:  {}", DATABASE_NAME);
    db_result
}

/// Creates tables in the database.
fn run_migrations(conn: &PgConnection) {
    use std::path::Path;
    // This directory traversal allows this library to be used by any crate in the `backend` crate.
    const MIGRATIONS_DIRECTORY: &'static str = "../db/migrations";

    let migrations_dir: &Path = Path::new(MIGRATIONS_DIRECTORY);
    migrations::run_pending_migrations_in_directory(conn, migrations_dir, &mut ::std::io::sink())
        .expect("Could not run migrations.");
}

table! {
    pg_database (datname) {
        datname -> Text,
        datistemplate -> Bool,
    }
}

/// Convenience function used when dropping the database
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