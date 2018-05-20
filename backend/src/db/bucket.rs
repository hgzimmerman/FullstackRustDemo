use schema::buckets;
use schema::junction_bucket_users;
use db::Conn;
use db::user::User;
use std::ops::Deref;
use error::JoeResult;
use diesel::prelude::*;
use diesel;

#[derive(Debug, Clone, Identifiable, Queryable, Crd, ErrorHandler)]
#[insertable = "NewBucket"]
#[table_name = "buckets"]
pub struct Bucket {
    /// Primary Key.
    pub id: i32,
    /// The name of the bucket
    pub bucket_name: String,
    /// The is public field indicates if the bucket will allow other users to request to join.
    pub is_public: bool
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "buckets"]
pub struct NewBucket {
    pub bucket_name: String,
    pub is_public: bool
}


#[derive(Debug, Clone, Identifiable, Queryable)]
#[table_name = "junction_bucket_users"]
pub struct BucketUser {
    pub id: i32,
    pub bucket_id: i32,
    pub user_id: i32,
    pub owner: bool,
    pub approved: bool
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "junction_bucket_users"]
pub struct NewBucketUser {
    pub bucket_id: i32,
    pub user_id: i32,
    pub owner: bool,
    pub approved: bool
}

#[derive(AsChangeset, Clone, Debug, PartialEq)]
#[table_name = "junction_bucket_users"]
pub struct BucketUserChangeset {
    pub id: i32,
    pub owner: bool,
    pub approved: bool
}


impl Bucket {

    pub fn get_public_buckets(conn: &Conn) -> JoeResult<Vec<Bucket>> {
        use schema::buckets::dsl::*;

        buckets
            .filter(is_public.eq(true))
            .load::<Bucket>(conn.deref())
            .map_err(User::handle_error)
    }

    pub fn add_user_to_bucket(new_bucket_user: NewBucketUser, conn: &Conn) -> JoeResult<()> {
        use schema::junction_bucket_users;

        diesel::insert_into(junction_bucket_users::table)
            .values(&new_bucket_user)
            .execute(conn.deref())
            .map_err(Bucket::handle_error)?;
        Ok(())
    }

    pub fn get_buckets_user_belongs_to(m_user_id: i32, conn: &Conn) -> JoeResult<Vec<Bucket>> {
        use schema::junction_bucket_users::dsl::*;
        // use schema::users::dsl::*;
        use schema::users;

        junction_bucket_users
            .filter(user_id.eq(m_user_id))
            .filter(approved.eq(true))
            .inner_join(buckets::table)
            .select(buckets::all_columns)
            .load::<Bucket>(conn.deref())
            .map_err(User::handle_error)
    }

    /// Helper function.
    /// Gets users depending on the approval column.
    fn get_users_approval(m_bucket_id: i32, approval: bool, conn: &Conn) -> JoeResult<Vec<User>> {
        use schema::junction_bucket_users::dsl::*;
        // use schema::users::dsl::*;
        use schema::users;

        junction_bucket_users
            .filter(bucket_id.eq(m_bucket_id))
            .filter(approved.eq(approval))
            .inner_join(users::table)
            .select(users::all_columns)
            .load::<User>(conn.deref())
            .map_err(Bucket::handle_error)
    }


    pub fn get_users_with_approval(m_bucket_id: i32, conn: &Conn) -> JoeResult<Vec<User>> {
        Self::get_users_approval(m_bucket_id, true, conn)
    }
    pub fn get_users_requiring_approval(m_bucket_id: i32, conn: &Conn) -> JoeResult<Vec<User>> {
        Self::get_users_approval(m_bucket_id, false, conn)
    }

    /// Is the user the owner of the bucket
    pub fn is_user_owner(m_user_id: i32, conn: &Conn) -> bool {
        use schema::junction_bucket_users::dsl::*;

        junction_bucket_users
            .filter(user_id.eq(m_user_id))
            .select(owner)
            .first::<bool>(conn.deref())
            .unwrap_or(false)
    }

    /// Is the user in the bucket, and approved by a bucket owner?
    pub fn is_user_approved(m_user_id: i32, conn: &Conn) -> bool {
        use schema::junction_bucket_users::dsl::*;

        junction_bucket_users
            .filter(user_id.eq(m_user_id))
            .select(approved)
            .first::<bool>(conn.deref())
            .unwrap_or(false)
    }

    pub fn apply_changeset(changeset: BucketUserChangeset, conn: &Conn) -> JoeResult<BucketUser> {
        use schema::junction_bucket_users;

        diesel::update(junction_bucket_users::table)
            .set(&changeset)
            .get_result(conn.deref())
            .map_err(Bucket::handle_error)
    }

     pub fn set_bucket_publicity(m_bucket_id: i32, publicity: bool, conn: &Conn) -> JoeResult<()> {

        use schema::buckets::dsl::*;
        let target = buckets
            .filter(id.eq(m_bucket_id));

        diesel::update(target)
            .set(is_public.eq(publicity))
            .execute(conn.deref())
            .map_err(Bucket::handle_error)?;

        Ok(())
    }

    pub fn set_user_approval(m_user_id: i32, m_bucket_id: i32, approval: bool, conn: &Conn) -> JoeResult<()> {

        use schema::junction_bucket_users::dsl::*;
        let target = junction_bucket_users
            .filter(user_id.eq(m_user_id))
            .filter(bucket_id.eq(m_bucket_id));

        diesel::update(target)
            .set(approved.eq(approval))
            .execute(conn.deref())
            .map_err(Bucket::handle_error)?;

        Ok(())
    }

    pub fn remove_user_from_bucket(m_user_id: i32, m_bucket_id: i32, conn: &Conn) -> JoeResult<()> {
        use schema::junction_bucket_users::dsl::*;
        use schema::junction_bucket_users;

        diesel::delete(junction_bucket_users::table)
            .filter(bucket_id.eq(m_bucket_id))
            .filter(user_id.eq(m_user_id))
            .execute(conn.deref())
            .map_err(Bucket::handle_error)?;
        Ok(())
    }

}
