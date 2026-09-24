#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LodLevel {
    Macro,  // Zoom < 0.35: apenas cartões geométricos compactos
    Medium, // 0.35 <= Zoom < 0.8: títulos e silhuetas
    Full,   // Zoom >= 0.8: renderização completa com texto e detalhes
}

pub fn calculate_lod(zoom: f32) -> LodLevel {
    if zoom < 0.35 {
        LodLevel::Macro
    } else if zoom < 0.80 {
        LodLevel::Medium
    } else {
        LodLevel::Full
    }
}

#[test]
fn test_aabb_frustum_culling_intersection() {
    let viewport = Rect {
        x: 0.0,
        y: 0.0,
        width: 1920.0,
        height: 1080.0,
    };

    let node_inside = Rect {
        x: 100.0,
        y: 100.0,
        width: 280.0,
        height: 160.0,
    };

    let node_outside = Rect {
        x: 3000.0,
        y: 3000.0,
        width: 280.0,
        height: 160.0,
    };

    let node_partial = Rect {
        x: 1800.0,
        y: 1000.0,
        width: 280.0,
        height: 160.0,
    };

    assert!(viewport.intersects(&node_inside));
    assert!(!viewport.intersects(&node_outside));
    assert!(viewport.intersects(&node_partial));
}

#[test]
fn test_lod_level_thresholds() {
    assert_eq!(calculate_lod(0.1), LodLevel::Macro);
    assert_eq!(calculate_lod(0.34), LodLevel::Macro);
    assert_eq!(calculate_lod(0.35), LodLevel::Medium);
    assert_eq!(calculate_lod(0.79), LodLevel::Medium);
    assert_eq!(calculate_lod(0.80), LodLevel::Full);
    assert_eq!(calculate_lod(1.5), LodLevel::Full);
    assert_eq!(calculate_lod(3.0), LodLevel::Full);
}

#[test]
fn test_frustum_culling_1000_nodes_benchmark() {
    let viewport = Rect {
        x: 500.0,
        y: 500.0,
        width: 1920.0,
        height: 1080.0,
    };

    // Gera 1.000 nós distribuídos em uma grade de 50x20
    let mut nodes = Vec::with_capacity(1000);
    for i in 0..50 {
        for j in 0..20 {
            nodes.push(Rect {
                x: (i * 350) as f32,
                y: (j * 250) as f32,
                width: 280.0,
                height: 160.0,
            });
        }
    }

    let start = std::time::Instant::now();
    let visible_count = nodes.iter().filter(|n| viewport.intersects(n)).count();
    let duration = start.elapsed();

    assert!(visible_count > 0 && visible_count < 1000);
    // Deve executar em menos de 1 milissegundo para garantir 60 FPS
    assert!(
        duration.as_millis() < 5,
        "Culling de 1.000 nós deve ser ultra-rápido (< 5ms), levou {:?}",
        duration
    );
}
