use chrono::{
//    Duration,
    NaiveDateTime,
//    Utc,
};
use crate::{
    calls::prelude::*,
    schema::{
        self,
        buckets,
        junction_bucket_users,
    },
    user::User,
};
use diesel::{
    self,
    prelude::*,
};
use error::BackendResult;
use identifiers::{
    bucket::BucketUuid,
    user::UserUuid,
};
use uuid::Uuid;

#[derive(Debug, Clone, Identifiable, Queryable, TypeName)]
#[primary_key(uuid)]
#[table_name = "buckets"]
pub struct Bucket {
    /// Primary Key.
    pub uuid: Uuid,
    /// The name of the bucket
    pub bucket_name: String,
    /// The is public field indicates if the bucket will allow other users to request to join
    /// A None variant indicates that it is private, if it is a Some with a time in the future, it is public.
    pub is_public_until: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "buckets"]
pub struct NewBucket {
    pub bucket_name: String,
    pub is_public_until: Option<NaiveDateTime>,
}

/// A junction table between buckets and users.
/// It also encodes if the user is allowed to access the bucket,
/// as well as if the user is responsible for the bucket.
#[derive(Debug, Clone, Identifiable, Queryable)]
#[table_name = "junction_bucket_users"]
pub struct BucketUser {
    pub id: Uuid,
    pub bucket_id: Uuid,
    pub user_id: Uuid,
    pub owner: bool,
//    pub approved: bool,
}

/// A new entry into the bucket <-> user junction table.
#[derive(Insertable, Debug, Clone)]
#[table_name = "junction_bucket_users"]
pub struct NewBucketUser {
    pub bucket_uuid: Uuid,
    pub user_uuid: Uuid,
    pub owner: bool,
//    pub approved: bool,
}

#[derive(AsChangeset, Clone, Debug, PartialEq)]
#[primary_key(uuid)]
#[table_name = "junction_bucket_users"]
pub struct BucketUserChangeset {
    pub uuid: Uuid,
    pub owner: bool,
//    pub approved: bool,
}

#[derive(Debug, Clone)]
pub struct UsersInBucketData {
    pub bucket: Bucket,
    pub users: Vec<User>,
}

impl Bucket {
    pub fn get_bucket(uuid: BucketUuid, conn: &PgConnection) -> BackendResult<Bucket> {
        get_row::<Bucket, _>(schema::buckets::table, uuid.0, conn)
    }
    pub fn delete_bucket(uuid: BucketUuid, conn: &PgConnection) -> BackendResult<Bucket> {
        delete_row::<Bucket, _>(schema::buckets::table, uuid.0, conn)
    }
    pub fn create_bucket(new: NewBucket, conn: &PgConnection) -> BackendResult<Bucket> {
        create_row::<Bucket, NewBucket, _>(schema::buckets::table, new, conn)
    }

    pub fn get_bucket_by_name(bucket_name: String, conn: &PgConnection) -> BackendResult<Bucket> {
        use crate::schema::buckets::dsl::{
            bucket_name as table_bucket_name,
            buckets,
        };

        buckets
            .filter(table_bucket_name.eq(bucket_name))
            .first::<Bucket>(conn)
            .map_err(handle_err::<Bucket>)
    }

    pub fn get_buckets_user_owns(user_uuid: UserUuid, conn: &PgConnection) -> BackendResult<Vec<Bucket>> {
        use crate::schema::{
            buckets::dsl::{
//                bucket_name as table_bucket_name,
                buckets,
                uuid as table_bucket_uuid,
            },
            junction_bucket_users::dsl::{
                bucket_uuid as junction_bucket_uuid,
                junction_bucket_users,
                owner as junction_owner,
                user_uuid as junction_user_uuid,
            },
        };
        let bucket_uuids: Vec<Uuid> = junction_bucket_users
            .filter(junction_user_uuid.eq(user_uuid.0))
            .filter(junction_owner.eq(true))
            .select(junction_bucket_uuid)
            .load::<Uuid>(conn)
            .map_err(handle_err::<Bucket>)?;

        buckets
            .filter(table_bucket_uuid.eq_any(bucket_uuids))
            .load::<Bucket>(conn)
            .map_err(handle_err::<Bucket>)
    }

//    /// Get buckets that are public, but the user is not a member of
//    #[deprecated]
//    pub fn get_public_buckets(user_uuid: UserUuid, conn: &PgConnection) -> BackendResult<Vec<Bucket>> {
//        use crate::schema::{
//            buckets::{
//                self,
//                dsl::*,
//            },
//            junction_bucket_users as junctions,
//            junction_bucket_users::dsl::junction_bucket_users,
//        };
//
//        // Don't return any buckets with these ids
//        let bucket_uuids_in_which_the_user_is_already_a_member_or_has_requested_to_join: Vec<Uuid> =
//            junction_bucket_users
//                .filter(junctions::user_uuid.eq(user_uuid.0))
//                .select(junctions::bucket_uuid)
//                .load::<Uuid>(conn)
//                .map_err(handle_err::<User>)?;
//
//        buckets
//            .filter(is_public_until.gt(Utc::now().naive_utc())) // Get buckets with an expiry date in the future.
//            .filter(buckets::uuid.ne_all(bucket_uuids_in_which_the_user_is_already_a_member_or_has_requested_to_join)) // If buckets are in this set, don't return them
//            .select(buckets::all_columns)
//            .load::<Bucket>(conn)
//            .map_err(handle_err::<User>)
//    }

