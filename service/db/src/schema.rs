// @generated automatically by Diesel CLI.

diesel::table! {
    offline_cash_queue (id) {
        id -> Int8,
        created_on -> Timestamptz,
        updated_on -> Timestamptz,
        #[max_length = 256]
        from_did -> Text,
        #[max_length = 256]
        to_did -> Text,
        certificates -> Jsonb,
        transactions -> Jsonb,
        #[max_length = 256]
        tx_hash -> Text,
        #[max_length = 256]
        signed_tx_hash -> Text,
        status -> Bool,
    }
}
