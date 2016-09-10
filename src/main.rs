#[macro_use]
extern crate glium;

use std::f32;      //pi
use std::ops::Mul; // multiplication overload

// ============================================================
// Vertex
// ============================================================
#[derive(Copy, Clone)]
struct Vertex {
	position : [f32;4],
}

impl Vertex {
	fn new(in_vertex : [f32; 3]) -> Vertex {
		Vertex {
			position: [in_vertex[0],in_vertex[1],in_vertex[2],1.0]
		}
	}
}

// ============================================================
// Matrix
// ============================================================
// NB: OpenGL (maybe) treats vectors as row vectors, so matrices should be transposed and multiplication reversed?
/// A 4x4 matrix for holding transformations.
#[derive(Copy, Clone)]
struct Matrix {
	_contents : [[f32; 4]; 4]
}

impl Matrix {
	fn new(in_contents : [[f32; 4]; 4]) -> Matrix {
		Matrix {
			_contents: in_contents
		}
	}
	
	fn contents(&self) -> &[[f32;4];4] {&self._contents}
}

// Matrix multiplication. TODO: use a linear algebra library.
impl Mul for Matrix {
	type Output = Matrix;
	
	fn mul (self, in_other : Matrix) -> Matrix {
		let a : &[[f32;4];4] = &self._contents;
		let b : &[[f32;4];4] = &in_other._contents;
		Matrix::new([[
			a[0][0]*b[0][0]+a[0][1]*b[1][0]+a[0][2]*b[2][0]+a[0][3]*b[3][0],
			a[0][0]*b[0][1]+a[0][1]*b[1][1]+a[0][2]*b[2][1]+a[0][3]*b[3][1],
			a[0][0]*b[0][2]+a[0][1]*b[1][2]+a[0][2]*b[2][2]+a[0][3]*b[3][2],
			a[0][0]*b[0][3]+a[0][1]*b[1][3]+a[0][2]*b[2][3]+a[0][3]*b[3][3]
		], [
			a[1][0]*b[0][0]+a[1][1]*b[1][0]+a[1][2]*b[2][0]+a[1][3]*b[3][0],
			a[1][0]*b[0][1]+a[1][1]*b[1][1]+a[1][2]*b[2][1]+a[1][3]*b[3][1],
			a[1][0]*b[0][2]+a[1][1]*b[1][2]+a[1][2]*b[2][2]+a[1][3]*b[3][2],
			a[1][0]*b[0][3]+a[1][1]*b[1][3]+a[1][2]*b[2][3]+a[1][3]*b[3][3]
		], [
			a[2][0]*b[0][0]+a[2][1]*b[1][0]+a[2][2]*b[2][0]+a[2][3]*b[3][0],
			a[2][0]*b[0][1]+a[2][1]*b[1][1]+a[2][2]*b[2][1]+a[2][3]*b[3][1],
			a[2][0]*b[0][2]+a[2][1]*b[1][2]+a[2][2]*b[2][2]+a[2][3]*b[3][2],
			a[2][0]*b[0][3]+a[2][1]*b[1][3]+a[2][2]*b[2][3]+a[2][3]*b[3][3]
		], [
			a[3][0]*b[0][0]+a[3][1]*b[1][0]+a[3][2]*b[2][0]+a[3][3]*b[3][0],
			a[3][0]*b[0][1]+a[3][1]*b[1][1]+a[3][2]*b[2][1]+a[3][3]*b[3][1],
			a[3][0]*b[0][2]+a[3][1]*b[1][2]+a[3][2]*b[2][2]+a[3][3]*b[3][2],
			a[3][0]*b[0][3]+a[3][1]*b[1][3]+a[3][2]*b[2][3]+a[3][3]*b[3][3]
		]])
	}
}

// ============================================================
// Mesh
// ============================================================
/// The mesh of a single object (a triangle, a sphere, a goove...)
struct Mesh {
	/// The vertices of the triangles out of which the mesh is made
	_vertices      : Vec<Vertex>,
	/// The order in which the vertices should be drawn.
	_index_type    : glium::index::PrimitiveType,
	_indices       : Vec<u16>,
	_vertex_buffer : glium::VertexBuffer<Vertex>,
	_index_buffer  : glium::index::IndexBuffer<u16>,
}

