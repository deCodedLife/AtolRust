pub const INCORRECT_REQUESTS: &'static [&'static str] = & [
    "{
        \"command\": \"get-transactions\",
        \"data\": {
            \"cashbox_id\": \"1_yashlek\",
            \"sale_id\": 1027510
        },
        \"jwt\": \"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjIiLCJpcCI6IjE3Mi43MS4xMDMuMTM4IiwiZW1haWwiOiJzdXBwb3J0QG94Ym94LnJ1Iiwicm9sZV9pZCI6IjEifQ.NTS5D60KmKJy5uvBOTjiXN9v9Gan3sgSrUBvV3uPNA0\"
    }",
    "{
        \"object\": \"atol\",
        \"data\": {
            \"cashbox_id\": \"1_yashlek\",
            \"sale_id\": 1027510
        },
        \"jwt\": \"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjIiLCJpcCI6IjE3Mi43MS4xMDMuMTM4IiwiZW1haWwiOiJzdXBwb3J0QG94Ym94LnJ1Iiwicm9sZV9pZCI6IjEifQ.NTS5D60KmKJy5uvBOTjiXN9v9Gan3sgSrUBvV3uPNA0\"
    }",
    "{
        \"object\": \"atol\",
        \"command\": \"get-transactions\",
        \"jwt\": \"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjIiLCJpcCI6IjE3Mi43MS4xMDMuMTM4IiwiZW1haWwiOiJzdXBwb3J0QG94Ym94LnJ1Iiwicm9sZV9pZCI6IjEifQ.NTS5D60KmKJy5uvBOTjiXN9v9Gan3sgSrUBvV3uPNA0\"
    }",
    "{}",
    "{
        \"object\": \"atol\",
        \"command\": \"get-transactions\",
        \"data\": {
            \"cashbox_id\": \"1_yashlek\",
            \"sale_id\": 1027510
        },
    }"

];