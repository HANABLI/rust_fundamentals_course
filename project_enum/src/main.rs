enum DiskType {
    SSD,
    HDD,
}

#[derive(Debug)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}
#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
}

struct Wine {
    name: String,
    region: WineRegions, // WineRegions is used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported"),
        _ => println!("{:?} is not supported", w),
    }
}

enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        _ => FileSize::Gigabytes(size / 1_000_000_000),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1000.0),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1000.0),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1000.0),
    }
}

fn main() {
    let size = 400000;
    let fileSize = format_size(size);
    println!("{}", fileSize);
}

// fn main() {
//     let wine_CM = Wine {
//         name: String::from("Chateau Margaux"),
//         region: WineRegions::Bordeaux,
//     };
//     let wine_B = Wine {
//         name: String::from("Barolo"),
//         region: WineRegions::Tuscany,
//     };
//     println!("Wine_CM: {} from {:?}", wine_CM.name, wine_CM.region);
//     println!("Wine_B: {} from {:?}", wine_B.name, wine_B.region);
//     supported_regions(wine_CM.region);
//     supported_regions(WineRegions::Rioja);
// }

// fn main() {
//     let disk_type = DiskType::SSD;
//     // Can't compare them like this!
//     // if disk_type == DiskType::SSD {
//     //     println!("Disk type is SSD");
//     // } else {
//     //     println!("Disk type is HDD");
//     // }
//     match disk_type {
//         DiskType::SSD => println!("Disk type is SSD"),
//         DiskType::HDD => println!("Disk type is HDD"),
//     }
//     let disk_size = DiskSize::GB(128);
//     println!("Disk size: {:?}", disk_size);
// }
