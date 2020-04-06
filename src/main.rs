use magick_rust::{magick_wand_genesis, MagickWand};
use mikack::{error::*, extractors};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{create_dir_all, File},
    io::prelude::*,
    path::{Path, PathBuf},
    sync::Once,
};
use url::Url;

static START: Once = Once::new();
struct ImgInfo {
    name: String,
    extension: String,
    bytes: Vec<u8>,
}

impl ImgInfo {
    fn fname(&self) -> String {
        format!(
            "{name}.{extension}",
            name = self.name,
            extension = self.extension
        )
    }
}

fn main() -> Result<()> {
    for (domain, _name) in extractors::platforms().iter() {
        let extr = extractors::get_extr(domain).unwrap();
        let headers = headers_gen(domain, extr.is_https());
        if let Some(favicon) = extr.get_favicon() {
            if !Path::new(&format!("favicon/{}.ico", domain)).exists() {
                let img = &get_img(*favicon, headers)?;
                save(img)?;
                println!("`{}` has been saved.", img.fname());
            } else {
                println!("`{}.ico` already exists, skiped.", domain);
            }
        } else {
            println!("`{}` has no favicon, ignored", domain);
        }
    }

    Ok(())
}

fn headers_gen<S: Into<String>>(domain: S, is_https: bool) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    let schema = if is_https { "https://" } else { "http://" };
    headers.insert(
        String::from("Referer"),
        format!("{schema}{domain}", schema = schema, domain = domain.into()),
    );

    headers
}

fn get_img<S: Into<String>, S2: Into<String>>(url: S, headers: HashMap<S2, S2>) -> Result<ImgInfo> {
    let url = &url.into();
    let mut header_map = HeaderMap::new();
    for (header, value) in headers {
        header_map.insert(
            HeaderName::from_bytes(header.into().as_bytes())?,
            HeaderValue::from_str(&value.into())?,
        );
    }
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?
        .get(url)
        .headers(header_map);

    let mut resp = client.send()?;
    let mut bytes = vec![];
    resp.copy_to(&mut bytes)?;
    let extension = Path::new(url)
        .extension()
        .unwrap_or(OsStr::new("ico"))
        .to_str()
        .unwrap_or("ico")
        .to_string();
    let name = Url::parse(url)?.domain().unwrap().to_string();

    Ok(ImgInfo {
        name,
        extension,
        bytes,
    })
}

fn save(img: &ImgInfo) -> Result<()> {
    let mut fpath = PathBuf::from("dist");
    if !fpath.exists() {
        create_dir_all(&fpath)?;
    }
    fpath.push(img.fname());
    let mut f = File::create(&fpath)?;
    f.write_all(&img.bytes)?;

    unifoed_process(fpath.to_str().unwrap(), &img.name);

    Ok(())
}

fn unifoed_process(ipath: &str, name: &str) {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    wand.read_image(ipath).unwrap();
    wand.fit(16, 16);
    wand.write_image_blob("ico").unwrap();
    let mut fpath = PathBuf::from("favicon");
    create_dir_all("favicon").unwrap();
    fpath.push(format!("{name}.ico", name = name));
    wand.write_image(fpath.to_str().unwrap()).unwrap();
}
