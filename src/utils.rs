use tungstenite::{connect, Message};
use url::Url;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub category: String,
    pub action: String,
    pub r2lab_state: Vec<R2LabState>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct R2LabState {
    #[serde(rename = "gnuradio_release")]
    pub gnuradio_release: String,
    pub uname: String,
    #[serde(rename = "usrp_on_off")]
    pub usrp_on_off: String,
    #[serde(rename = "os_release")]
    pub os_release: String,
    #[serde(rename = "control_ping")]
    pub control_ping: String,
    #[serde(rename = "cmc_on_off")]
    pub cmc_on_off: String,
    #[serde(rename = "image_radical")]
    pub image_radical: String,
    pub id: i64,
    pub available: Option<String>,
    #[serde(rename = "control_ssh")]
    pub control_ssh: String,
    #[serde(rename = "images_usrp")]
    #[serde(default)]
    pub images_usrp: Vec<::serde_json::Value>,
    #[serde(rename = "images_wifi")]
    pub images_wifi: Option<Vec<String>>,
    #[serde(rename = "usrp_type")]
    pub usrp_type: Option<String>,
    #[serde(rename = "usrp_duplexer")]
    pub usrp_duplexer: Option<String>,
    #[serde(default)]
    pub images: Vec<String>,
}

pub fn list_of_images(){

    let (mut socket, response) =
        connect(Url::parse("wss://r2lab.inria.fr:999").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    socket.write_message(Message::text("{category: \"nodes\", action: \"request\", message: \"please\"}").into()).unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}


//wss://r2lab.inria.fr:999
