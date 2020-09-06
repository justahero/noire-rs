
/// Defines a particular binding between Shader input variable and outside parameters.
///
/// Example:
/// const VERTEX_SHADER: &str = r#"
/// #version 450
///
/// uniform vec3 color;
/// layout(location=0) in vec2 position;
///
/// void main() {
///     gl_Position = vec4(position, 0.0, 1.0);
/// }
/// "#;
///
/// The binding defines input 'uniform' variable
///
#[derive(Debug)]
pub struct BindingDescriptor {

}

impl BindingDescriptor {
}
