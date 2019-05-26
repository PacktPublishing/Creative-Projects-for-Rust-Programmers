use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Default)]
struct Product {
    id: u32,
    category: String,
    name: String,
}

#[derive(Debug, Default)]
struct Sale {
    id: String,
    product_id: u32,
    date: i64,
    quantity: f64,
    unit: String,
}

enum LocationItem {
    Other,
    InProduct,
    InSale,
}
enum LocationProduct {
    Other,
    InId,
    InCategory,
    InName,
}
enum LocationSale {
    Other,
    InId,
    InProductId,
    InDate,
    InQuantity,
    InUnit,
}

fn main() {
    let mut location_item = LocationItem::Other;
    let mut location_product = LocationProduct::Other;
    let mut location_sale = LocationSale::Other;
    let pathname = std::env::args().nth(1).unwrap();
    let mut product: Product = Default::default();
    let mut sale: Sale = Default::default();
    let file = std::fs::File::open(pathname).unwrap();
    let file = std::io::BufReader::new(file);
    let parser = EventReader::new(file);
    for event in parser {
        match &location_item {
            LocationItem::Other => match event {
                Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "product" => {
                    location_item = LocationItem::InProduct;
                    location_product = LocationProduct::Other;
                    product = Default::default();
                }
                Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "sale" => {
                    location_item = LocationItem::InSale;
                    location_sale = LocationSale::Other;
                    sale = Default::default();
                }
                _ => {}
            },
            LocationItem::InProduct => match &location_product {
                LocationProduct::Other => match event {
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "id" => {
                        location_product = LocationProduct::InId;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "category" =>
                    {
                        location_product = LocationProduct::InCategory;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "name" => {
                        location_product = LocationProduct::InName;
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_item = LocationItem::Other;
                        println!("  Exit product: {:?}", product);
                    }
                    _ => {}
                },
                LocationProduct::InId => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        product.id = characters.parse::<u32>().unwrap();
                        println!("Got product.id: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_product = LocationProduct::Other;
                    }
                    _ => {}
                },
                LocationProduct::InCategory => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        product.category = characters.clone();
                        println!("Got product.category: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_product = LocationProduct::Other;
                    }
                    _ => {}
                },
                LocationProduct::InName => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        product.name = characters.clone();
                        println!("Got product.name: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_product = LocationProduct::Other;
                    }
                    _ => {}
                },
            },
            LocationItem::InSale => match &location_sale {
                LocationSale::Other => match event {
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "id" => {
                        location_sale = LocationSale::InId;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "product-id" =>
                    {
                        location_sale = LocationSale::InProductId;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "date" => {
                        location_sale = LocationSale::InDate;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "quantity" =>
                    {
                        location_sale = LocationSale::InQuantity;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "unit" => {
                        location_sale = LocationSale::InUnit;
                    }
                    Ok(XmlEvent::EndElement { ref name, .. }) if name.local_name == "sale" => {
                        location_item = LocationItem::Other;
                        println!("  Exit sale: {:?}", sale);
                    }
                    _ => {}
                },
                LocationSale::InId => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        sale.id = characters.clone();
                        println!("Got sale.id: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    }
                    _ => {}
                },
                LocationSale::InProductId => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        sale.product_id = characters.parse::<u32>().unwrap();
                        println!("Got sale.product-id: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    }
                    _ => {}
                },
                LocationSale::InDate => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        sale.date = characters.parse::<i64>().unwrap();
                        println!("Got sale.date: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    }
                    _ => {}
                },
                LocationSale::InQuantity => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        sale.quantity = characters.parse::<f64>().unwrap();
                        println!("Got sale.quantity: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    }
                    _ => {}
                },
                LocationSale::InUnit => match event {
                    Ok(XmlEvent::Characters(characters)) => {
                        sale.unit = characters.clone();
                        println!("Got sale.unit: {}.", characters);
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    }
                    _ => {}
                },
            },
        }
    }
}
