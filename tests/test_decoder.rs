extern crate serde_json;
extern crate prometheus_api;

use prometheus_api::*;
use std::collections::BTreeMap;


#[test]
fn decode_vector() {
    let json = "{
       \"status\": \"success\",
       \"data\": {
          \"resultType\": \"vector\",
          \"result\": [
             {
                \"metric\": {
                   \"__name__\": \"http_requests_total\",
                   \"code\": \"200\",
                   \"method\": \"get\"
                },
                \"value\": [
                   1494690291.96,
                   \"47\"
                ]
             },
             {
                \"metric\": {
                   \"__name__\": \"http_requests_total\",
                   \"code\": \"200\",
                   \"method\": \"post\"
                },
                \"value\": [
                   1494690291.96,
                   \"1\"
                ]
             }
          ]
       }
    }";

    let resp: PromResponse = serde_json::from_str(json).unwrap();

    let mut metric_1 = BTreeMap::new();
    metric_1.insert("__name__".to_string(), "http_requests_total".to_string());
    metric_1.insert("code".to_string(), "200".to_string());
    metric_1.insert("method".to_string(), "get".to_string());

    let mut metric_2 = BTreeMap::new();
    metric_2.insert("__name__".to_string(), "http_requests_total".to_string());
    metric_2.insert("code".to_string(), "200".to_string());
    metric_2.insert("method".to_string(), "post".to_string());

    let exp = PromResponse {
        status: Status::Success,
        data: Data::Vector(vec![InstantVecItem {
                                    metric: metric_1,
                                    value: (1494690291.96, "47".to_string()),
                                },
                                InstantVecItem {
                                    metric: metric_2,
                                    value: (1494690291.96, "1".to_string()),
                                }]),
        error_type: None,
        error: None,
    };

    assert_eq!(resp, exp);
}

#[test]
fn decode_matrix() {
    let json = "
	{
	   \"status\":\"success\",
	   \"data\":{
		  \"resultType\":\"matrix\",
		  \"result\":[
			 {
				\"metric\":{
				   \"__name__\":\"http_requests_total\",
				   \"code\":\"200\",
				   \"method\":\"get\"
				},
				\"values\":[
				   [
					  1494697452.491,
					  \"10\"
				   ],
				   [
					  1494697467.491,
					  \"10\"
				   ],
				   [
					  1494697482.491,
					  \"10\"
				   ],
				   [
					  1494697497.491,
					  \"10\"
				   ]
				]
			 },
			 {
				\"metric\":{
				   \"__name__\":\"http_requests_total\",
				   \"code\":\"400\",
				   \"method\":\"get\"
				},
				\"values\":[
				   [
					  1494697452.491,
					  \"21\"
				   ],
				   [
					  1494697467.491,
					  \"21\"
				   ],
				   [
					  1494697482.491,
					  \"21\"
				   ],
				   [
					  1494697497.491,
					  \"21\"
				   ]
				]
			 }
		  ]
	   }
	}";

    let resp: PromResponse = serde_json::from_str(json).unwrap();

    let mut metric_1 = BTreeMap::new();
    metric_1.insert("__name__".to_string(), "http_requests_total".to_string());
    metric_1.insert("code".to_string(), "200".to_string());
    metric_1.insert("method".to_string(), "get".to_string());

    let mut metric_2 = BTreeMap::new();
    metric_2.insert("__name__".to_string(), "http_requests_total".to_string());
    metric_2.insert("code".to_string(), "400".to_string());
    metric_2.insert("method".to_string(), "get".to_string());

    let exp = PromResponse {
        status: Status::Success,
        data: Data::Matrix(vec![MatrixItem {
                                    metric: metric_1,
                                    values: vec![(1494697452.491, "10".to_string()),
                                                 (1494697467.491, "10".to_string()),
                                                 (1494697482.491, "10".to_string()),
                                                 (1494697497.491, "10".to_string())],
                                },
                                MatrixItem {
                                    metric: metric_2,
                                    values: vec![(1494697452.491, "21".to_string()),
                                                 (1494697467.491, "21".to_string()),
                                                 (1494697482.491, "21".to_string()),
                                                 (1494697497.491, "21".to_string())],
                                }]),
        error_type: None,
        error: None,
    };

    assert_eq!(resp, exp);
}

#[test]
fn decode_scalar() {
    let json = "{
	   \"status\": \"success\",
	   \"data\": {
		  \"resultType\": \"scalar\",
		  \"result\": [
			 1494700723.206,
			 \"784\"
		  ]
	   }
	}";
    let resp: PromResponse = serde_json::from_str(json).unwrap();

    let exp = PromResponse {
        status: Status::Success,
        data: Data::Scalar((1494700723.206, "784".to_string())),
        error_type: None,
        error: None,
    };

    assert_eq!(resp, exp);
}
