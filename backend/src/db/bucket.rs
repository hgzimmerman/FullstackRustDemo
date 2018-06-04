use schema::buckets;
use schema::junction_bucket_users;
use db::user::User;
use error::JoeResult;
use diesel::prelude::*;
use diesel;
use chrono::{NaiveDateTime, Utc, Duration};
use uuid::Uuid;
use identifiers::bucket::BucketUuid;
use identifiers::user::UserUuid;

#[derive(Debug, Clone, Identifiable, Queryable, CrdUuid, ErrorHandler)]
#[insertable = "NewBucket"]
#[table_name = "buckets"]
pub struct Bucket {
    /// Primary Key.
    pub id: Uuid,
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
    pub approved: bool,
}

/// A new entry into the bucket <-> user junction table.
#[derive(Insertable, Debug, Clone)]
#[table_name = "junction_bucket_users"]
pub struct NewBucketUser {
    pub bucket_id: Uuid,
    pub user_id: Uuid,
    pub owner: bool,
    pub approved: bool,
}

#[derive(AsChangeset, Clone, Debug, PartialEq)]
#[table_name = "junction_bucket_users"]
pub struct BucketUserChangeset {
    pub id: Uuid,
    pub owner: bool,
    pub approved: bool,
}

#[derive(Debug, Clone)]
pub struct UsersInBucketData {
    pub bucket: Bucket,
    pub users: Vec<User>,
}


impl Bucket {
    /// Get buckets that are public, but the user is not a member of
    pub fn get_public_buckets(user_uuid: UserUuid, conn: &PgConnection) -> JoeResult<Vec<Bucket>> {
        use schema::buckets::dsl::*;
        use schema::buckets;
        use schema::junction_bucket_users as junctions;
        use schema::junction_bucket_users::dsl::*;
        //        use diesel::query_dsl::InternalJoinDsl;

        // Don't return any buckets with these ids
        let bucket_uuids_in_which_the_user_is_already_a_member_or_has_requested_to_join: Vec<Uuid> = junction_bucket_users
            .filter(junctions::user_id.eq(user_uuid.0))
            .select(junctions::bucket_id)
            .load::<Uuid>(conn)
            .map_err(User::handle_error)?;

        //        use log;
        //        log::info!("Dont include buckets: {:?}", bucket_ids_in_which_the_user_is_already_a_member_or_has_requested_to_join);
        //
        //        let all_junctions = junction_bucket_users.load::<BucketUser>(conn.deref()).map_err(User::handle_error)?;
        //        log::info!("All junctions: {:?}", all_junctions);


        buckets
            .filter(is_public_until.gt(Utc::now().naive_utc())) // Get buckets with an expiry date in the future.
            .filter(buckets::id.ne_all(bucket_uuids_in_which_the_user_is_already_a_member_or_has_requested_to_join)) // If buckets are in this set, don't return them
            .select(buckets::all_columns)
            .load::<Bucket>(conn)
            .map_err(User::handle_error)
    }

    pub fn add_user_to_bucket(new_bucket_user: NewBucketUser, conn: &PgConnection) -> JoeResult<()> {
        use schema::junction_bucket_users;

        diesel::insert_into(junction_bucket_users::table)
            .values(&new_bucket_user)
            .execute(conn)
            .map_err(Bucket::handle_error)?;
        Ok(())
    }

    pub fn get_buckets_user_belongs_to(user_uuid: UserUuid, conn: &PgConnection) -> JoeResult<Vec<Bucket>> {
        use schema::junction_bucket_users::dsl::*;
        // use schema::users::dsl::*;
        use schema::junction_bucket_users as junction;

        junction_bucket_users
            .filter(junction::user_id.eq(user_uuid.0))
            .filter(approved.eq(true))
            .inner_join(buckets::table)
            .select(buckets::all_columns)
            .load::<Bucket>(conn)
            .map_err(User::handle_error)
    }

    /// Helper function.
    /// Gets users depending on the approval column.
    fn get_users_approval(bucket_uuid: BucketUuid, approval: bool, conn: &PgConnection) -> JoeResult<Vec<User>> {
        use schema::junction_bucket_users::dsl::*;
        // use schema::users::dsl::*;
        use schema::users;

        let m_bucket_uuid: Uuid = bucket_uuid.0;

        junction_bucket_users
            .filter(bucket_id.eq(m_bucket_uuid))
            .filter(approved.eq(approval))
            .inner_join(users::table)
            .select(users::all_columns)
            .load::<User>(conn)
            .map_err(Bucket::handle_error)
    }


