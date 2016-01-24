use glium::{Display, Program};
pub use glium::uniforms::{Uniforms, UniformValue, AsUniformValue};
use texture::TextureRef;
use mesh::{Vertex, VertexType};


pub trait Shader {
    type Vertex: VertexType;
    type Uniforms: Uniforms;

    fn program(&Display) -> Program;
}


pub struct Default;


impl Shader for Default {
    type Vertex = Vertex;
    type Uniforms = DefaultUniforms;

    fn program(display: &Display) -> Program {
        let vert = include_str!("shader/default.vert");
        let frag = include_str!("shader/default.frag");
        program!(display,
            140 => {
                vertex: vert,
                fragment: frag,
            },
        ).unwrap()
    }
}


#[macro_export]
macro_rules! uniforms_define {
    ($struct_name:ident {$($field:ident: $t:ty),*}) => {
        pub struct $struct_name {
            $(
                pub $field: $t,
            )*
        }

        impl $crate::shader::Uniforms for $struct_name {
            fn visit_values<'a, F>(&'a self, mut output: F)
                where F: FnMut(&str, $crate::shader::UniformValue<'a>)
            {
                use $crate::shader::AsUniformValue;
                $(
                    output(stringify!($field), self.$field.as_uniform_value());
                )*
            }
        }
    };
    ($struct_name:ident {$($field:ident: $t:ty),*,}) => {
        uniforms_define! { $struct_name {$($field: $t),*} }
    }
}


uniforms_define! {
    DefaultUniforms {
        tex: TextureRef,
        matrix: [[f32; 4]; 4],
    }
}