impl Mesh {
	fn new (
		in_display    : &glium::backend::glutin_backend::GlutinFacade,
		in_vertices   : &Vec<Vertex>,
		in_index_type : &glium::index::PrimitiveType,
		in_indices    : &Vec<u16>,
	) -> Mesh {
		Mesh {
			_vertices      : in_vertices.to_owned(),
			_index_type    : in_index_type.to_owned(),
			_indices       : in_indices.to_owned(),
			_vertex_buffer : glium::VertexBuffer::new(in_display, in_vertices).unwrap(),
			_index_buffer  : glium::index::IndexBuffer::new (
				in_display,
				*in_index_type,
				in_indices,
			).unwrap(),
		}
	}
	
	fn vertex_buffer(&self) -> &glium::VertexBuffer<Vertex> {&self._vertex_buffer}
	fn index_buffer(&self) -> &glium::index::IndexBuffer<u16> {&self._index_buffer}
}


// ============================================================
// Atom
// ============================================================
/// The atom, the fundamental unit of a molecular viewer.
struct Atom<'a> {
	_mesh        : &'a Mesh,
	_position    : [f32;3],
	_size        : f32,
	_colour      : [f32;3],
	_body_matrix : Matrix,
}

impl<'a> Atom<'a> {
	fn new (
		in_mesh     : &'a Mesh,
		in_position : &[f32;3],
		in_size     : &f32,
		in_colour   : &[f32;3],
	) -> Atom<'a> {
		Atom {
			_mesh        : in_mesh,
			_position    : in_position.to_owned(),
			_size        : in_size.to_owned(),
			_colour      : in_colour.to_owned(),
			_body_matrix : Matrix::new([
				[*in_size, 0.0     , 0.0     , in_position[0]],
				[0.0     , *in_size, 0.0     , in_position[1]],
				[0.0     , 0.0     , *in_size, in_position[2]],
				[0.0     , 0.0     , 0.0     , 1.0           ]
			]),
		}
	}
	
	fn mesh(&self) -> &Mesh {&self._mesh}
	fn colour(&self) -> &[f32;3] {&self._colour}
	fn body_matrix(&self) -> &Matrix {&self._body_matrix}
}

// ============================================================
// Molecule
// ============================================================
// Will likely be the top level struct, unless we need something which has an OpenGL thing + this
/// The molecule. May also be a cluster, crystal motif,...
struct Molecule<'a> {
	_atoms : Vec<Atom<'a>>,
}

impl<'a> Molecule<'a> {
	fn new() -> Molecule<'a> {Molecule{_atoms : Vec::new()}}
	
	fn add_atom(
		&mut self,
		in_mesh     : &'a Mesh,
		in_position : &[f32;3],
		in_size     : &f32,
		in_colour   : &[f32;3],
	) {self._atoms.push(Atom::new(in_mesh, in_position, in_size, in_colour))}
	
	fn atoms(&self) -> &Vec<Atom> {&self._atoms}
}


// ============================================================
// Camera
// ============================================================
struct Camera {
	_position           : [f32;3],
	_focus              : [f32;3],
	_field_of_view      : f32,
	_near_plane         : f32,
	_far_plane          : f32,
	_camera_matrix      : Matrix,
	_perspective_matrix : Matrix,
	_view_matrix        : Matrix,
}

