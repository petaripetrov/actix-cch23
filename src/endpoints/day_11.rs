use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, HttpResponse};
use image::{io::Reader as ImageRader, Rgb};

use crate::common::{EndpointRet, ServerError};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    image: TempFile,
}

#[post("11/red_pixels")]
async fn red_pixels(MultipartForm(form): MultipartForm<UploadForm>) -> EndpointRet {
    // A beautiful nest of match statements and error handling
    // Cant use the shorthand '?' because that can't be mapped to ServerError
    // TODO figure out why '?' can't map to ServerError
    // TODO impl the necessary trait to cast ? to ServerErrro
    let img = match ImageRader::open(form.image.file) {
        Ok(reader) => match reader.with_guessed_format() {
            Ok(file) => match file.decode() {
                Ok(image) => image,
                Err(_) => return Err(ServerError::InternalError),
            },
            Err(_) => return Err(ServerError::InternalError),
        },
        Err(_) => return Err(ServerError::InternalError),
    };

    // Have to take the image as rgb8, so we also have to cast
    // all of the numbers to usize to get around overflows
    let pixels = match img.as_rgb8() {
        Some(img) => img.pixels(),
        None => return Err(ServerError::InternalError),
    };

    let count = pixels
        .filter(|Rgb([red, green, blue])| {
            let r = *red as usize;
            let g = *green as usize;
            let b = *blue as usize;

            return r > g + b;
        })
        .count();

    Ok(HttpResponse::Ok().body(count.to_string()))
}
