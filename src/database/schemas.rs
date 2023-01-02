pub struct TableSchemas {
    pub users: String,
    pub monitoring_server: String,
    pub monitor: String,
}

impl TableSchemas {
    pub fn get_all() -> TableSchemas {
        TableSchemas {
            users: Self::user_schema(),
            monitor: Self::monitors_schema(),
            monitoring_server: Self::monitoring_server_schema()
        }
    }

    fn user_schema() -> String {
        "users (
            id BIGSERIAL,
            username TEXT,
            created_at DATE,
            monitoring_server_id BIGINT,
            monitors INTEGER[]
        );".to_string()
    }

    fn monitors_schema() -> String {
        "monitor (
            id BIGSERIAL NOT NULL,
            name TEXT NOT NULL
        );".to_string()
    }

    fn monitoring_server_schema() -> String {
        "monitoring_server (
            id BIGSERIAL,
            address TEXT,
            mprober_api_port BIGINT,
            auth_required BOOL,
            auth_key BIGINT
        );".to_string()
    }
}