impl Camera {
	fn new (
		in_display       : &glium::backend::glutin_backend::GlutinFacade,
		in_position      : &[f32;3],
		in_focus         : &[f32;3],
		in_field_of_view : &f32,
		in_near_plane    : &f32,
		in_far_plane     : &f32
	) -> Camera {
		
		let (w, h) = (*in_display).get_framebuffer_dimensions();
		let mut w = w as f32;
		let mut h = h as f32;
		if w > h {
			w = w/h;
			h = 1.0;
		} else {
			w = 1.0;
			h = h/w;
		}
		
		let s = 1.0/(in_field_of_view*f32::consts::PI/360.0).tan();
		let n = in_near_plane.to_owned();
		let f = in_far_plane.to_owned();
		let perspective_matrix = Matrix::new([
			[s/w, 0.0, 0.0    , 0.0      ],
			[0.0, s/h, 0.0    , 0.0      ],
			[0.0, 0.0, (f+n)/(f-n), 2.0*f*n/(n-f)],
			[0.0, 0.0, 1.0   , 0.0      ]
		]);
		
		let mut camera = Camera {
			_position           : in_position.to_owned(),
			_focus              : in_focus.to_owned(),
			_field_of_view      : in_field_of_view.to_owned(),
			_near_plane         : in_near_plane.to_owned(),
			_far_plane          : in_far_plane.to_owned(),
			_camera_matrix      : Matrix::new([[0.0;4];4]),
			_perspective_matrix : perspective_matrix,
			_view_matrix        : Matrix::new([[0.0;4];4]),
		};
		camera.update();
		camera
	}
	
	fn view_matrix(&self) -> &Matrix {&self._view_matrix}
	
	fn set_position(&mut self, in_position : [f32;3]) {self._position = in_position; self.update();}
	
	fn update(&mut self) {
		let x = self._focus[0]-self._position[0];
		let y = self._focus[1]-self._position[1];
		let z = self._focus[2]-self._position[2];
		
		// theta is the orbital angle
		let cos_theta =  z/(x*x+z*z).sqrt();
		let sin_theta =  x/(x*x+z*z).sqrt();
		let orbital_matrix = Matrix::new([
			[ cos_theta, 0.0,-sin_theta, 0.0],
			[ 0.0      , 1.0, 0.0      , 0.0],
			[ sin_theta, 0.0, cos_theta, 0.0],
			[ 0.0      , 0.0, 0.0      , 1.0]
		]);
		
		// phi is the azimuthal angle
		let cos_phi = (x*x+z*z).sqrt()/(x*x+y*y+z*z).sqrt();
		let sin_phi = y/(x*x+y*y+z*z).sqrt();
		let azimuthal_matrix = Matrix::new([
			[1.0,  0.0    ,  0.0    , 0.0],
			[0.0,  cos_phi, -sin_phi, 0.0],
			[0.0,  sin_phi,  cos_phi, 0.0],
			[0.0,  0.0    ,  0.0    , 1.0]
		]);
		
		let translation_matrix = Matrix::new([
			[1.0, 0.0, 0.0, -self._position[0]],
			[0.0, 1.0, 0.0, -self._position[1]],
			[0.0, 0.0, 1.0, -self._position[2]],
			[0.0, 0.0, 0.0,  1.0              ]
		]);
		
		self._camera_matrix = azimuthal_matrix*orbital_matrix*translation_matrix;
		self._view_matrix = self._perspective_matrix*self._camera_matrix;
	}
}

