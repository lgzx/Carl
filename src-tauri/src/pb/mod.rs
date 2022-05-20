pub mod abi;

use core::panic;
use std::collections::HashMap;

pub use abi::*;

use self::cmd::RequestCmd;

impl From<&str> for Config {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
}

impl From<&str> for Request {
    fn from(s: &str) -> Self {
        let req: HashMap<String, String> = serde_json::from_str(s).unwrap();
        let route = req.get("route").unwrap();
        let data = req.get("data").unwrap();

        Self {
            route: route.into(),
            data: data.into(),
        }
    }
}

impl From<bool> for Response {
    fn from(b: bool) -> Self {
        let mut res = Self {
            ..Default::default()
        };
        match b {
            true => res.status = "ok".to_string(),
            false => res.status = "error".to_string(),
        };
        res
    }
}

impl From<(&str, &str)> for Cmd {
    fn from(info: (&str, &str)) -> Self {
        let inner_cmd = match info.0 {
            "addconfig" => RequestCmd::Addconfig(AddConfig {
                cfg: Some(info.1.into()),
            }),
            _ => panic!("error parse cmd"),
        };
        Self {
            request_cmd: Some(inner_cmd),
        }
    }
}

impl From<Request> for Cmd {
    fn from(req: Request) -> Self {
        (&req.route[..], &req.data[..]).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_str_to_request() {
        let req = r#"{"route": "test", "data":""}"#;

        let request: Request = req.into();

        let excepted = Request {
            route: "test".to_string(),
            ..Default::default()
        };

        assert_eq!(request, excepted);
    }

    #[test]
    fn test_request_to_cmd() {
        let req = r#"{"route": "addconfig", "data":"{\"broker\":\"123\", \"topic\":\"ttt\"}"}"#;

        let request: Request = req.into();

        let cmd: Cmd = request.into();
        let excepcted: Cmd = Cmd {
            ..Default::default()
        };

        assert_eq!(cmd, excepcted);
    }
}
