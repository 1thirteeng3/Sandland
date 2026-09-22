#[test]
fn test_culling_bounding_box_efficiency() {
    let screen_width = 1920.0f32;
    let screen_height = 1080.0f32;
    let zoom = 1.0f32;
    let viewport_x = 0.0f32;
    let viewport_y = 0.0f32;

    let world_left = -viewport_x / zoom;
    let world_top = -viewport_y / zoom;
    let world_right = (screen_width - viewport_x) / zoom;
    let world_bottom = (screen_height - viewport_y) / zoom;

    let mut total_nodes = 0;
    let mut visible_nodes = 0;

    // Simula grid de 100+ nós (15x10 = 150 nós espalhados)
    for col in 0..15 {
        for row in 0..10 {
            total_nodes += 1;
            let x = (col * 300) as f32;
            let y = (row * 200) as f32;
            let width = 250.0f32;
            let height = 150.0f32;

            let is_visible = x + width >= world_left
                && x <= world_right
                && y + height >= world_top
                && y <= world_bottom;

            if is_visible {
                visible_nodes += 1;
            }
        }
    }

    assert_eq!(total_nodes, 150);
    assert!(visible_nodes > 0 && visible_nodes < total_nodes);
}