    /// If a user joins a bucket, then they will be added to the bucket.
    pub fn add_user_to_bucket(new_bucket_user: NewBucketUser, conn: &PgConnection) -> BackendResult<()> {
        use crate::schema::junction_bucket_users;

        diesel::insert_into(junction_bucket_users::table)
            .values(&new_bucket_user)
            .execute(conn)
            .map_err(handle_err::<Bucket>)?;
        Ok(())
    }

    /// If a user has visited a bucket before, the bucket name is saved for easy access via results returned from this call.
    pub fn get_buckets_user_belongs_to(user_uuid: UserUuid, conn: &PgConnection) -> BackendResult<Vec<Bucket>> {
        use crate::schema::{
            junction_bucket_users as junctions,
            junction_bucket_users::dsl::junction_bucket_users,
        };

        junction_bucket_users
            .filter(junctions::user_uuid.eq(user_uuid.0))
            .inner_join(buckets::table)
            .select(buckets::all_columns)
            .load::<Bucket>(conn)
            .map_err(handle_err::<User>)
    }

//    /// Helper function.
//    /// Gets users depending on the approval column.
//    /// It will exclude the user making the request.
//    #[deprecated]
//    fn get_users_approval(
//        bucket_uuid: BucketUuid,
//        user_uuid: UserUuid,
//        approval: bool,
//        conn: &PgConnection,
//    ) -> BackendResult<Vec<User>> {
//        use crate::schema::{
//            junction_bucket_users as junctions,
//            junction_bucket_users::dsl::junction_bucket_users,
//            users,
//        };
//
//        junction_bucket_users
//            .filter(junctions::bucket_uuid.eq(bucket_uuid.0))
//            .filter(junctions::approved.eq(approval))
//            .filter(junctions::user_uuid.ne(user_uuid.0))
//            .inner_join(users::table)
//            .select(users::all_columns)
//            .load::<User>(conn)
//            .map_err(handle_err::<Bucket>)
//    }

//    /// This function gets all players that are part of the bucket,
//    /// excluding the active user
//    #[deprecated]
//    pub fn get_users_with_approval(
//        bucket_uuid: BucketUuid,
//        user_uuid: UserUuid,
//        conn: &PgConnection,
//    ) -> BackendResult<Vec<User>> {
//        Self::get_users_approval(bucket_uuid, user_uuid, true, conn)
//    }

//    #[deprecated]
//    fn get_users_requiring_approval(
//        bucket_uuid: BucketUuid,
//        user_uuid: UserUuid,
//        conn: &PgConnection,
//    ) -> BackendResult<Vec<User>> {
//        Self::get_users_approval(bucket_uuid, user_uuid, false, conn)
//    }
//
//    #[deprecated]
//    pub fn get_users_requiring_approval_for_owned_buckets(
//        bucket_owner_uuid: UserUuid,
//        conn: &PgConnection,
//    ) -> BackendResult<Vec<UsersInBucketData>> {
//        use crate::schema::{
//            buckets,
//            junction_bucket_users as junctions,
//            junction_bucket_users::dsl::*,
//        };
//
//        let buckets: Vec<Bucket> = junction_bucket_users
//            .filter(junctions::user_uuid.eq(bucket_owner_uuid.0))
//            .filter(owner.eq(true))
//            .inner_join(buckets::table)
//            .select(buckets::all_columns)
//            .load::<Bucket>(conn)
//            .map_err(handle_err::<Bucket>)?;
//
//        // This is an ineffecient query. Its time will scale linearly (with a high constant) with the number of buckets the user owns.
//        let bucket_users = buckets
//            .into_iter()
//            .filter_map(|bucket| {
//                if let Ok(users) = Self::get_users_requiring_approval(BucketUuid(bucket.uuid), bucket_owner_uuid, conn)
//                {
//                    Some(UsersInBucketData { bucket, users })
//                } else {
//                    None
//                }
//            })
//            .collect();
//        Ok(bucket_users)
//    }

