use glium::{DrawError, GliumCreationError, ProgramCreationError, SwapBuffersError};
use glium::vertex::BufferCreationError as VertexCreationError;
use glium::index::BufferCreationError as IndexCreationError;
use glium::texture::TextureCreationError;
use glium::glutin::CreationError;
use std::convert::From;
use image::ImageError;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    description: String,
    error_type: ErrorEnum,
}

#[derive(Debug)]
pub enum ErrorEnum {
    NoContext,
    GliumCreationError(GliumCreationError<CreationError>),
    ProgramCreationError(ProgramCreationError),
    TextureCreationError(TextureCreationError),
    IndexCreationError(IndexCreationError),
    VertexCreationError(VertexCreationError),
    DrawError(DrawError),
    SwapBuffersError(SwapBuffersError),
    // NoWindow,
    // NoInnerPixelSize,
    // ComponentNotFound(u64),
    ImageError(ImageError),
}

// impl Error {
// pub fn no_window() -> Error {
//    Error {
//        description: "No window found".to_owned(),
//        error_type: ErrorEnum::NoWindow,
//    }
//
// pub fn no_inner_pixel_size() -> Error {
//    Error {
//        description: "No inner pixel size of WinRef found".to_owned(),
//        error_type: ErrorEnum::NoInnerPixelSize,
//    }
//
// pub fn could_not_find_component(id: u64) -> Error {
//    Error {
//        description: format!("Could not find component {}", id),
//        error_type: ErrorEnum::ComponentNotFound(id),
//    }
//
//

impl From<GliumCreationError<CreationError>> for Error {
    fn from(error: GliumCreationError<CreationError>) -> Self {
        Error {
            description: format!("Glium creation error: {:?}", error),
            error_type: ErrorEnum::GliumCreationError(error),
        }
    }
}

impl From<ProgramCreationError> for Error {
    fn from(error: ProgramCreationError) -> Self {
        Error {
            description: format!("Program creation error: {:?}", error),
            error_type: ErrorEnum::ProgramCreationError(error),
        }
    }
}

impl From<TextureCreationError> for Error {
    fn from(error: TextureCreationError) -> Self {
        Error {
            description: format!("Texture creation error: {:?}", error),
            error_type: ErrorEnum::TextureCreationError(error),
        }
    }
}

impl From<IndexCreationError> for Error {
    fn from(error: IndexCreationError) -> Self {
        Error {
            description: format!("Index creation error: {:?}", error),
            error_type: ErrorEnum::IndexCreationError(error),
        }
    }
}

impl From<VertexCreationError> for Error {
    fn from(error: VertexCreationError) -> Self {
        Error {
            description: format!("Vertex creation error: {:?}", error),
            error_type: ErrorEnum::VertexCreationError(error),
        }
    }
}

impl From<DrawError> for Error {
    fn from(error: DrawError) -> Self {
        Error {
            description: format!("Draw error: {:?}", error),
            error_type: ErrorEnum::DrawError(error),
        }
    }
}

impl From<SwapBuffersError> for Error {
    fn from(error: SwapBuffersError) -> Self {
        Error {
            description: format!("Swap buffers error: {:?}", error),
            error_type: ErrorEnum::SwapBuffersError(error),
        }
    }
}

impl From<ImageError> for Error {
    fn from(error: ImageError) -> Self {
        Error {
            description: format!("Image error: {:?}", error),
            error_type: ErrorEnum::ImageError(error),
        }
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error {
            description: format!("No context error"),
            error_type: ErrorEnum::NoContext
        }
    }
}