    /// This function gets all players that are part of the bucket, excluding the active user
    pub fn get_users_with_approval(bucket_uuid: BucketUuid, conn: &PgConnection) -> JoeResult<Vec<User>> {
        Self::get_users_approval(bucket_uuid, true, conn)
    }
    pub fn get_users_requiring_approval(bucket_uuid: BucketUuid, conn: &PgConnection) -> JoeResult<Vec<User>> {
        Self::get_users_approval(bucket_uuid, false, conn)
    }

    pub fn get_users_requiring_approval_for_owned_buckets(bucket_owner_id: UserUuid, conn: &PgConnection) -> JoeResult<Vec<UsersInBucketData>> {
        use schema::junction_bucket_users::dsl::*;
        // use schema::users::dsl::*;
        use schema::buckets;

        let buckets: Vec<Bucket> = junction_bucket_users
            .filter(user_id.eq(bucket_owner_id.0))
            .filter(owner.eq(true))
            .inner_join(buckets::table)
            .select(buckets::all_columns)
            .load::<Bucket>(conn)
            .map_err(Bucket::handle_error)?;

        // This is an ineffecient query. Its time will scale linearly (with a high constant) with the number of buckets the user owns.
        let bucket_users = buckets
            .into_iter()
            .filter_map(|bucket| if let Ok(users) = Self::get_users_requiring_approval(BucketUuid(bucket.id), conn) {
                Some(UsersInBucketData { bucket, users })
            } else {
                None
            })
            .collect();
        Ok(bucket_users)

    }

    /// Is the user the owner of the bucket
    pub fn is_user_owner(user_uuid: UserUuid, bucket_uuid: BucketUuid, conn: &PgConnection) -> JoeResult<bool> {
        use schema::junction_bucket_users::dsl::*;

        let m_bucket_uuid: Uuid = bucket_uuid.0;

        junction_bucket_users
            .filter(user_id.eq(user_uuid.0))
            .filter(bucket_id.eq(m_bucket_uuid))
            .select(owner)
            .first::<bool>(conn)
            .map_err(Bucket::handle_error)
    }

    /// Is the user in the bucket, and approved by a bucket owner?
    pub fn is_user_approved(user_uuid: UserUuid, bucket_uuid: BucketUuid, conn: &PgConnection) -> bool {
        use schema::junction_bucket_users::dsl::*;

        let m_bucket_uuid: Uuid = bucket_uuid.0;

        junction_bucket_users
            .filter(user_id.eq(user_uuid.0))
            .filter(bucket_id.eq(m_bucket_uuid))
            .select(approved)
            .first::<bool>(conn)
            .unwrap_or(false)
    }

    pub fn apply_changeset(changeset: BucketUserChangeset, conn: &PgConnection) -> JoeResult<BucketUser> {
        use schema::junction_bucket_users;

        diesel::update(junction_bucket_users::table)
            .set(&changeset)
            .get_result(conn)
            .map_err(Bucket::handle_error)
    }

    pub fn set_bucket_publicity(bucket_uuid: BucketUuid, publicity: bool, conn: &PgConnection) -> JoeResult<()> {
        use schema::buckets::dsl::*;
        use schema::buckets;

        let m_bucket_uuid: Uuid = bucket_uuid.0;

        let target = buckets.filter(
            buckets::id.eq(m_bucket_uuid),
        );

        let expire_time: Option<NaiveDateTime> = if publicity {
            Some(Utc::now().naive_utc() + Duration::days(1))
        } else {
            None
        };

        diesel::update(target)
            .set(is_public_until.eq(expire_time))
            .execute(conn)
            .map_err(Bucket::handle_error)?;

        Ok(())
    }

    pub fn set_user_approval(user_uuid: UserUuid, bucket_uuid: BucketUuid, approval: bool, conn: &PgConnection) -> JoeResult<()> {
        use schema::junction_bucket_users::dsl::*;

        let m_bucket_uuid: Uuid = bucket_uuid.0;

        let target = junction_bucket_users
            .filter(user_id.eq(user_uuid.0))
            .filter(bucket_id.eq(m_bucket_uuid));

        diesel::update(target)
            .set(approved.eq(approval))
            .execute(conn)
            .map_err(Bucket::handle_error)?;

        Ok(())
    }

    /// Removes the user from the junction table for the given bucket.
    /// This has the effect of denying any request to join the bucket, as well as kicking a user out of the bucket.
    pub fn remove_user_from_bucket(user_uuid: UserUuid, bucket_uuid: BucketUuid, conn: &PgConnection) -> JoeResult<()> {
        use schema::junction_bucket_users::dsl::*;
        use schema::junction_bucket_users;

        let m_bucket_uuid: Uuid = bucket_uuid.0;

        diesel::delete(junction_bucket_users::table)
            .filter(bucket_id.eq(m_bucket_uuid))
            .filter(user_id.eq(user_uuid.0))
            .execute(conn)
            .map_err(Bucket::handle_error)?;
        Ok(())
    }
}
