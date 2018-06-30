use super::AtlasPage;
use super::Format;
use super::TextureFilter;
use super::TextureWrap;
use super::AtlasRegion;

pub struct AtlasLoader
{

}

impl AtlasLoader
{
    pub fn load(data: &String) -> Vec<AtlasPage>
    {
        data.split("\n\n")
            .map(|p| Self::initialize_page(p)).collect()
    }

    fn initialize_page(data: &str) -> AtlasPage
    {
        let page_data = data.to_string();
        let page_data = page_data.trim();
        //println!("_{}", page_data);

        // let end_of_line1 = page_data.find("\n").unwrap();
        // let (page_title, rest) = page_data.split_at(end_of_line1);
        let lines: Vec<_> = page_data.split("\n").collect();
        let page_title = lines[0];
        let size_text = lines[1];
        let format_text = lines[2];
        let filter_text = lines[3];
        let repeat_text = lines[4];
        let regions_start= page_title.len() + size_text.len() + format_text.len() + filter_text.len() + repeat_text.len() + 5;

        let (_, regions_text) = page_data.split_at(regions_start);
        // println!("_{}", regions_text);

        let regions = regions_text.lines().fold(Vec::default(), |accum, line| {
            if !line.starts_with(" ")
            {
                accum.into_iter().chain(vec![AtlasRegion {
                    name: line.to_string(),
                    rotate: false,
                    xy: (0, 0),
                    size: (0, 0),
                    split: (0, 0, 0, 0),
                    pad: (0, 0, 0, 0),
                    orig: (0, 0),
                    offset: (0, 0),
                    index: None
                }]).collect()
            }
            else
            {
                let (last, rest) = accum.split_last().unwrap();
                // let separator_index = line.find(": ").unwrap();
                // let (key, value) = line.split_at(separator_index+2);
                let parts: Vec<_> = line.split(": ").collect();
                let key = parts[0].to_string();
                let value = parts[1];
                println!("_{}_ {}", key, value);
                let updated_region = match key.trim().as_ref()
                {
                    "rotate" => {
                        AtlasRegion { rotate: match value {"false" => false, _ => true}, ..last.clone() }
                    },
                    "xy" => {
                        let coords: Vec<_> = value.split(",").collect();
                        let x = coords[0].trim().parse::<i32>().unwrap();
                        let y = coords[1].trim().parse::<i32>().unwrap();
                        AtlasRegion { xy: (x, y), ..last.clone()}
                    },
                    "size" => {
                        let coords: Vec<_> = value.split(",").collect();
                        let x = coords[0].trim().parse::<i32>().unwrap();
                        let y = coords[1].trim().parse::<i32>().unwrap();
                        AtlasRegion { size: (x, y), ..last.clone()}
                    },
                    "split" => {
                        let coords: Vec<_> = value.split(",").collect();
                        let w = coords[0].trim().parse::<i32>().unwrap();
                        let x = coords[1].trim().parse::<i32>().unwrap();
                        let y = coords[2].trim().parse::<i32>().unwrap();
                        let z = coords[3].trim().parse::<i32>().unwrap();
                        AtlasRegion { split: (w, x, y, z), ..last.clone()}
                    },
                    "pad" => {
                        let coords: Vec<_> = value.split(",").collect();
                        let w = coords[0].trim().parse::<i32>().unwrap();
                        let x = coords[1].trim().parse::<i32>().unwrap();
                        let y = coords[2].trim().parse::<i32>().unwrap();
                        let z = coords[3].trim().parse::<i32>().unwrap();
                        AtlasRegion { pad: (w, x, y, z), ..last.clone()}
                    },
                    "orig" => {
                        let coords: Vec<_> = value.split(",").collect();
                        let x = coords[0].trim().parse::<i32>().unwrap();
                        let y = coords[1].trim().parse::<i32>().unwrap();
                        AtlasRegion { orig: (x, y), ..last.clone()}
                    },
                    "offset" => {
                        let coords: Vec<_> = value.split(",").collect();
                        let x = coords[0].trim().parse::<i32>().unwrap();
                        let y = coords[1].trim().parse::<i32>().unwrap();
                        AtlasRegion { offset: (x, y), ..last.clone()}
                    },
                    "index" => {
                        AtlasRegion { index: value.trim().parse::<i32>().map(|x| Some(x)).unwrap_or(None), ..last.clone() }
                    },
                    _ => {println!("No joy {}", key); last.clone()}
                };
                rest.into_iter().map(|i| i.clone()).chain(vec![updated_region]).collect()
            }
        });

        AtlasPage {
            name: page_title.to_string(),
            size: (0, 0),
            format: Format::RGBA8888,
            filter: TextureFilter::Linear,
            repeat: TextureWrap::Repeat,
            regions
        }
    }
}

#[cfg(test)]
mod tests {

    use std::fs;
    use super::AtlasLoader;

    #[test]
    fn it_works() {
        let atlas_data = fs::read_to_string("test/fake-atlas.atlas").expect("Unable to open manifest");
        let atlas_pages = AtlasLoader::load(&atlas_data);

        assert_eq!(2, atlas_pages.len());
        let ref page1 = atlas_pages[0];
        let ref page2 = atlas_pages[1];

        assert_eq!("page1.png", page1.name);
        assert_eq!("page2.png", page2.name);

        assert_eq!(2, page1.regions.len());
        assert_eq!(1, page2.regions.len());

        let ref reg1 = page1.regions[0];
        let ref reg2 = page1.regions[1];
        let ref reg3 = page2.regions[0];

        assert_eq!(true, reg1.rotate);
        assert_eq!(false, reg2.rotate);
        assert_eq!(false, reg3.rotate);

        assert_eq!((372, 100), reg1.xy);
        assert_eq!((2, 21), reg2.xy);
        assert_eq!((519, 223), reg3.xy);

        assert_eq!((26, 108), reg1.size);
        assert_eq!((103, 81), reg2.size);
        assert_eq!((21, 42), reg3.size);

        assert_eq!((26, 108), reg1.orig);
        assert_eq!((103, 81), reg2.orig);
        assert_eq!((21, 42), reg3.orig);
    }
}
