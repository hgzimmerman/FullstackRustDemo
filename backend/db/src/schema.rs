table! {
    answers (uuid) {
        uuid -> Uuid,
        question_uuid -> Uuid,
        author_uuid -> Uuid,
        answer_text -> Nullable<Varchar>,
    }
}

table! {
    articles (uuid) {
        uuid -> Uuid,
        author_uuid -> Uuid,
        title -> Varchar,
        slug -> Varchar,
        body -> Text,
        publish_date -> Nullable<Timestamp>,
    }
}

table! {
    buckets (uuid) {
        uuid -> Uuid,
        bucket_name -> Varchar,
        is_public_until -> Nullable<Timestamp>,
    }
}

table! {
    chats (uuid) {
        uuid -> Uuid,
        chat_name -> Varchar,
        leader_uuid -> Uuid,
    }
}

table! {
    forums (uuid) {
        uuid -> Uuid,
        title -> Varchar,
        description -> Varchar,
    }
}

table! {
    junction_bucket_users (uuid) {
        uuid -> Uuid,
        bucket_uuid -> Uuid,
        user_uuid -> Uuid,
        owner -> Bool,
        approved -> Bool,
    }
}

table! {
    junction_chat_users (uuid) {
        uuid -> Uuid,
        chat_uuid -> Uuid,
        user_uuid -> Uuid,
    }
}

table! {
    messages (uuid) {
        uuid -> Uuid,
        author_uuid -> Uuid,
        chat_uuid -> Uuid,
        reply_uuid -> Nullable<Uuid>,
        message_content -> Varchar,
        read_flag -> Bool,
        create_date -> Timestamp,
    }
}

table! {
    post_downvotes (uuid) {
        uuid -> Uuid,
        post_uuid -> Uuid,
        user_uuid -> Uuid,
    }
}

table! {
    posts (uuid) {
        uuid -> Uuid,
        thread_uuid -> Uuid,
        author_uuid -> Uuid,
        parent_uuid -> Nullable<Uuid>,
        created_date -> Timestamp,
        modified_date -> Nullable<Timestamp>,
        content -> Varchar,
        censored -> Bool,
    }
}

table! {
    post_upvotes (uuid) {
        uuid -> Uuid,
        post_uuid -> Uuid,
        user_uuid -> Uuid,
    }
}

table! {
    questions (uuid) {
        uuid -> Uuid,
        bucket_uuid -> Uuid,
        author_uuid -> Uuid,
        question_text -> Varchar,
        on_floor -> Bool,
    }
}

table! {
    threads (uuid) {
        uuid -> Uuid,
        forum_uuid -> Uuid,
        author_uuid -> Uuid,
        created_date -> Timestamp,
        locked -> Bool,
        archived -> Bool,
        title -> Varchar,
    }
}

table! {
    users (uuid) {
        uuid -> Uuid,
        user_name -> Varchar,
        display_name -> Varchar,
        password_hash -> Varchar,
        locked -> Nullable<Timestamp>,
        failed_login_count -> Int4,
        banned -> Bool,
        roles -> Array<Int4>,
    }
}

joinable!(answers -> questions (question_uuid));
joinable!(answers -> users (author_uuid));
joinable!(articles -> users (author_uuid));
joinable!(chats -> users (leader_uuid));
joinable!(junction_bucket_users -> buckets (bucket_uuid));
joinable!(junction_bucket_users -> users (user_uuid));
joinable!(junction_chat_users -> chats (chat_uuid));
joinable!(junction_chat_users -> users (user_uuid));
joinable!(messages -> chats (chat_uuid));
joinable!(messages -> users (author_uuid));
joinable!(post_downvotes -> posts (post_uuid));
joinable!(post_downvotes -> users (user_uuid));
joinable!(post_upvotes -> posts (post_uuid));
joinable!(post_upvotes -> users (user_uuid));
joinable!(posts -> threads (thread_uuid));
joinable!(posts -> users (author_uuid));
joinable!(questions -> buckets (bucket_uuid));
joinable!(questions -> users (author_uuid));
joinable!(threads -> forums (forum_uuid));
joinable!(threads -> users (author_uuid));

allow_tables_to_appear_in_same_query!(
    answers,
    articles,
    buckets,
    chats,
    forums,
    junction_bucket_users,
    junction_chat_users,
    messages,
    post_downvotes,
    posts,
    post_upvotes,
    questions,
    threads,
    users,
);
