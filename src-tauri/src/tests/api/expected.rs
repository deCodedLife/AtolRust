pub const CORRECT_REQUESTS: &'static [&'static str] = &[
    "{
        \"object\": \"atol\",
        \"command\": \"get-transactions\",
        \"data\": {
            \"cashbox_id\": \"1_yashlek\",
            \"sale_id\": 1027510
        },
        \"jwt\": \"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjIiLCJpcCI6IjE3Mi43MS4xMDMuMTM4IiwiZW1haWwiOiJzdXBwb3J0QG94Ym94LnJ1Iiwicm9sZV9pZCI6IjEifQ.NTS5D60KmKJy5uvBOTjiXN9v9Gan3sgSrUBvV3uPNA0\"
    }",
    "{
        \"object\": \"atol\",
        \"command\": \"get-transactions\",
        \"data\": {},
        \"jwt\": \"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjIiLCJpcCI6IjE3Mi43MS4xMDMuMTM4IiwiTjiXN9v9Gan3sgSrUBvV3uPNA0\"
    }",
    "{
        \"object\": \"atol\",
        \"command\": \"get-transactions\",
        \"data\": {
            \"cashbox_id\": \"1_yashlek\",
            \"sale_id\": 1027510
        }
    }"
];