// ============================================================
// Main Program
// ============================================================
/// Furnace - draw a molecule!
fn main() {
	// ==============================
	// Make display
	// ==============================
	use glium::{DisplayBuild, Surface};
	let display : glium::backend::glutin_backend::GlutinFacade = glium::glutin::WindowBuilder::new()
		.with_title("Furnace: Molecular Visualisation".to_string())
		.build_glium().unwrap();
	
	implement_vertex!(Vertex, position);
	
	// ==============================
	// Dark2
	// ==============================
	
	let turquoise = [ 27.0/255.0,158.0/255.0,119.0/255.0];
	let orange    = [217.0/255.0, 95.0/255.0,  2.0/255.0];
	let blue      = [117.0/255.0,112.0/255.0,179.0/255.0];
	let pink      = [231.0/255.0, 41.0/255.0,138.0/255.0];
	let green     = [102.0/255.0,166.0/255.0, 30.0/255.0];
	let yellow    = [230.0/255.0,171.0/255.0,  2.0/255.0];
	let brown     = [166.0/255.0,118.0/255.0, 29.0/255.0];
	let grey      = [102.0/255.0,102.0/255.0,102.0/255.0];
	
	// ==============================
	// Make meshes
	// ==============================
	// The positions of each vertex of the triangle
	let triangle_vertex0 = Vertex::new([-1.0, -1.0, 0.0]);
	let triangle_vertex1 = Vertex::new([-1.0,  1.0, 0.0]);
	let triangle_vertex2 = Vertex::new([ 1.0,  0.0, 0.0]);
	let triangle = Mesh::new(
		&display,
		&vec![triangle_vertex0, triangle_vertex1, triangle_vertex2],
		&glium::index::PrimitiveType::TriangleStrip,
		&vec![0, 1, 2u16]
	);

	// The positions of each vertex of the square
	let square_vertex0 = Vertex::new([-1.0, -1.0, 0.0]);
	let square_vertex1 = Vertex::new([ 1.0, -1.0, 0.0]);
	let square_vertex2 = Vertex::new([-1.0,  1.0, 0.0]);
	let square_vertex3 = Vertex::new([ 1.0,  1.0, 0.0]);
	let square = Mesh::new(
		&display,
		&vec![square_vertex0, square_vertex1, square_vertex2, square_vertex3],
		&glium::index::PrimitiveType::TriangleStrip,
		&vec![0, 1, 2, 3u16]
	);
	
	let tetrahedron = Mesh::new(
		&display,
		&vec![
			Vertex::new([-1.0,  0.0, -0.7]),
			Vertex::new([ 1.0,  0.0, -0.7]),
			Vertex::new([ 0.0, -1.0,  0.7]),
			Vertex::new([ 0.0,  1.0,  0.7]),
		],
		&glium::index::PrimitiveType::TriangleStrip,
		&vec![0, 1, 3, 2, 0, 1u16]
	);
	
	// A cube (will likely get weird rounded edges because of normal interpolation.
	// Different vertices should be used for different faces at each corner. (not needed since atoms are spheres.)
	// n.b. uses TrianglesList not TriangleStrip, because triangle strips don't do corners.
	let cube = Mesh::new(
		&display,
		&vec![
			Vertex::new([-1.0, -1.0, -1.0]),
			Vertex::new([ 1.0, -1.0, -1.0]),
			Vertex::new([-1.0,  1.0, -1.0]),
			Vertex::new([ 1.0,  1.0, -1.0]),
			Vertex::new([-1.0, -1.0,  1.0]),
			Vertex::new([ 1.0, -1.0,  1.0]),
			Vertex::new([-1.0,  1.0,  1.0]),
			Vertex::new([ 1.0,  1.0,  1.0])
		],
		&glium::index::PrimitiveType::TrianglesList,
		&vec![
			0, 2, 1, 3, 1, 2,   // the -z face
			2, 6, 3, 7, 3, 6,   // the  y face
			4, 5, 6, 7, 6, 5,   // the  z face
			0, 1, 4, 5, 4, 1,   // the -y face
			1, 3, 5, 7, 5, 3,   // the  x face
			0, 4, 2, 6, 2, 4u16 // the -x face
		]
	);
	
	// An icosahedron
	let phi = 2.0/(1.0+5.0f32.sqrt());
	let icosahedron = Mesh::new(
		&display,
		&vec![
			Vertex::new([ 0.0,  1.0,  phi]),
			Vertex::new([ 0.0, -1.0,  phi]),
			Vertex::new([ 0.0,  1.0, -phi]),
			Vertex::new([ 0.0, -1.0, -phi]),
			Vertex::new([ phi,  0.0,  1.0]),
			Vertex::new([ phi,  0.0, -1.0]),
			Vertex::new([-phi,  0.0,  1.0]),
			Vertex::new([-phi,  0.0, -1.0]),
			Vertex::new([ 1.0,  phi,  0.0]),
			Vertex::new([-1.0,  phi,  0.0]),
			Vertex::new([ 1.0, -phi,  0.0]),
			Vertex::new([-1.0, -phi,  0.0]),
		],
		&glium::index::PrimitiveType::TrianglesList,
		&vec![
			0, 8, 2,
			0, 2, 9,
			1, 3, 10,
			1, 11, 3,
			4, 0, 6,
			4, 6, 1,
			5, 7, 2,
			5, 3, 7,
			8, 4, 10,
			8, 10, 5,
			9, 11, 6,
			9, 7, 11,
			0, 4, 8,
			0, 9, 6,
			1, 10, 4,
			1, 6, 11,
			2, 8, 5,
			2, 7, 9,
			3, 5, 10,
			3, 11, 7u16
		]
	);
	
	// ==============================
	// Make molecule
	// ==============================
	let mut molecule = Molecule::new();
	molecule.add_atom(&cube, &[ 0.0,  0.0, 0.0], &0.2, &orange);
	molecule.add_atom(&tetrahedron, &[ 0.5,  0.5, 0.0], &0.2, &green);
	molecule.add_atom(&triangle, &[ 0.5, -0.5, 0.0], &0.2, &blue);
	molecule.add_atom(&triangle, &[-0.5,  0.5, 0.0], &0.2, &blue);
	molecule.add_atom(&tetrahedron, &[-0.5, -0.5, 0.0], &0.2, &green);
	molecule.add_atom(&square, &[ 0.5,  0.0, -0.5], &0.2, &turquoise);
	molecule.add_atom(&square, &[-0.5,  0.0, -0.5], &0.2, &turquoise);
	molecule.add_atom(&icosahedron, &[ 0.0,  0.5, 0.5], &0.2, &pink);
	molecule.add_atom(&square, &[ 0.0, -0.5, 0.5], &0.2, &turquoise);
	
	// ==============================
	// Make camera
	// ==============================
	// camera position
	let camera_position = [0.0,0.0,2.0];
	// camera focus (the point the camera is pointing at)
	let camera_focus = [0.0,0.0,0.0];
	// field of view, in degrees
	let field_of_view = 90.0;
	// near and far clipping planes
	let near_plane = 1.0;
	let far_plane = 10.0;
	
	let mut camera = Camera::new(&display, &camera_position, &camera_focus, &field_of_view, &near_plane, &far_plane);
	
	// ==============================
	// Make shaders
	// ==============================
	// Vertex shader in OpenGL v140 (written in GLSL) 
	let vertex_shader_src = r#"
	#version 140
	
	uniform mat4 matrix;
	uniform vec3 colour;
	
	in vec4 position;
	
	out vec3 fragmentColor;

	void main() {
		gl_Position = position*matrix;
		fragmentColor = colour;
	}
	"#;

	// Fragment/Pixel shader in OpenGL v140 (written in GLSL) 
	let fragment_shader_src = r#"
		#version 140
		
		in vec3 fragmentColor;
		
		out vec4 color;

		void main() {
			color = vec4((fragmentColor), 1.0);
		}
	"#;

	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
	
	// ==============================
	// Run everything
	// ==============================
	let mut i = 0;
	let spin_rate = 0.001;
	
	// this probably wants to be somewhere in the loop.
	let params = glium::DrawParameters {
		depth: glium::Depth {
			test: glium::DepthTest::IfLess,
			write: true,
			.. Default::default()
		},
		backface_culling : glium::BackfaceCullingMode::CullCounterClockwise,
		.. Default::default()
	};
	
	loop {
		let angle = (i as f32)*spin_rate;
		camera.set_position([2.0*angle.cos(),0.0,2.0*angle.sin()]);
		
		let mut target = display.draw();
		target.clear_color_and_depth((0.93, 0.91, 0.835, 1.0), 1.0);
		for atom in molecule.atoms() {
			let matrix = *camera.view_matrix() * *atom.body_matrix();
			let uniforms = uniform!{matrix: matrix.contents().to_owned(), colour: atom.colour().to_owned()};
			target.draw(
				atom.mesh().vertex_buffer(),
				atom.mesh().index_buffer(),
				&program,
				&uniforms,
				&params,
				//&Default::default(), // This should be params, but that's not working.
			).unwrap();
		}
		target.finish().unwrap();

		for ev in display.poll_events() {
			match ev {
				glium::glutin::Event::Closed => return,
				_ => ()
			}
		}
		i+=1;
	}
}
