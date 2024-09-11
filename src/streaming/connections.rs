#[derive(Debug, serde::Deserialize)]
pub struct Connection {
    source: String,
    target: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Connections {
    pub nodes: Vec<Connection>,
}

pub fn convert_connections_to_chain(connections: Vec<Connection>) -> Vec<String> {
    const LAST_TARGET: &str = "camera";
    let mut successor: String = LAST_TARGET.to_string();
    let mut chain: Vec<String> = vec![];

    loop {
        match find_preceding(&connections, successor.clone()) {
            Some(preceding) => {
                successor = preceding.clone();
                chain.push(preceding);
            }
            None => break,
        }
    }
    chain.reverse();
    chain
}

fn find_preceding(connections: &Vec<Connection>, successor: String) -> Option<String> {
    for conn in connections.iter() {
        if conn.target == successor {
            return Some(conn.source.clone());
        }
    }
    None
}
