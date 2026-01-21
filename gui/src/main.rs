use egui::{Stroke, Color32};
use egui::epaint::{CubicBezierShape, Pos2};


#[derive(Clone)]
struct Node {
    id: u32,
    pos: Pos2,
    dragging: bool,
}


struct Connection {
    from_node: u32,
    to_node: u32,
}

use engine::graph::Graph;
struct App {
    graph: Graph,
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    selected_node: Option<u32>,
    connecting_from: Option<u32>,
    dragging_node: Option<u32>,
    mouse_pos: egui::Pos2,
}

impl Default for App {
    fn default() -> Self {
        Self {
            graph: Graph::new(),
            nodes: vec![],             // <- initialize nodes
            connections: vec![],       // <- initialize connections
            selected_node: None,
            connecting_from: None,
            dragging_node: None,       // <- initialize dragging_node
            mouse_pos: egui::Pos2::ZERO,
        }
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.mouse_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());

            // Draw connections
            for conn in &self.connections {
                let from_node = self.nodes.iter().find(|n| n.id == conn.from_node).unwrap();
                let to_node = self.nodes.iter().find(|n| n.id == conn.to_node).unwrap();

                let node_width = 120.0;
                let node_height = 60.0;
                let circle_radius = 6.0;

                // Use the centers of the IO circles
                let start = from_node.pos + egui::vec2(node_width, node_height / 2.0); // output circle center
                let end = to_node.pos + egui::vec2(0.0, node_height / 2.0); // input circle center

                let ctrl1 = start + egui::vec2(50.0, 0.0);
                let ctrl2 = end - egui::vec2(50.0, 0.0);

                let bez = CubicBezierShape::from_points_stroke(
                    [start, ctrl1, ctrl2, end],
                    false,
                    Color32::TRANSPARENT,
                    Stroke::new(2.0, Color32::WHITE),
                );

                ui.painter().add(egui::Shape::CubicBezier(bez));
            }


            // Draw nodes and handle dragging
            for i in 0..self.nodes.len() {
                let node = &mut self.nodes[i];
                let node_width = 120.0;
                let node_height = 60.0;

                // Node rectangle
                let node_rect = egui::Rect::from_min_size(node.pos, egui::vec2(node_width, node_height));
                ui.painter().rect_filled(node_rect, 5.0, Color32::DARK_GRAY);

                // Node text
                ui.painter().text(
                    node.pos + egui::vec2(10.0, 10.0),
                    egui::Align2::LEFT_TOP,
                    format!("Node {}", node.id),
                    egui::TextStyle::Button.resolve(ui.style()),
                    Color32::WHITE,
                );

                // Input circle (left)
                let circle_radius = 6.0;
                let input_pos = node.pos + egui::vec2(0.0, node_height / 2.0);
                let input_rect = egui::Rect::from_center_size(input_pos, egui::vec2(circle_radius*2.0, circle_radius*2.0));
                ui.painter().circle_filled(input_pos, circle_radius, Color32::LIGHT_BLUE);
                let input_response = ui.allocate_rect(input_rect, egui::Sense::click());

                // Output circle (right)
                let output_pos = node.pos + egui::vec2(node_width, node_height / 2.0);
                let output_rect = egui::Rect::from_center_size(output_pos, egui::vec2(circle_radius*2.0, circle_radius*2.0));
                ui.painter().circle_filled(output_pos, circle_radius, Color32::LIGHT_GREEN);
                let output_response = ui.allocate_rect(output_rect, egui::Sense::click());

                // Start a new connection if clicking output
                if output_response.clicked() {
                    self.connecting_from = Some(node.id);
                }

                // Only allow dragging if output wasn't hovered
                let node_rect = egui::Rect::from_min_size(node.pos, egui::vec2(node_width, node_height));
                let rect_response = ui.allocate_rect(node_rect, egui::Sense::click_and_drag());

                if rect_response.drag_started() && !output_response.hovered() {
                    self.dragging_node = Some(node.id);
                }

                if rect_response.dragged() && self.dragging_node == Some(node.id) {
                    node.pos += rect_response.drag_delta();
                }

                if rect_response.drag_stopped() {
                    self.dragging_node = None;
                }

            }


            // Draw temporary line if dragging a new connection
            if let Some(start_id) = self.connecting_from {
                let start_node = self.nodes.iter().find(|n| n.id == start_id).unwrap();
                let node_width = 120.0;
                let node_height = 60.0;

                // Start from the output circle
                let start_pos = start_node.pos + egui::vec2(node_width, node_height / 2.0);

                ui.painter().line_segment(
                    [start_pos, self.mouse_pos],
                    Stroke::new(2.0, Color32::LIGHT_BLUE),
                );
            }

        });

        ctx.request_repaint(); // smooth dragging
    }
}


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Node Graph", options, Box::new(|_cc| Box::new(App::default())))
}
