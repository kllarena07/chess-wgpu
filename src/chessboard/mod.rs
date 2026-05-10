use std::sync::Arc;

mod vertex;
use vertex::Vertex;

use wgpu::util::DeviceExt;

use crate::piece::{Piece, SquareState};

pub struct Chessboard {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    board_state: [Option<Piece>; 64]
}

impl Chessboard {
    pub fn new(device: Arc<wgpu::Device>, config: Arc<wgpu::SurfaceConfiguration>) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("chessboard.wgsl").into())
        });

        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    Vertex::desc()
                ], // 2.
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        const WHITE: [f32; 3] = [0.925, 0.925, 0.800];
        const GREEN: [f32; 3] = [0.450, 0.667, 0.290];

        let mut vertices: Vec<Vertex> = vec![];

        for i in 0..64 {
            let col = i % 8;
            let row = i / 8;
            let color = if (row + col) % 2 == 0 { WHITE } else { GREEN };

            let x_offset = ((i % 8) as f32) * 0.25;
            let y_offset = ((i / 8) as f32) * 0.25;

            let mut new_vertices: Vec<Vertex> = vec![
                Vertex { position: [-1.0 + x_offset, 1.0 - y_offset, 0.0], color },
                Vertex { position: [-1.0 + x_offset, 0.75 - y_offset, 0.0], color },
                Vertex { position: [-0.75 + x_offset, 1.0 - y_offset, 0.0], color },
                Vertex { position: [-0.75 + x_offset, 0.75 - y_offset, 0.0], color },
            ];

            vertices.append(new_vertices.as_mut());
        }

        let num_vertices = vertices.len() as u32;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX
        });

        let board_state: [Option<Piece>; 64] = [
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackCastle, 0.0, 0.0)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackKnight, 0.25, 0.0)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackBishop, 0.5, 0.0)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackQueen, 0.75, 0.0)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackKing, 1.0, 0.0)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackBishop, 1.25, 0.0)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackKnight, 1.5, 0.0)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackCastle, 1.75, 0.0)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackPawn, 0.0, 0.25)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackPawn, 0.25, 0.25)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackPawn, 0.5, 0.25)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackPawn, 0.75, 0.25)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackPawn, 1.0, 0.25)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackPawn, 1.25, 0.25)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackPawn, 1.5, 0.25)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::BlackPawn, 1.75, 0.25)),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhitePawn, 0.0, 1.5)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhitePawn, 0.25, 1.5)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhitePawn, 0.5, 1.5)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhitePawn, 0.75, 1.5)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhitePawn, 1.0, 1.5)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhitePawn, 1.25, 1.5)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhitePawn, 1.5, 1.5)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhitePawn, 1.75, 1.5)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhiteCastle, 0.0, 1.75)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhiteKnight, 0.25, 1.75)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhiteBishop, 0.5, 1.75)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhiteQueen, 0.75, 1.75)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhiteKing, 1.0, 1.75)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhiteBishop, 1.25, 1.75)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhiteKnight, 1.5, 1.75)),
            Some(Piece::new(Arc::clone(&device), Arc::clone(&config), SquareState::WhiteCastle, 1.75, 1.75)),
        ];

        Self {
            render_pipeline,
            vertex_buffer,
            num_vertices,
            board_state
        }
    }
    pub fn get_board_state(&self) -> &[Option<Piece>] {
        &self.board_state
    }
    pub fn render_pipeline(&self) -> wgpu::RenderPipeline {
        self.render_pipeline.clone()
    }
    pub fn vertex_buffer(&self) -> wgpu::Buffer {
        self.vertex_buffer.clone()
    }
    pub fn num_vertices(&self) -> u32 {
        self.num_vertices
    }
    pub fn move_piece(&mut self, piece: usize, position: (f32, f32)) {
        println!("moving: {:?}", self.board_state[piece]);
        if let Some(piece) = self.board_state[piece].as_mut() {
            piece.move_piece(position);
            println!("Moved {:?} to {:?}", piece.piece_type, position);
        }
    }
}

