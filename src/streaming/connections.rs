#[derive(Debug, serde::Deserialize)]
pub struct Connection {
    source: String,
    target: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Connections {
    pub nodes: Vec<Connection>,
}

/*
* connections = Vec({"3", "camera"}, {"2", "3"}, {"1", "2"});
* convert_connections_to_process_chain(connections) -> Vec("1", "2", "3");
*/
pub fn convert_connections_to_process_chain(connections: Vec<Connection>) -> Vec<String> {
    const LAST_TARGET: &str = "camera";
    let mut successor: String = LAST_TARGET.to_string();
    let mut process_chain: Vec<String> = vec![];

    loop {
        match find_preceding(&connections, successor.clone()) {
            Some(preceding) => {
                successor = preceding.clone();
                process_chain.push(preceding);
            }
            None => break,
        }
    }
    process_chain.reverse();
    process_chain
}

/*
* find_preceding(Vec({"abc", "def"}, {"123", "456"}), "123") -> Some("456")
* find_preceding(Vec({"abc", "def"}, {"123", "456"}), "xyz") -> None
*/
fn find_preceding(connections: &Vec<Connection>, successor: String) -> Option<String> {
    for conn in connections.iter() {
        if conn.target == successor {
            return Some(conn.source.clone());
        }
    }
    None
}