    /// Is the user the owner of the bucket
    pub fn is_user_owner(user_uuid: UserUuid, bucket_uuid: BucketUuid, conn: &PgConnection) -> bool {
        use crate::schema::{
            junction_bucket_users as junctions,
            junction_bucket_users::dsl::junction_bucket_users,
        };

        junction_bucket_users
            .filter(junctions::user_uuid.eq(user_uuid.0))
            .filter(junctions::bucket_uuid.eq(bucket_uuid.0))
            .select(junctions::owner)
            .first::<bool>(conn)
            .unwrap_or(false)
        //            .map_err(Bucket::handle_error)
    }

//
//    #[deprecated]
//    pub fn is_user_approved(user_uuid: UserUuid, bucket_uuid: BucketUuid, conn: &PgConnection) -> bool {
//        use crate::schema::{
//            junction_bucket_users as junctions,
//            junction_bucket_users::dsl::junction_bucket_users,
//        };
//
//        junction_bucket_users
//            .filter(junctions::user_uuid.eq(user_uuid.0))
//            .filter(junctions::bucket_uuid.eq(bucket_uuid.0))
//            .select(junctions::approved)
//            .first::<bool>(conn)
//            .unwrap_or(false)
//    }

    #[deprecated]
    pub fn apply_changeset(changeset: BucketUserChangeset, conn: &PgConnection) -> BackendResult<BucketUser> {
        use crate::schema::junction_bucket_users;

        diesel::update(junction_bucket_users::table)
            .set(&changeset)
            .get_result(conn)
            .map_err(handle_err::<Bucket>)
    }

//    #[deprecated]
//    pub fn set_bucket_publicity(bucket_uuid: BucketUuid, publicity: bool, conn: &PgConnection) -> BackendResult<()> {
//        use crate::schema::buckets::{
//            self,
//            dsl::*,
//        };
//
//        let target = buckets.filter(buckets::uuid.eq(bucket_uuid.0));
//
//        let expire_time: Option<NaiveDateTime> = if publicity {
//            Some(Utc::now().naive_utc() + Duration::days(1))
//        } else {
//            None
//        };
//
//        diesel::update(target)
//            .set(buckets::is_public_until.eq(expire_time))
//            .execute(conn)
//            .map_err(handle_err::<Bucket>)?;
//
//        Ok(())
//    }

//    #[deprecated]
//    pub fn set_user_approval(
//        user_uuid: UserUuid,
//        bucket_uuid: BucketUuid,
//        approval: bool,
//        conn: &PgConnection,
//    ) -> BackendResult<()> {
//        use crate::schema::{
//            junction_bucket_users as junctions,
//            junction_bucket_users::dsl::junction_bucket_users,
//        };
//
//        let target = junction_bucket_users
//            .filter(junctions::user_uuid.eq(user_uuid.0))
//            .filter(junctions::bucket_uuid.eq(bucket_uuid.0));
//
//        diesel::update(target)
//            .set(junctions::approved.eq(approval))
//            .execute(conn)
//            .map_err(handle_err::<Bucket>)?;
//
//        Ok(())
//    }

    /// Removes the user from the junction table for the given bucket.
    /// This has the effect of denying any request to join the bucket, as well as kicking a user out of the bucket.

    #[deprecated]
    pub fn remove_user_from_bucket(
        user_uuid: UserUuid,
        bucket_uuid: BucketUuid,
        conn: &PgConnection,
    ) -> BackendResult<()> {
        use crate::schema::junction_bucket_users as junctions;

        diesel::delete(junctions::table)
            .filter(junctions::bucket_uuid.eq(bucket_uuid.0))
            .filter(junctions::user_uuid.eq(user_uuid.0))
            .execute(conn)
            .map_err(handle_err::<Bucket>)?;
        Ok(())
    }
}
