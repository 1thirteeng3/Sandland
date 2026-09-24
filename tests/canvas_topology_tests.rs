use sandland_lib::domain::workspace::topology::{BoardTopology, CanvasEdge, CanvasNode, NodeSide};

#[test]
fn test_topology_add_edge_and_reject_self_loop() {
    let mut topology = BoardTopology::new("ws-1".to_string());

    let node1 = CanvasNode {
        id: "node-1".to_string(),
        item_id: None,
        local_cell_path: None,
        title: Some("Node 1".to_string()),
        content: Some("Content 1".to_string()),
        x: 100.0,
        y: 100.0,
        width: 280.0,
        height: 160.0,
        color_preset: None,
        node_type: Some("note".to_string()),
    };

    let node2 = CanvasNode {
        id: "node-2".to_string(),
        item_id: None,
        local_cell_path: None,
        title: Some("Node 2".to_string()),
        content: Some("Content 2".to_string()),
        x: 400.0,
        y: 100.0,
        width: 280.0,
        height: 160.0,
        color_preset: None,
        node_type: Some("note".to_string()),
    };

    topology.nodes.push(node1);
    topology.nodes.push(node2);

    // 1. Tentar adicionar auto-loop (deve falhar)
    let self_loop_edge = CanvasEdge {
        id: "edge-self".to_string(),
        source_node_id: "node-1".to_string(),
        target_node_id: "node-1".to_string(),
        from_side: NodeSide::Right,
        to_side: NodeSide::Left,
        label: None,
        directed: true,
    };
    assert!(topology.add_edge(self_loop_edge).is_err(), "Auto-loop deveria ser rejeitado");

    // 2. Adicionar aresta válida
    let valid_edge = CanvasEdge {
        id: "edge-1".to_string(),
        source_node_id: "node-1".to_string(),
        target_node_id: "node-2".to_string(),
        from_side: NodeSide::Right,
        to_side: NodeSide::Left,
        label: Some("relacionado".to_string()),
        directed: true,
    };
    assert!(topology.add_edge(valid_edge).is_ok());
    assert_eq!(topology.edges.len(), 1);
    assert!(topology.validate().is_ok());
}

#[test]
fn test_topology_cascading_edge_deletion() {
    let mut topology = BoardTopology::new("ws-1".to_string());

    let node1 = CanvasNode {
        id: "node-A".to_string(),
        item_id: None,
        local_cell_path: None,
        title: Some("Node A".to_string()),
        content: None,
        x: 0.0,
        y: 0.0,
        width: 200.0,
        height: 100.0,
        color_preset: None,
        node_type: None,
    };

    let node2 = CanvasNode {
        id: "node-B".to_string(),
        item_id: None,
        local_cell_path: None,
        title: Some("Node B".to_string()),
        content: None,
        x: 300.0,
        y: 0.0,
        width: 200.0,
        height: 100.0,
        color_preset: None,
        node_type: None,
    };

    let node3 = CanvasNode {
        id: "node-C".to_string(),
        item_id: None,
        local_cell_path: None,
        title: Some("Node C".to_string()),
        content: None,
        x: 600.0,
        y: 0.0,
        width: 200.0,
        height: 100.0,
        color_preset: None,
        node_type: None,
    };

    topology.nodes.extend(vec![node1, node2, node3]);

    let edge1 = CanvasEdge {
        id: "e1".to_string(),
        source_node_id: "node-A".to_string(),
        target_node_id: "node-B".to_string(),
        from_side: NodeSide::Right,
        to_side: NodeSide::Left,
        label: None,
        directed: true,
    };

    let edge2 = CanvasEdge {
        id: "e2".to_string(),
        source_node_id: "node-B".to_string(),
        target_node_id: "node-C".to_string(),
        from_side: NodeSide::Right,
        to_side: NodeSide::Left,
        label: None,
        directed: true,
    };

    topology.add_edge(edge1).unwrap();
    topology.add_edge(edge2).unwrap();
    assert_eq!(topology.edges.len(), 2);

    // Deletar node-B deve remover tanto e1 (onde B era target) quanto e2 (onde B era source)
    topology.delete_node("node-B");
    assert_eq!(topology.nodes.len(), 2);
    assert_eq!(topology.edges.len(), 0, "Todas as arestas associadas a node-B devem ser removidas em cascata");
}
