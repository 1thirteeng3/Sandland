use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct NodeTest {
    id: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TopologyTest {
    workspace_id: String,
    nodes: Vec<NodeTest>,
}

#[test]
fn test_messagepack_json_parity() {
    let original = TopologyTest {
        workspace_id: "ws-test-01".to_string(),
        nodes: vec![
            NodeTest {
                id: "node-1".to_string(),
                x: 100.0,
                y: 200.0,
                width: 250.0,
                height: 140.0,
            },
        ],
    };

    // Serializa em MessagePack (mpk) de alta frequência
    let mpk_bytes = rmp_serde::to_vec(&original).expect("MessagePack serialization");
    let from_mpk: TopologyTest = rmp_serde::from_slice(&mpk_bytes).expect("MessagePack deserialization");
    assert_eq!(original, from_mpk);

    // Serializa em JSON legível
    let json_str = serde_json::to_string(&original).expect("JSON serialization");
    let from_json: TopologyTest = serde_json::from_str(&json_str).expect("JSON deserialization");
    assert_eq!(original, from_json);

    // Garante paridade semântica
    assert_eq!(from_mpk, from_json);
}
