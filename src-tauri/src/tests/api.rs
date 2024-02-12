use crate::api::DocaRequest;

#[test]
fn request_serialization() {

    let mut correct: Vec<String> = Vec::new();
    correct.push( "{
        \"object\": \"atol\",
        \"command\": \"get-transactions\",
        \"data\": {
            \"cashbox_id\": \"1_yashlek\",
            \"sale_id\": 1027510
        },
        \"jwt\": \"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjIiLCJpcCI6IjE3Mi43MS4xMDMuMTM4IiwiZW1haWwiOiJzdXBwb3J0QG94Ym94LnJ1Iiwicm9sZV9pZCI6IjEifQ.NTS5D60KmKJy5uvBOTjiXN9v9Gan3sgSrUBvV3uPNA0\"
    }".to_string() );

    correct.push( "{
        \"object\": \"atol\",
        \"command\": \"get-transactions\",
        \"data\": {},
        \"jwt\": \"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjIiLCJpcCI6IjE3Mi43MS4xMDMuMTM4IiwiTjiXN9v9Gan3sgSrUBvV3uPNA0\"
    }".to_string() );

    correct.push( "{
        \"object\": \"atol\",
        \"command\": \"get-transactions\",
        \"data\": {
            \"cashbox_id\": \"1_yashlek\",
            \"sale_id\": 1027510
        }
    }".to_string() );

    for request in correct {
        let _: DocaRequest = serde_json::from_str(&request).expect( "Cant't desirialize api request" );
    }

    let mut incorrect: Vec<String> = Vec::new();

    incorrect.push(  "{
        \"command\": \"get-transactions\",
        \"data\": {
            \"cashbox_id\": \"1_yashlek\",
            \"sale_id\": 1027510
        },
        \"jwt\": \"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjIiLCJpcCI6IjE3Mi43MS4xMDMuMTM4IiwiZW1haWwiOiJzdXBwb3J0QG94Ym94LnJ1Iiwicm9sZV9pZCI6IjEifQ.NTS5D60KmKJy5uvBOTjiXN9v9Gan3sgSrUBvV3uPNA0\"
    }".to_string() );

    incorrect.push(  "{
        \"object\": \"atol\",
        \"data\": {
            \"cashbox_id\": \"1_yashlek\",
            \"sale_id\": 1027510
        },
        \"jwt\": \"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjIiLCJpcCI6IjE3Mi43MS4xMDMuMTM4IiwiZW1haWwiOiJzdXBwb3J0QG94Ym94LnJ1Iiwicm9sZV9pZCI6IjEifQ.NTS5D60KmKJy5uvBOTjiXN9v9Gan3sgSrUBvV3uPNA0\"
    }".to_string() );

    incorrect.push(  "{
        \"object\": \"atol\",
        \"command\": \"get-transactions\",
        \"jwt\": \"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjIiLCJpcCI6IjE3Mi43MS4xMDMuMTM4IiwiZW1haWwiOiJzdXBwb3J0QG94Ym94LnJ1Iiwicm9sZV9pZCI6IjEifQ.NTS5D60KmKJy5uvBOTjiXN9v9Gan3sgSrUBvV3uPNA0\"
    }".to_string() );

    incorrect.push(  "{}".to_string() );

    incorrect.push(  "{
        \"object\": \"atol\",
        \"command\": \"get-transactions\",
        \"data\": {
            \"cashbox_id\": \"1_yashlek\",
            \"sale_id\": 1027510
        },
    }".to_string() );

    for request in incorrect {
        assert_eq!( false, serde_json::from_str::<DocaRequest>(&request).is_ok() );
    }

}